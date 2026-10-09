#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressUsage {
    Business,
    Billing,
    Communication,
    Contact,
    Mailing,
    Personal,
    Postal,
    Residential,
}
