pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistered {
    Yes,
    No,
    Restricted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize
    for DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistered
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Yes => serializer.serialize_str("yes"),
            Self::No => serializer.serialize_str("no"),
            Self::Restricted => serializer.serialize_str("restricted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistered
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "yes" => Ok(Self::Yes),
            "no" => Ok(Self::No),
            "restricted" => Ok(Self::Restricted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistered
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yes => write!(f, "yes"),
            Self::No => write!(f, "no"),
            Self::Restricted => write!(f, "restricted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
