pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OcrPredictResponseOcrText {
    StringList(Vec<String>),

    StringListList(Vec<Vec<String>>),
}

impl OcrPredictResponseOcrText {
    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn is_string_list_list(&self) -> bool {
        matches!(self, Self::StringListList(_))
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_list_list(&self) -> Option<&Vec<Vec<String>>> {
        match self {
            Self::StringListList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list_list(self) -> Option<Vec<Vec<String>>> {
        match self {
            Self::StringListList(value) => Some(value),
            _ => None,
        }
    }
}
