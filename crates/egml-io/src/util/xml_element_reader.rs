use crate::Error;
use crate::util::xml_element::XmlElement;
use quick_xml::Reader;
use quick_xml::events::Event;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct XmlElementSpans<Elem> {
    spans: HashMap<Elem, Vec<Range<usize>>>,
}

impl<Elem: XmlElement> XmlElementSpans<Elem> {
    pub fn new(spans: HashMap<Elem, Vec<Range<usize>>>) -> Self {
        Self { spans }
    }

    /// Constructs spans that make `element` appear as a single direct child covering `[0..len]`.
    /// Used to bridge element-level bytes into parent-level dispatchers.
    pub fn single(element: Elem, len: usize) -> Self {
        let mut spans = HashMap::new();
        spans.insert(element, vec![0..len]);
        Self { spans }
    }

    pub fn spans(&self) -> &HashMap<Elem, Vec<Range<usize>>> {
        &self.spans
    }

    pub fn get(&self, element: Elem) -> &[Range<usize>] {
        self.spans
            .get(&element)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn first(&self, element: Elem) -> Option<&Range<usize>> {
        let spans = self.spans.get(&element)?;
        debug_assert!(
            spans.len() <= 1,
            "expected at most one span for {:?}",
            element
        );
        spans.first()
    }
}

pub fn collect_child<Elem, T, E>(
    xml_document: &[u8],
    spans: &XmlElementSpans<Elem>,
    element: Elem,
    deserializer: fn(&[u8], &XmlElementSpans<Elem>) -> Result<T, E>,
) -> Result<Option<T>, E>
where
    Elem: XmlElement,
    E: From<Error>,
{
    let all_spans = spans.get(element);
    if all_spans.len() >= 2 {
        debug!(
            "expected at most one {:?}, found {}",
            element,
            all_spans.len()
        );
    }
    match all_spans.first() {
        None => Ok(None),
        Some(x) => {
            let slice = &xml_document[x.start..x.end];
            let child_spans = extract_xml_element_spans(slice)?;
            deserializer(slice, &child_spans).map(Some)
        }
    }
}

/// Deserializes every span of `element`, pairing each result with the byte
/// range it came from. Shared by [`collect_children`] (fail-fast) and
/// [`collect_children_lenient`] (skip-and-continue).
fn collect_children_raw<Elem, T, E>(
    xml_document: &[u8],
    spans: &XmlElementSpans<Elem>,
    element: Elem,
    deserializer: fn(&[u8], &XmlElementSpans<Elem>) -> Result<T, E>,
) -> Vec<(Range<usize>, Result<T, E>)>
where
    Elem: XmlElement + Send + Sync,
    T: Send,
    E: From<Error> + Send,
{
    spans
        .get(element)
        .into_par_iter()
        .map(|x| {
            let result = extract_xml_element_spans(&xml_document[x.start..x.end])
                .map_err(E::from)
                .and_then(|child_spans| deserializer(&xml_document[x.start..x.end], &child_spans));
            (x.clone(), result)
        })
        .collect()
}

pub fn collect_children<Elem, T, E>(
    xml_document: &[u8],
    spans: &XmlElementSpans<Elem>,
    element: Elem,
    deserializer: fn(&[u8], &XmlElementSpans<Elem>) -> Result<T, E>,
) -> Result<Vec<T>, E>
where
    Elem: XmlElement + Send + Sync,
    T: Send,
    E: From<Error> + Send,
{
    collect_children_raw(xml_document, spans, element, deserializer)
        .into_iter()
        .map(|(_, result)| result)
        .collect()
}

/// A child element that failed to deserialize and was dropped by
/// [`collect_children_lenient`], along with the byte range it occupied in
/// the original document (useful for correlating with source XML).
#[derive(Debug)]
pub struct SkippedChild<Elem, E> {
    pub element: Elem,
    pub span: Range<usize>,
    pub error: E,
}

/// Like [`collect_children`], but never fails outright: children that fail to
/// deserialize are dropped (and logged at debug level) instead of aborting
/// the whole collection. Use this where individual members are allowed to be
/// invalid without invalidating the rest of the document (e.g. `surfaceMember`),
/// not for required singleton children.
pub fn collect_children_lenient<Elem, T, E>(
    xml_document: &[u8],
    spans: &XmlElementSpans<Elem>,
    element: Elem,
    deserializer: fn(&[u8], &XmlElementSpans<Elem>) -> Result<T, E>,
) -> (Vec<T>, Vec<SkippedChild<Elem, E>>)
where
    Elem: XmlElement + Send + Sync,
    T: Send,
    E: From<Error> + Send + Debug,
{
    let mut values = Vec::new();
    let mut skipped = Vec::new();

    for (span, result) in collect_children_raw(xml_document, spans, element, deserializer) {
        match result {
            Ok(value) => values.push(value),
            Err(error) => {
                debug!(
                    ?element,
                    start = span.start,
                    end = span.end,
                    ?error,
                    "skipping invalid child element"
                );
                skipped.push(SkippedChild {
                    element,
                    span,
                    error,
                });
            }
        }
    }

    (values, skipped)
}

/// Like [`collect_children`], but for deserializers that manage their own span
/// extraction and only need the raw element bytes.
pub fn collect_children_simple<Elem, T, E>(
    xml_document: &[u8],
    spans: &XmlElementSpans<Elem>,
    element: Elem,
    deserializer: fn(&[u8]) -> Result<T, E>,
) -> Result<Vec<T>, E>
where
    Elem: XmlElement + Send + Sync,
    T: Send,
    E: Send,
{
    spans
        .get(element)
        .into_par_iter()
        .map(|x| deserializer(&xml_document[x.start..x.end]))
        .collect()
}

pub fn extract_xml_element_spans<Elem: XmlElement>(
    xml_document: &[u8],
) -> Result<XmlElementSpans<Elem>, Error> {
    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);

    let mut depth = 0;
    let mut element_spans: HashMap<Elem, Vec<Range<usize>>> = HashMap::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                // Skip grandchildren and deeper — only direct children of the root are recorded.
                if depth >= 2 {
                    reader.read_to_end(e.name())?;
                    continue;
                }
                depth += 1;
                // depth == 1 means this is the root element (the document being scanned).
                // Skip recording it; traversal continues into its children.
                if depth == 1 {
                    continue;
                }

                if let Some(x) = Elem::from_local_name(e.local_name().as_ref()) {
                    // buffer_position() is right after `>` of the start tag,
                    // so the `<` of the start tag is e.len() + 2 bytes back.
                    let pos_start = reader.buffer_position() as usize - e.len() - 2;
                    reader.read_to_end(e.name())?;
                    // buffer_position() is now right after `>` of the closing tag.
                    let pos_end = reader.buffer_position() as usize;

                    element_spans.entry(x).or_default().push(pos_start..pos_end);
                    depth -= 1;
                }
            }
            Ok(Event::Empty(e)) if depth == 1 => {
                // Self-closing elements (<foo/>) only appear as direct children (depth == 1).
                if let Some(x) = Elem::from_local_name(e.local_name().as_ref()) {
                    // buffer_position() is right after `>` of `<foo/>`,
                    // so `<` is e.len() + 3 bytes back (for `<`, `/`, `>`).
                    let pos_start = reader.buffer_position() as usize - e.len() - 3;
                    let pos_end = reader.buffer_position() as usize;
                    element_spans.entry(x).or_default().push(pos_start..pos_end);
                }
            }
            Ok(Event::End(_)) => {
                depth -= 1;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Error::from(e)),
            _ => {}
        }
    }

    Ok(XmlElementSpans::new(element_spans))
}
