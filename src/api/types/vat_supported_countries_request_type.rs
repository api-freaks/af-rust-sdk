pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VatSupportedCountriesRequestType {
    Iban,
    Swift,
    Vat,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VatSupportedCountriesRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Iban => serializer.serialize_str("IBAN"),
            Self::Swift => serializer.serialize_str("SWIFT"),
            Self::Vat => serializer.serialize_str("VAT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VatSupportedCountriesRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "IBAN" => Ok(Self::Iban),
            "SWIFT" => Ok(Self::Swift),
            "VAT" => Ok(Self::Vat),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VatSupportedCountriesRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Iban => write!(f, "IBAN"),
            Self::Swift => write!(f, "SWIFT"),
            Self::Vat => write!(f, "VAT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
