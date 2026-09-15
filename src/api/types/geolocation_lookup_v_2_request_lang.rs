pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeolocationLookupV2RequestLang {
    En,
    De,
    Ru,
    Ja,
    Fr,
    Cn,
    Es,
    Cs,
    It,
    Ko,
    Fa,
    Pt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GeolocationLookupV2RequestLang {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::En => serializer.serialize_str("en"),
            Self::De => serializer.serialize_str("de"),
            Self::Ru => serializer.serialize_str("ru"),
            Self::Ja => serializer.serialize_str("ja"),
            Self::Fr => serializer.serialize_str("fr"),
            Self::Cn => serializer.serialize_str("cn"),
            Self::Es => serializer.serialize_str("es"),
            Self::Cs => serializer.serialize_str("cs"),
            Self::It => serializer.serialize_str("it"),
            Self::Ko => serializer.serialize_str("ko"),
            Self::Fa => serializer.serialize_str("fa"),
            Self::Pt => serializer.serialize_str("pt"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GeolocationLookupV2RequestLang {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "en" => Ok(Self::En),
            "de" => Ok(Self::De),
            "ru" => Ok(Self::Ru),
            "ja" => Ok(Self::Ja),
            "fr" => Ok(Self::Fr),
            "cn" => Ok(Self::Cn),
            "es" => Ok(Self::Es),
            "cs" => Ok(Self::Cs),
            "it" => Ok(Self::It),
            "ko" => Ok(Self::Ko),
            "fa" => Ok(Self::Fa),
            "pt" => Ok(Self::Pt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GeolocationLookupV2RequestLang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::De => write!(f, "de"),
            Self::Ru => write!(f, "ru"),
            Self::Ja => write!(f, "ja"),
            Self::Fr => write!(f, "fr"),
            Self::Cn => write!(f, "cn"),
            Self::Es => write!(f, "es"),
            Self::Cs => write!(f, "cs"),
            Self::It => write!(f, "it"),
            Self::Ko => write!(f, "ko"),
            Self::Fa => write!(f, "fa"),
            Self::Pt => write!(f, "pt"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
