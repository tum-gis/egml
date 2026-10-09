use crate::Error;
use crate::util::formatting::Formatting;
use crate::util::{ChunkedSink, XmlElement, XmlNamespace};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

pub struct XmlFragmentWriter<W: Write> {
    writer: quick_xml::Writer<W>,
    namespace_prefixes: HashMap<&'static str, String>,
    formatting: Formatting,
    depth: usize,
}

impl<W: Write> XmlFragmentWriter<W> {
    pub fn new(writer: W, formatting: Formatting) -> Self {
        let xml_writer = quick_xml::Writer::new(writer);

        Self {
            writer: xml_writer,
            namespace_prefixes: HashMap::new(),
            formatting,
            depth: 0,
        }
    }

    pub fn formatting(&self) -> &Formatting {
        &self.formatting
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the prefix currently bound to `namespace` ("" if it has
    /// none), falling back to [`XmlNamespace::default_prefix`] when no
    /// binding has been set.
    pub fn namespace_prefix<N: XmlNamespace>(&self, namespace: N) -> &str {
        match self.namespace_prefixes.get(namespace.uri()) {
            Some(prefix) => prefix.as_str(),
            None => namespace.default_prefix().unwrap_or_default(),
        }
    }

    /// Overrides the prefix bound to `namespace` (e.g. from a document's
    /// `xmlns:*` declarations). Pass `""` to bind it as the default
    /// namespace (no prefix).
    pub fn set_namespace_prefix<N: XmlNamespace>(&mut self, namespace: N, prefix: String) {
        self.namespace_prefixes.insert(namespace.uri(), prefix);
    }

    /// Returns `element`'s fully-qualified tag name in `namespace`, using
    /// the prefix currently bound to it (e.g. `"gml:Polygon"`).
    fn qualify<N: XmlNamespace, E: XmlElement>(
        &self,
        namespace: N,
        element: E,
    ) -> Cow<'static, str> {
        match self.namespace_prefix(namespace) {
            "" => Cow::Borrowed(element.local_name()),
            prefix => Cow::Owned(format!("{prefix}:{}", element.local_name())),
        }
    }

    /// Sets the nesting depth new events are written at. Call this before
    /// writing anything to offset a whole fragment — e.g. a subtree
    /// serialized independently that will be spliced `depth` levels deep
    /// into the final document.
    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    /// Writes a start tag for `element`, qualified against `namespace` using
    /// the prefix currently bound to it (see
    /// [`namespace_prefix`](Self::namespace_prefix)) — e.g. `<gml:Envelope>`.
    /// The matching end tag is the caller's responsibility.
    pub fn write_start_event<N: XmlNamespace, E: XmlElement>(
        &mut self,
        namespace: N,
        element: E,
    ) -> Result<(), Error> {
        self.write_start_event_with_attributes(namespace, element, [] as [(&str, &str); 0])
    }

    /// Like [`write_start_event`](Self::write_start_event), but also writes
    /// `attributes` on the start tag.
    pub fn write_start_event_with_attributes<
        N: XmlNamespace,
        E: XmlElement,
        K: AsRef<str>,
        V: AsRef<str>,
    >(
        &mut self,
        namespace: N,
        element: E,
        attributes: impl IntoIterator<Item = (K, V)>,
    ) -> Result<(), Error> {
        let qualified_name = self.qualify(namespace, element);

        let mut start = BytesStart::new(qualified_name.as_ref());
        for (key, value) in attributes {
            start.push_attribute((key.as_ref(), value.as_ref()));
        }

        match self.formatting {
            Formatting::Compact => {}
            Formatting::NewLine => {}
            Formatting::Indent { char, size } => {
                let mut buffer = Vec::with_capacity(self.depth * size);
                buffer.resize(buffer.len() + self.depth * size, char);
                self.get_mut().write_all(&buffer)?;
            }
        };
        self.writer.write_event(Event::Start(start))?;
        match self.formatting {
            Formatting::Compact => {}
            Formatting::NewLine => self.writer.get_mut().write_all(b"\n")?,
            Formatting::Indent { char: _, size: _ } => self.writer.get_mut().write_all(b"\n")?,
        };

        self.depth += 1;

        Ok(())
    }

    pub fn write_end_event<N: XmlNamespace, E: XmlElement>(
        &mut self,
        namespace: N,
        element: E,
    ) -> Result<(), Error> {
        let qualified_name = self.qualify(namespace, element);

        self.depth = self.depth.saturating_sub(1);

        match self.formatting {
            Formatting::Compact => {}
            Formatting::NewLine => {}
            Formatting::Indent { char, size } => {
                let mut buffer = Vec::with_capacity(self.depth * size);
                buffer.resize(buffer.len() + self.depth * size, char);
                self.get_mut().write_all(&buffer)?;
            }
        };

        let end = BytesEnd::new(qualified_name.as_ref());
        self.writer.write_event(Event::End(end))?;

        match self.formatting {
            Formatting::Compact => {}
            Formatting::NewLine => self.writer.get_mut().write_all(b"\n")?,
            Formatting::Indent { char: _, size: _ } => self.writer.get_mut().write_all(b"\n")?,
        };

        Ok(())
    }

    pub fn write_leaf_element<'a, N: XmlNamespace, E: XmlElement>(
        &mut self,
        namespace: N,
        element: E,
        content: &str,
    ) -> Result<(), Error> {
        self.write_leaf_element_with_attributes(
            namespace,
            element,
            [] as [(&str, &str); 0],
            content,
        )
    }

    /// Writes a start tag (with `attributes`), `content` as escaped text,
    /// and the matching end tag as a single unit — e.g. `<area uom="m2">120</area>` —
    /// with no newlines between them regardless of `Formatting`. Like
    /// [`write_leaf_bytes`](Self::write_leaf_bytes), the whole element gets
    /// one indent prefix and trailing newline, not one per tag. `element`'s
    /// tag name is qualified against `namespace` using the prefix currently
    /// bound to it (see [`prefix`](Self::namespace_prefix)).
    pub fn write_leaf_element_with_attributes<
        N: XmlNamespace,
        E: XmlElement,
        K: AsRef<str>,
        V: AsRef<str>,
    >(
        &mut self,
        namespace: N,
        element: E,
        attributes: impl IntoIterator<Item = (K, V)>,
        content: &str,
    ) -> Result<(), Error> {
        let qualified_name = self.qualify(namespace, element);
        let name = qualified_name.as_ref();

        let prefix = indent_prefix(self.formatting, self.depth, true);
        let suffix = newline_suffix(self.formatting);
        if let Some(prefix) = &prefix {
            self.writer.get_mut().write_all(prefix)?;
        }

        let mut start = BytesStart::new(name);
        for (key, value) in attributes {
            start.push_attribute((key.as_ref(), value.as_ref()));
        }
        self.writer.write_event(Event::Start(start))?;
        self.writer
            .write_event(Event::Text(BytesText::new(content)))?;
        self.writer.write_event(Event::End(BytesEnd::new(name)))?;

        if let Some(suffix) = &suffix {
            self.writer.get_mut().write_all(suffix)?;
        }

        Ok(())
    }

    /// Writes a single self-closed empty element with `attributes` — e.g.
    /// `<tran:predecessor xlink:show="bogus"/>` — as one atomic unit: one
    /// indent prefix and trailing newline, like
    /// [`write_leaf_element_with_attributes`](Self::write_leaf_element_with_attributes),
    /// but with no content and no separate end tag. `element`'s tag name is
    /// qualified against `namespace` using the prefix currently bound to it
    /// (see [`namespace_prefix`](Self::namespace_prefix)).
    pub fn write_empty_element_with_attributes<
        N: XmlNamespace,
        E: XmlElement,
        K: AsRef<str>,
        V: AsRef<str>,
    >(
        &mut self,
        namespace: N,
        element: E,
        attributes: impl IntoIterator<Item = (K, V)>,
    ) -> Result<(), Error> {
        let qualified_name = self.qualify(namespace, element);
        let name = qualified_name.as_ref();

        let prefix = indent_prefix(self.formatting, self.depth, true);
        let suffix = newline_suffix(self.formatting);
        if let Some(prefix) = &prefix {
            self.writer.get_mut().write_all(prefix)?;
        }

        let mut empty = BytesStart::new(name);
        for (key, value) in attributes {
            empty.push_attribute((key.as_ref(), value.as_ref()));
        }
        self.writer.write_event(Event::Empty(empty))?;

        if let Some(suffix) = &suffix {
            self.writer.get_mut().write_all(suffix)?;
        }

        Ok(())
    }

    pub fn write_leaf_bytes(&mut self, buf: &[u8]) -> Result<(), Error> {
        let prefix = indent_prefix(self.formatting, self.depth, true);
        let suffix = newline_suffix(self.formatting);
        if let Some(prefix) = prefix {
            self.writer.get_mut().write_all(&prefix)?;
        }
        self.writer.get_mut().write_all(buf)?;
        if let Some(suffix) = suffix {
            self.writer.get_mut().write_all(&suffix)?;
        }

        Ok(())
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.writer.get_mut()
    }

    /// Creates an in-memory fragment writer for serializing a subtree
    /// independently (e.g. on another thread) before splicing its bytes
    /// back in via [`get_mut`](Self::get_mut) or
    /// [`write_leaf_bytes`](Self::write_leaf_bytes). Inherits this writer's
    /// formatting and namespace prefixes, and starts one level deeper than
    /// this writer's current depth, so the child's own qualified names and
    /// indentation already match where it will be spliced.
    pub fn spawn_child(&self) -> XmlFragmentWriter<ChunkedSink> {
        let chunked_sink = ChunkedSink::new();
        let mut child: XmlFragmentWriter<ChunkedSink> =
            XmlFragmentWriter::new(chunked_sink, self.formatting);
        child.namespace_prefixes = self.namespace_prefixes.clone();
        child.set_depth(self.depth + 1);
        child
    }

    /// Splices `bytes` in verbatim — e.g. the output of a
    /// [`spawn_child`](Self::spawn_child) fragment once serialized. Unlike
    /// [`write_leaf_bytes`](Self::write_leaf_bytes), no indent prefix or
    /// trailing newline is added, since a spawned child already carries its
    /// own (from the depth it was spawned at).
    pub fn splice_child_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.get_mut().write_all(bytes)?;
        Ok(())
    }

    pub fn into_inner(self) -> quick_xml::Writer<W> {
        self.writer
    }
}

impl XmlFragmentWriter<ChunkedSink> {
    pub fn new_chunked(formatting: Formatting) -> Self {
        Self::new(ChunkedSink::new(), formatting)
    }

    /// Merges a nested chunked child's fragments into this one — e.g. a
    /// subtree that itself spawned chunked children (see
    /// [`spawn_child`](Self::spawn_child)). O(1) regardless of how many
    /// chunks `other` holds, since it relinks the underlying chunks instead
    /// of copying them.
    pub fn merge_chunked(&mut self, other: XmlFragmentWriter<ChunkedSink>) {
        self.get_mut().append(other.into_inner().into_inner());
    }

    /// Streams every accumulated chunk to `sink` in order. This is where
    /// the copies deferred by [`merge_chunked`](Self::merge_chunked)
    /// finally happen — one per chunk, the same unavoidable cost as writing
    /// directly to `sink` — but paid exactly once, not once per merge.
    pub fn write_to<W: Write>(self, sink: &mut W) -> Result<(), Error> {
        self.into_inner().into_inner().write_to(sink)?;
        Ok(())
    }
}

impl XmlFragmentWriter<Vec<u8>> {
    pub fn new_in_memory(formatting: Formatting) -> Self {
        let buffer = Vec::new();
        Self::new(buffer, formatting)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.into_inner().into_inner()
    }
}

/// Builds the whitespace to write *before* a `Start`/`End`/`Empty` event,
/// given the nesting level it sits at. Returns `None` for non-structural
/// events (`Text`, ...) and for `Formatting::Compact`.
fn indent_prefix(formatting: Formatting, level: usize, is_structural: bool) -> Option<Vec<u8>> {
    if !is_structural {
        return None;
    }

    match formatting {
        Formatting::Compact => None,
        Formatting::NewLine => None,
        Formatting::Indent { char, size } => {
            let mut buffer = Vec::with_capacity(level * size);
            buffer.resize(buffer.len() + level * size, char);
            Some(buffer)
        }
    }
}

/// Builds the newline to write *after* an event. `None` for
/// `Formatting::Compact`.
fn newline_suffix(formatting: Formatting) -> Option<Vec<u8>> {
    match formatting {
        Formatting::Compact => None,
        Formatting::NewLine => Some(b"\n".to_vec()),
        Formatting::Indent { char: _, size: _ } => Some(b"\n".to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::Reader;
    use quick_xml::events::Event;

    /// Minimal namespace/element fixtures for exercising the generic writer
    /// without coupling its tests to a real vocabulary like `GmlElement`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct TestNamespace;

    impl XmlNamespace for TestNamespace {
        fn uri(&self) -> &'static str {
            "urn:example:test"
        }

        fn default_prefix(&self) -> Option<&'static str> {
            None
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestElement {
        Area,
        Note,
        CityModel,
        Building,
        Building2,
        Address,
        Sibling,
    }

    impl XmlElement for TestElement {
        fn from_local_name(local_name: &str) -> Option<Self> {
            match local_name {
                "area" => Some(Self::Area),
                "note" => Some(Self::Note),
                "CityModel" => Some(Self::CityModel),
                "Building" => Some(Self::Building),
                "Building2" => Some(Self::Building2),
                "Address" => Some(Self::Address),
                "sibling" => Some(Self::Sibling),
                _ => None,
            }
        }

        fn local_name(&self) -> &'static str {
            match self {
                Self::Area => "area",
                Self::Note => "note",
                Self::CityModel => "CityModel",
                Self::Building => "Building",
                Self::Building2 => "Building2",
                Self::Address => "Address",
                Self::Sibling => "sibling",
            }
        }
    }

    /// Writes `CityModel { Building {}, Building2 { Address {} } }`: two
    /// levels deep, with a leaf that has no children, so every depth
    /// transition (down, across, up, up-through-a-sibling) is covered.
    fn write_sample_tree(formatting: Formatting, current_level: usize) -> String {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        writer.set_depth(current_level);

        writer
            .write_start_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel start");
        writer
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start");
        writer
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end");
        writer
            .write_start_event(TestNamespace, TestElement::Building2)
            .expect("write Building2 start");
        writer
            .write_start_event(TestNamespace, TestElement::Address)
            .expect("write Address start");
        writer
            .write_end_event(TestNamespace, TestElement::Address)
            .expect("write Address end");
        writer
            .write_end_event(TestNamespace, TestElement::Building2)
            .expect("write Building2 end");
        writer
            .write_end_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel end");

        let bytes = writer.into_bytes();
        String::from_utf8(bytes).expect("output should be valid UTF-8")
    }

    #[test]
    fn compact_has_no_whitespace_regardless_of_level() {
        let expected = "<CityModel><Building></Building><Building2><Address></Address></Building2></CityModel>";

        assert_eq!(write_sample_tree(Formatting::Compact, 0), expected);
        assert_eq!(write_sample_tree(Formatting::Compact, 5), expected);
    }

    #[test]
    fn new_line_prefixes_every_structural_event_ignoring_level() {
        let expected = "<CityModel>\n<Building>\n</Building>\n<Building2>\n<Address>\n</Address>\n\
             </Building2>\n</CityModel>\n";

        assert_eq!(write_sample_tree(Formatting::NewLine, 0), expected);
        assert_eq!(write_sample_tree(Formatting::NewLine, 7), expected);
    }

    #[test]
    fn indent_reflects_nesting_depth_at_base_level_zero() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };

        let expected = "<CityModel>\n\t<Building>\n\t</Building>\n\t<Building2>\n\t\t\
             <Address>\n\t\t</Address>\n\t</Building2>\n</CityModel>\n";

        assert_eq!(write_sample_tree(formatting, 0), expected);
    }

    #[test]
    fn indent_prefixes_the_very_first_line_with_the_base_level_offset() {
        // A fragment serialized with a non-zero base depth (e.g. for later
        // splicing into a larger document) must already carry its leading
        // indentation on the first line, without a leading newline.
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };

        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        writer.set_depth(3);
        writer
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start");
        writer
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");

        assert_eq!(text, "\t\t\t<Building>\n\t\t\t</Building>\n");
    }

    #[test]
    fn indent_omits_the_leading_newline_on_the_very_first_line_at_base_level_zero() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };

        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        writer
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start");
        writer
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");

        assert_eq!(text, "<Building>\n</Building>\n");
    }

    #[test]
    fn indent_adds_base_level_offset_on_top_of_nesting_depth() {
        // Simulates a fragment a parallel worker serialized for a subtree
        // that will be spliced 3 levels deep into the final document.
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };

        let indent = |level: usize| "\t".repeat(level);
        let expected = format!(
            "{}<CityModel>\n{}<Building>\n{}</Building>\n{}<Building2>\n{}\
             <Address>\n{}</Address>\n{}</Building2>\n{}</CityModel>\n",
            indent(3),
            indent(4),
            indent(4),
            indent(4),
            indent(5),
            indent(5),
            indent(4),
            indent(3),
        );

        assert_eq!(write_sample_tree(formatting, 3), expected);
    }

    #[test]
    fn indent_with_zero_size_only_emits_newlines() {
        let formatting = Formatting::Indent {
            char: b'-',
            size: 0,
        };

        let expected = "<CityModel>\n<Building>\n</Building>\n<Building2>\n<Address>\n</Address>\n\
             </Building2>\n</CityModel>\n";

        assert_eq!(write_sample_tree(formatting, 0), expected);
        assert_eq!(write_sample_tree(formatting, 4), expected);
    }

    #[test]
    fn parallel_fragments_stitch_into_a_consistently_indented_document() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };

        // Two independent "workers" each serialize one Building at depth 1
        // (as if inside a CityModel they know nothing about), unaware of
        // each other. Each fragment already carries its own leading indent
        // from set_depth, so the caller only needs to insert the newline
        // that separates it from whatever came before it.
        let mut fragment_a = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        fragment_a.set_depth(1);
        fragment_a
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start fragment a");
        fragment_a
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end fragment a");
        let fragment_a_bytes = fragment_a.into_bytes();

        let mut fragment_b = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        fragment_b.set_depth(1);
        fragment_b
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start fragment b");
        fragment_b
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end fragment b");
        let fragment_b_bytes = fragment_b.into_bytes();

        let mut document = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        document
            .write_start_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel start");
        document
            .splice_child_bytes(&fragment_a_bytes)
            .expect("stitch fragment a");
        document
            .splice_child_bytes(&fragment_b_bytes)
            .expect("stitch fragment b");
        document
            .write_end_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel end");

        let bytes = document.into_bytes();
        let text = String::from_utf8(bytes).expect("output should be valid UTF-8");

        assert_eq!(
            text,
            "<CityModel>\n\t<Building>\n\t</Building>\n\t<Building>\n\t</Building>\n</CityModel>\n"
        );
    }

    #[test]
    fn chunked_sink_splices_fragments_without_copying_into_a_consistently_indented_document() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };

        let chunked_sink = ChunkedSink::new();
        let mut document = XmlFragmentWriter::new(chunked_sink, formatting);
        document
            .write_start_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel start");

        let mut fragment_a = document.spawn_child();
        fragment_a
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start fragment a");
        fragment_a
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end fragment a");
        document.merge_chunked(fragment_a);

        let mut fragment_b = document.spawn_child();
        fragment_b
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start fragment b");
        fragment_b
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end fragment b");
        document.merge_chunked(fragment_b);

        document
            .write_end_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel end");

        let mut bytes = Vec::new();
        document.write_to(&mut bytes).expect("stream chunks out");
        let text = String::from_utf8(bytes).expect("output should be valid UTF-8");

        assert_eq!(
            text,
            "<CityModel>\n\t\t<Building>\n\t\t</Building>\n\t\t<Building>\n\t\t</Building>\n</CityModel>\n"
        );
    }

    #[test]
    fn merge_chunked_relinks_nested_grandchildren_in_order() {
        // Mirrors nested parallel serialization: a chunked child spawns its
        // own chunked children, splices them into itself first, and only
        // then gets merged into the root — merge_chunked must carry all of
        // its grandchildren's chunks along, in order, in O(1).
        let formatting = Formatting::Compact;

        let mut root = XmlFragmentWriter::<ChunkedSink>::new_chunked(formatting);
        root.write_start_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel start");

        let mut middle = root.spawn_child();
        middle
            .write_start_event(TestNamespace, TestElement::Building2)
            .expect("write Building2 start");

        let mut grandchild = middle.spawn_child();
        grandchild
            .write_start_event(TestNamespace, TestElement::Address)
            .expect("write Address start");
        grandchild
            .write_end_event(TestNamespace, TestElement::Address)
            .expect("write Address end");
        middle.merge_chunked(grandchild);

        middle
            .write_end_event(TestNamespace, TestElement::Building2)
            .expect("write Building2 end");
        root.merge_chunked(middle);

        root.write_end_event(TestNamespace, TestElement::CityModel)
            .expect("write CityModel end");

        let mut bytes = Vec::new();
        root.write_to(&mut bytes).expect("stream chunks out");
        let text = String::from_utf8(bytes).expect("output should be valid UTF-8");

        assert_eq!(
            text,
            "<CityModel><Building2><Address></Address></Building2></CityModel>"
        );
    }

    #[test]
    fn indent_output_round_trips_to_the_same_event_structure() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };
        let text = write_sample_tree(formatting, 0);

        let mut reader = Reader::from_str(&text);
        reader.config_mut().trim_text(true);

        let mut structural_names = Vec::new();
        loop {
            match reader.read_event().expect("well-formed XML") {
                Event::Start(start) => {
                    structural_names.push(format!("Start({})", start.name().as_ref()));
                }
                Event::End(end) => {
                    structural_names.push(format!("End({})", end.name().as_ref()));
                }
                Event::Empty(empty) => {
                    structural_names.push(format!("Empty({})", empty.name().as_ref()));
                }
                Event::Eof => break,
                _ => {}
            }
        }

        assert_eq!(
            structural_names,
            vec![
                "Start(CityModel)",
                "Start(Building)",
                "End(Building)",
                "Start(Building2)",
                "Start(Address)",
                "End(Address)",
                "End(Building2)",
                "End(CityModel)",
            ]
        );
    }

    #[test]
    fn leaf_element_stays_on_one_line_under_compact() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_leaf_element_with_attributes(
                TestNamespace,
                TestElement::Area,
                [("uom", "m2")],
                "120",
            )
            .expect("write leaf element");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, r#"<area uom="m2">120</area>"#);
    }

    #[test]
    fn leaf_element_stays_on_one_line_under_indent() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        writer.set_depth(2);

        writer
            .write_leaf_element_with_attributes(
                TestNamespace,
                TestElement::Area,
                [("uom", "m2")],
                "120",
            )
            .expect("write leaf element");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, "\t\t<area uom=\"m2\">120</area>\n");
    }

    #[test]
    fn leaf_element_escapes_content() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_leaf_element_with_attributes(
                TestNamespace,
                TestElement::Note,
                [] as [(&str, &str); 0],
                "a < b & c",
            )
            .expect("write leaf element");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, "<note>a &lt; b &amp; c</note>");
    }

    #[test]
    fn leaf_element_does_not_change_depth() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_leaf_element_with_attributes(
                TestNamespace,
                TestElement::Area,
                [("uom", "m2")],
                "120",
            )
            .expect("write leaf element");
        writer
            .write_start_event(TestNamespace, TestElement::Sibling)
            .expect("write sibling start");
        writer
            .write_end_event(TestNamespace, TestElement::Sibling)
            .expect("write sibling end");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, r#"<area uom="m2">120</area><sibling></sibling>"#);
    }

    #[test]
    fn empty_element_writes_self_closed_tag_under_compact() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_empty_element_with_attributes(TestNamespace, TestElement::Area, [("uom", "m2")])
            .expect("write empty element");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, r#"<area uom="m2"/>"#);
    }

    #[test]
    fn empty_element_stays_on_one_line_under_indent() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        writer.set_depth(2);

        writer
            .write_empty_element_with_attributes(TestNamespace, TestElement::Area, [("uom", "m2")])
            .expect("write empty element");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, "\t\t<area uom=\"m2\"/>\n");
    }

    #[test]
    fn empty_element_without_attributes_writes_self_closed_tag() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_empty_element_with_attributes(
                TestNamespace,
                TestElement::Note,
                [] as [(&str, &str); 0],
            )
            .expect("write empty element");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, "<note/>");
    }

    #[test]
    fn empty_element_does_not_change_depth() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_empty_element_with_attributes(TestNamespace, TestElement::Area, [("uom", "m2")])
            .expect("write empty element");
        writer
            .write_start_event(TestNamespace, TestElement::Sibling)
            .expect("write sibling start");
        writer
            .write_end_event(TestNamespace, TestElement::Sibling)
            .expect("write sibling end");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, r#"<area uom="m2"/><sibling></sibling>"#);
    }

    #[test]
    fn start_event_writes_qualified_open_tag_and_increments_depth() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_start_event(TestNamespace, TestElement::Area)
            .expect("write start event");
        writer
            .write_end_event(TestNamespace, TestElement::Area)
            .expect("write end event");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, "<area></area>");
    }

    #[test]
    fn spawn_child_inherits_formatting_prefix_and_depth_plus_one() {
        let formatting = Formatting::Indent {
            char: b'\t',
            size: 1,
        };
        let mut parent = XmlFragmentWriter::<Vec<u8>>::new_in_memory(formatting);
        parent.set_namespace_prefix(TestNamespace, "t".to_string());
        parent.set_depth(2);

        let mut child = parent.spawn_child();
        child
            .write_start_event(TestNamespace, TestElement::Building)
            .expect("write Building start");
        child
            .write_end_event(TestNamespace, TestElement::Building)
            .expect("write Building end");

        let mut bytes = Vec::new();
        child.write_to(&mut bytes).expect("stream chunks out");
        let text = String::from_utf8(bytes).expect("valid UTF-8");
        assert_eq!(text, "\t\t\t<t:Building>\n\t\t\t</t:Building>\n");
    }

    #[test]
    fn start_event_with_attributes_writes_them_on_the_open_tag() {
        let mut writer = XmlFragmentWriter::<Vec<u8>>::new_in_memory(Formatting::Compact);

        writer
            .write_start_event_with_attributes(TestNamespace, TestElement::Area, [("uom", "m2")])
            .expect("write start event with attributes");
        writer
            .write_end_event(TestNamespace, TestElement::Area)
            .expect("write end event");

        let text = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(text, r#"<area uom="m2"></area>"#);
    }
}
