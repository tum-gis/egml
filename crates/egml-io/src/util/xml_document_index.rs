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

/// One matched element within a document: its own byte range (covering the
/// full `<Tag>...</Tag>` or `<Tag/>`), plus the spans of *its* direct
/// children — computed once, during the single top-to-bottom walk that
/// found this element, so descending into it (via [`get`](Self::get) /
/// [`first`](Self::first)) never re-parses.
///
/// Building the whole tree in one pass (rather than re-scanning a slice
/// afresh every time a caller descends into it) is sound because
/// [`XmlElement::from_local_name`] is context-free: a given local name
/// always maps to the same `Elem` variant no matter how deep it appears, so
/// every level's matches can be recorded during a single walk.
#[derive(Debug, Clone)]
pub struct XmlDocumentIndex<Elem> {
    range: Range<usize>,
    children: HashMap<Elem, Vec<Self>>,
    /// `true` when the scan that produced this node hit its `max_depth`
    /// limit right here, so `children` is empty because it was never
    /// inspected — not because the underlying XML actually has none. A
    /// caller that needs to know what this node itself contains should
    /// re-scan `xml_document[self.range()]` (e.g. via
    /// [`from_scan`](Self::from_scan) with a larger or `None` `max_depth`)
    /// rather than trust an empty [`get`](Self::get)/[`first`](Self::first).
    truncated: bool,
}

impl<Elem: XmlElement> XmlDocumentIndex<Elem> {
    /// The byte range this node itself covers in the document it was
    /// extracted from.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    /// The map of this node's direct children, keyed by element type. Use
    /// [`get`](Self::get)/[`first`](Self::first) for typed lookups; this is
    /// for callers that need to iterate every child regardless of type
    /// (e.g. to preserve original document order across mixed types).
    pub fn children(&self) -> &HashMap<Elem, Vec<Self>> {
        &self.children
    }

    pub fn get(&self, element: Elem) -> &[Self] {
        self.children
            .get(&element)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn first(&self, element: Elem) -> Option<&Self> {
        let matches = self.get(element);
        debug_assert!(
            matches.len() <= 1,
            "expected at most one span for {:?}",
            element
        );
        matches.first()
    }

    /// `true` if this node's `children` was never inspected because the
    /// scan that produced it hit its `max_depth` limit here — meaning an
    /// empty [`get`](Self::get)/[`first`](Self::first) result on this node
    /// isn't proof the element is absent, only that it wasn't looked for.
    /// Re-scan `xml_document[self.range()]` (via
    /// [`from_scan`](Self::from_scan) with a larger or `None` `max_depth`)
    /// to get a trustworthy answer.
    pub fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Records every matched element in one pass, each carrying its own
    /// already-extracted children — so descending into any of them via
    /// [`get`](Self::get)/[`first`](Self::first) never re-parses.
    ///
    /// `max_depth` bounds how many levels below `xml_document`'s root are
    /// inspected for matches: `Some(1)` records only direct children
    /// (grandchildren and deeper are walked, to find each direct child's
    /// end, but never inspected — descending into one later needs its own
    /// scan), while `None` records matches at every depth, however deep,
    /// at the cost of scanning the whole document up front. Pass `Some(1)`
    /// when the caller only needs one level and may never look further;
    /// pass `None` to avoid the O(depth) re-scanning that repeated shallow
    /// scans at each level would cost.
    pub fn from_scan(xml_document: &[u8], max_depth: Option<usize>) -> Result<Self, Error> {
        let mut reader = Reader::from_reader(xml_document);
        reader.config_mut().trim_text(true);

        // Consume the document's own root Start event without recording it —
        // it isn't itself a "child" of anything — then record everything
        // nested inside it.
        loop {
            match reader.read_event()? {
                Event::Start(_) => break,
                Event::Empty(_) | Event::Eof => {
                    return Ok(Self {
                        range: 0..xml_document.len(),
                        children: HashMap::new(),
                        truncated: false,
                    });
                }
                _ => {}
            }
        }

        let children = Self::parse_children_recursively(&mut reader, 0, 1, max_depth)?;
        Ok(Self {
            range: 0..xml_document.len(),
            children,
            truncated: false,
        })
    }

    /// Parses events until (and including) the `End` that closes the
    /// element whose `Start` the caller just consumed, recording every
    /// child that matches `Elem` at `depth`, recursing further only while
    /// `depth` is still within `max_depth`. Children that don't match
    /// `Elem`, or that lie beyond `max_depth`, are still walked (to
    /// correctly find the end of their content) but discarded: anything
    /// nested inside an unmatched wrapper, or below the requested depth,
    /// stays invisible, exactly as if it had been skipped outright. A
    /// matched child recorded right at the `max_depth` boundary is marked
    /// [`truncated`](XmlDocumentIndex::is_truncated) so callers can tell
    /// "not scanned" apart from "scanned and empty".
    ///
    /// `base` is the absolute byte offset (within the document the
    /// top-level [`Reader`] was built from) of the *start* of the element
    /// whose content is being parsed. Every recorded range is relative to
    /// `base`, not to the document root, because callers re-slice
    /// `xml_document` down to one element's own bytes at every level and
    /// treat that slice as a new zero-based buffer.
    fn parse_children_recursively(
        reader: &mut Reader<&[u8]>,
        base: usize,
        depth: usize,
        max_depth: Option<usize>,
    ) -> Result<HashMap<Elem, Vec<Self>>, Error> {
        let mut children: HashMap<Elem, Vec<Self>> = HashMap::new();
        let at_max_depth = max_depth.is_some_and(|max| depth >= max);

        loop {
            match reader.read_event()? {
                Event::Start(e) => {
                    // buffer_position() is right after `>` of the start tag,
                    // so the `<` of the start tag is e.len() + 2 bytes back.
                    let abs_start = reader.buffer_position() as usize - e.len() - 2;
                    let matched = Elem::from_local_name(e.local_name().as_ref());

                    let grandchildren = if at_max_depth {
                        reader.read_to_end(e.name())?;
                        HashMap::new()
                    } else {
                        Self::parse_children_recursively(reader, abs_start, depth + 1, max_depth)?
                    };
                    // buffer_position() is now right after `>` of the closing tag.
                    let abs_end = reader.buffer_position() as usize;

                    if let Some(x) = matched {
                        children.entry(x).or_default().push(Self {
                            range: (abs_start - base)..(abs_end - base),
                            children: grandchildren,
                            truncated: at_max_depth,
                        });
                    }
                }
                Event::Empty(e) => {
                    if let Some(x) = Elem::from_local_name(e.local_name().as_ref()) {
                        // buffer_position() is right after `>` of `<foo/>`,
                        // so `<` is e.len() + 3 bytes back (for `<`, `/`, `>`).
                        let abs_start = reader.buffer_position() as usize - e.len() - 3;
                        let abs_end = reader.buffer_position() as usize;
                        children.entry(x).or_default().push(Self {
                            range: (abs_start - base)..(abs_end - base),
                            children: HashMap::new(),
                            // A self-closing tag genuinely has no children —
                            // that's known, not merely unscanned — even if
                            // it sits at the `max_depth` boundary.
                            truncated: false,
                        });
                    }
                }
                Event::End(_) | Event::Eof => return Ok(children),
                _ => {}
            }
        }
    }

    pub fn convert<Elem2: XmlElement>(
        &self,
        f: impl Fn(Elem) -> Option<Elem2> + Copy,
    ) -> XmlDocumentIndex<Elem2> {
        let children = self
            .children
            .iter()
            .filter_map(|(&k, v)| f(k).map(|k2| (k2, v.iter().map(|c| c.convert(f)).collect())))
            .collect();
        XmlDocumentIndex {
            range: self.range.clone(),
            children,
            truncated: self.truncated,
        }
    }
}

/// A leaf/child deserializer as passed to [`collect_child`], [`collect_children`],
/// and [`collect_children_lenient`]: the matched node's own byte slice, the
/// node itself (for further descent), and the config to keep threading down
/// in case that node turns out to be truncated.
///
/// Generic over `Config` (rather than fixed to this crate's own
/// [`DeserializationConfig`](crate::util::DeserializationConfig)) because
/// `collect_child` et al. never inspect `config` themselves — they only pass
/// it straight through to `deserializer` — so a downstream crate with its
/// own richer config type (e.g. one carrying settings beyond
/// `geometry_scan_depth`) can reuse these helpers with that type directly.
pub type Deserializer<Elem, T, E, Config> =
    fn(&[u8], &XmlDocumentIndex<Elem>, &Config) -> Result<T, E>;

pub fn collect_child<Elem, T, E, Config>(
    xml_document: &[u8],
    index: &XmlDocumentIndex<Elem>,
    element: Elem,
    config: &Config,
    deserializer: Deserializer<Elem, T, E, Config>,
) -> Result<Option<T>, E>
where
    Elem: XmlElement,
    E: From<Error>,
{
    let all_matches = index.get(element);
    if all_matches.len() >= 2 {
        debug!(
            "expected at most one {:?}, found {}",
            element,
            all_matches.len()
        );
    }
    match all_matches.first() {
        None => Ok(None),
        Some(child) => {
            let slice = &xml_document[child.range()];
            deserializer(slice, child, config).map(Some)
        }
    }
}

/// Deserializes every span of `element`, pairing each result with the byte
/// range it came from. Shared by [`collect_children`] (fail-fast) and
/// [`collect_children_lenient`] (skip-and-continue).
fn collect_children_raw<Elem, T, E, Config: Sync>(
    xml_document: &[u8],
    index: &XmlDocumentIndex<Elem>,
    element: Elem,
    config: &Config,
    deserializer: Deserializer<Elem, T, E, Config>,
) -> Vec<(Range<usize>, Result<T, E>)>
where
    Elem: XmlElement + Send + Sync,
    T: Send,
    E: Send,
{
    index
        .get(element)
        .into_par_iter()
        .map(|child| {
            let result = deserializer(&xml_document[child.range()], child, config);
            (child.range(), result)
        })
        .collect()
}

pub fn collect_children<Elem, T, E, Config: Sync>(
    xml_document: &[u8],
    index: &XmlDocumentIndex<Elem>,
    element: Elem,
    config: &Config,
    deserializer: Deserializer<Elem, T, E, Config>,
) -> Result<Vec<T>, E>
where
    Elem: XmlElement + Send + Sync,
    T: Send,
    E: Send,
{
    collect_children_raw(xml_document, index, element, config, deserializer)
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
pub fn collect_children_lenient<Elem, T, E, Config: Sync>(
    xml_document: &[u8],
    index: &XmlDocumentIndex<Elem>,
    element: Elem,
    config: &Config,
    deserializer: Deserializer<Elem, T, E, Config>,
) -> (Vec<T>, Vec<SkippedChild<Elem, E>>)
where
    Elem: XmlElement + Send + Sync,
    T: Send,
    E: Send + Debug,
{
    let mut values = Vec::new();
    let mut skipped = Vec::new();

    for (span, result) in collect_children_raw(xml_document, index, element, config, deserializer) {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal element vocabulary for exercising the scans without coupling
    /// their tests to a real vocabulary like `GmlElement`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestElement {
        A,
        B,
        C,
    }

    impl XmlElement for TestElement {
        fn from_local_name(local_name: &str) -> Option<Self> {
            match local_name {
                "A" => Some(Self::A),
                "B" => Some(Self::B),
                "C" => Some(Self::C),
                _ => None,
            }
        }

        fn local_name(&self) -> &'static str {
            match self {
                Self::A => "A",
                Self::B => "B",
                Self::C => "C",
            }
        }
    }

    fn slice<'a>(xml: &'a [u8], node: &XmlDocumentIndex<TestElement>) -> &'a [u8] {
        &xml[node.range.clone()]
    }

    mod direct_children_scan {
        use super::*;

        #[test]
        fn finds_each_direct_child() {
            let xml = b"<Root><A/><B/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            assert_eq!(index.children.get(&TestElement::A).unwrap().len(), 1);
            assert_eq!(index.children.get(&TestElement::B).unwrap().len(), 1);
        }

        #[test]
        fn records_every_occurrence_of_a_repeated_child() {
            let xml = b"<Root><A/><A/><A/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            assert_eq!(index.children.get(&TestElement::A).unwrap().len(), 3);
        }

        #[test]
        fn range_covers_the_full_start_to_end_tag() {
            let xml = b"<Root><A>text</A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert_eq!(slice(xml, a), b"<A>text</A>");
        }

        #[test]
        fn range_covers_a_self_closing_child() {
            let xml = b"<Root><A/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert_eq!(slice(xml, a), b"<A/>");
        }

        #[test]
        fn ignores_unmatched_elements() {
            let xml = b"<Root><Other/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            assert!(index.children.is_empty());
        }

        #[test]
        fn does_not_see_matches_nested_inside_a_direct_child() {
            let xml = b"<Root><A><B/></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            assert_eq!(index.children.get(&TestElement::A).unwrap().len(), 1);
            assert!(!index.children.contains_key(&TestElement::B));
        }

        #[test]
        fn does_not_see_matches_nested_inside_an_unmatched_wrapper() {
            // "Other" isn't in `TestElement`'s vocabulary, so it's an
            // unmatched wrapper — its content must stay invisible even
            // though `A` inside it would match on its own.
            let xml = b"<Root><Other><A/></Other></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            assert!(index.children.is_empty());
        }

        #[test]
        fn a_direct_child_carries_no_children_of_its_own() {
            let xml = b"<Root><A><B/></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert!(a.children.is_empty());
        }

        #[test]
        fn a_direct_child_with_its_own_child_is_marked_truncated() {
            let xml = b"<Root><A><B/></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert!(
                a.is_truncated(),
                "A has a B inside it that the depth-1 scan never looked for"
            );
        }

        #[test]
        fn a_childless_direct_child_is_not_marked_truncated() {
            let xml = b"<Root><A/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert!(
                !a.is_truncated(),
                "A is self-closing — it genuinely has no children"
            );
        }

        #[test]
        fn empty_root_has_no_children() {
            let xml = b"<Root/>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(1)).unwrap();

            assert!(index.children.is_empty());
        }
    }

    mod recursive_scan {
        use super::*;

        #[test]
        fn finds_a_direct_child() {
            let xml = b"<Root><A/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            assert_eq!(index.children.get(&TestElement::A).unwrap().len(), 1);
        }

        #[test]
        fn finds_matches_nested_inside_a_matched_child() {
            let xml = b"<Root><A><B/></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert_eq!(a.children.get(&TestElement::B).unwrap().len(), 1);
        }

        #[test]
        fn finds_matches_three_levels_deep() {
            let xml = b"<Root><A><B><C/></B></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            let a = &index.children[&TestElement::A][0];
            let b = &a.children[&TestElement::B][0];
            assert_eq!(b.children.get(&TestElement::C).unwrap().len(), 1);
        }

        #[test]
        fn does_not_see_matches_nested_inside_an_unmatched_wrapper() {
            // "Other" isn't in `TestElement`'s vocabulary, so it's an
            // unmatched wrapper — its content must stay invisible even
            // though `A` inside it would match on its own.
            let xml = b"<Root><Other><A/></Other></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            assert!(index.children.is_empty());
        }

        #[test]
        fn top_level_range_covers_the_full_start_to_end_tag() {
            let xml = b"<Root><A>text</A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert_eq!(slice(xml, a), b"<A>text</A>");
        }

        #[test]
        fn nested_range_is_relative_to_its_own_parent_not_the_document_root() {
            // Regression test: nested ranges must be usable the way every
            // caller uses them — by slicing the PARENT's own bytes
            // (`&xml_document[child.range()]` where `xml_document` is
            // already narrowed to the parent), not the original document.
            // Absolute-from-root offsets would read the wrong bytes here.
            let xml = b"<Root><A><B>text</B></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            let a = &index.children[&TestElement::A][0];
            let a_slice = slice(xml, a);
            assert_eq!(a_slice, b"<A><B>text</B></A>");

            let b = &a.children[&TestElement::B][0];
            assert_eq!(&a_slice[b.range.clone()], b"<B>text</B>");
        }

        #[test]
        fn records_every_occurrence_of_a_repeated_child() {
            let xml = b"<Root><A/><A/><A/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            assert_eq!(index.children.get(&TestElement::A).unwrap().len(), 3);
        }

        #[test]
        fn range_covers_a_self_closing_nested_child() {
            let xml = b"<Root><A><B/></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            let a = &index.children[&TestElement::A][0];
            let b = &a.children[&TestElement::B][0];
            assert_eq!(&slice(xml, a)[b.range.clone()], b"<B/>");
        }

        #[test]
        fn empty_root_has_no_children() {
            let xml = b"<Root/>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            assert!(index.children.is_empty());
        }

        #[test]
        fn stops_recording_matches_beyond_max_depth() {
            let xml = b"<Root><A><B><C/></B></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(2)).unwrap();

            let a = &index.children[&TestElement::A][0];
            let b = &a.children[&TestElement::B][0];
            assert!(b.children.is_empty());
        }

        #[test]
        fn a_node_at_the_max_depth_boundary_is_marked_truncated() {
            let xml = b"<Root><A><B><C/></B></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(2)).unwrap();

            let a = &index.children[&TestElement::A][0];
            let b = &a.children[&TestElement::B][0];
            assert!(
                b.is_truncated(),
                "B sits exactly at max_depth 2, so its own children were never inspected"
            );
        }

        #[test]
        fn nodes_within_max_depth_are_not_marked_truncated() {
            let xml = b"<Root><A><B><C/></B></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(2)).unwrap();

            let a = &index.children[&TestElement::A][0];
            assert!(
                !a.is_truncated(),
                "A is within max_depth 2 — its children were fully inspected"
            );
        }

        #[test]
        fn unbounded_scan_never_marks_anything_truncated() {
            let xml = b"<Root><A><B><C/></B></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();

            let a = &index.children[&TestElement::A][0];
            let b = &a.children[&TestElement::B][0];
            let c = &b.children[&TestElement::C][0];
            assert!(!a.is_truncated());
            assert!(!b.is_truncated());
            assert!(!c.is_truncated());
        }

        #[test]
        fn a_self_closing_element_at_the_boundary_is_not_truncated() {
            // A self-closing tag genuinely has no children — that's known,
            // not merely unscanned, even though it sits at the boundary.
            let xml = b"<Root><A><B/></A></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, Some(2)).unwrap();

            let a = &index.children[&TestElement::A][0];
            let b = &a.children[&TestElement::B][0];
            assert!(!b.is_truncated());
        }
    }

    mod collect_generic_over_config {
        use super::*;

        /// A config type from a hypothetical downstream crate — deliberately
        /// unrelated to (and not built from) this crate's own
        /// `DeserializationConfig`, to prove `collect_child`/`collect_children`
        /// don't require it: they only pass `config` through opaquely.
        struct ForeignConfig {
            marker: &'static str,
        }

        fn deserialize_a(
            _xml: &[u8],
            _node: &XmlDocumentIndex<TestElement>,
            config: &ForeignConfig,
        ) -> Result<&'static str, Error> {
            Ok(config.marker)
        }

        #[test]
        fn collect_child_works_with_a_foreign_config_type() {
            let xml = b"<Root><A/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();
            let config = ForeignConfig {
                marker: "from-downstream-crate",
            };

            let result =
                collect_child(xml, &index, TestElement::A, &config, deserialize_a).unwrap();

            assert_eq!(result, Some("from-downstream-crate"));
        }

        #[test]
        fn collect_children_works_with_a_foreign_config_type() {
            let xml = b"<Root><A/><A/></Root>";
            let index = XmlDocumentIndex::<TestElement>::from_scan(xml, None).unwrap();
            let config = ForeignConfig {
                marker: "from-downstream-crate",
            };

            let result: Vec<&'static str> =
                collect_children(xml, &index, TestElement::A, &config, deserialize_a).unwrap();

            assert_eq!(
                result,
                vec!["from-downstream-crate", "from-downstream-crate"]
            );
        }
    }
}
