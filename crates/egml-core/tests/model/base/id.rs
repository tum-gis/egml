use egml_core::model::base::Id;
use egml_core::Error::MustNotBeEmpty;

#[test]
fn id_from_empty_string() {
    let result = Id::try_from("".to_string());

    assert_eq!(result, Err(MustNotBeEmpty("id")));
}

#[test]
fn test() {
    let source_text = "<gml:Point>
              <gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>
            </gml:Point>";

    let id_a = Id::from_hashed_string(source_text);
    let id_b = Id::from_hashed_string(source_text);

    assert_eq!(id_a, id_b);
}
