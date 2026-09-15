use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiFreaksEnvironment {
    #[serde(rename = "default")]
    Default,
}
impl ApiFreaksEnvironment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Default => "https://api.apifreaks.com",
        }
    }
}
impl Default for ApiFreaksEnvironment {
    fn default() -> Self {
        Self::Default
    }
}
