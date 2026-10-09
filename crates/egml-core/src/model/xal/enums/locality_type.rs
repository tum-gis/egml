#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalityType {
    Municipality,
    PostTown,
    Place,
    Suburb,
    Town,
    Village,
    Area,
    Zone,
}
