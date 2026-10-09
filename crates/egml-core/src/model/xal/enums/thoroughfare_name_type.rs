#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThoroughfareNameType {
    NameOnly,
    PreDirection,
    PostDirection,
    NameAndNumber,
    NameAndType,
    NameNumberAndType,
    Unstructured,
    SubThoroughfareConnector,
    ReferenceLocation,
    Type,
}
