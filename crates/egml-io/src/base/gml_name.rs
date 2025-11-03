use crate::Error;
use egml_core::model::base::{AbstractGml, Id};
use quick_xml::Reader;
use quick_xml::events::Event;

pub fn parse_abstract_gml(source_text: &str, id: Id) -> Result<AbstractGml, Error> {
    let mut reader = Reader::from_str(source_text);
    reader.config_mut().trim_text(true);

    let mut names = Vec::new();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"gml:name" => {
                    names.push(reader.read_text(e.name())?.to_string());
                }
                _ => {
                    reader.read_to_end(e.name())?;
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(Error::from(e)),
            _ => (),
        }
    }

    Ok(AbstractGml { id, name: names })
}

#[cfg(test)]
mod tests {
    use crate::base::gml_name::parse_abstract_gml;
    use egml_core::model::base::Id;

    #[test]
    fn parsing_abstract_gml_with_two_names() {
        let id = Id::from_hashed_string("test_id");
        let source_text = "<gml:name>my_name_1</gml:name><gml:name>my_name_2</gml:name>";

        let abstract_gml = parse_abstract_gml(source_text, id).expect("");
        assert_eq!(abstract_gml.name, vec!["my_name_1", "my_name_2"]);
    }

    #[test]
    fn parsing_abstract_gml_with_other_elements() {
        let id = Id::from_hashed_string("test_id");
        let source_text = "
      <gml:name>0507</gml:name>
      <gml:boundedBy>
        <gml:Envelope srsName=\"urn:ogc:def:crs:EPSG::25832\" srsDimension=\"3\">
          <gml:lowerCorner>690984.6702564997 5336061.499479238 507.40999999999997</gml:lowerCorner>
          <gml:upperCorner>691088.7286507211 5336156.516425779 537.924940696475</gml:upperCorner>
        </gml:Envelope>
      </gml:boundedBy>";

        let abstract_gml = parse_abstract_gml(source_text, id).expect("");
        assert_eq!(abstract_gml.name, vec!["0507"]);
    }

    #[test]
    fn parsing_nested_abstract_gml() {
        let id = Id::from_hashed_string("test_id");
        let source_text = "
      <gml:name>0507</gml:name>
      <con:Window>
        <gml:name>window 23</gml:name>
      </con:Window>";

        let abstract_gml = parse_abstract_gml(source_text, id).expect("");
        assert_eq!(abstract_gml.name.len(), 1);
    }
}
