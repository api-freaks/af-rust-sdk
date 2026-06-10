pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WebScrapeRequestBody {
    WebScrapeRequestBodyBlockUrl(WebScrapeRequestBodyBlockUrl),

    WebScrapeRequestBodyOne(WebScrapeRequestBodyOne),
}

impl WebScrapeRequestBody {
    pub fn is_web_scrape_request_body_block_url(&self) -> bool {
        matches!(self, Self::WebScrapeRequestBodyBlockUrl(_))
    }

    pub fn is_web_scrape_request_body_one(&self) -> bool {
        matches!(self, Self::WebScrapeRequestBodyOne(_))
    }

    pub fn as_web_scrape_request_body_block_url(&self) -> Option<&WebScrapeRequestBodyBlockUrl> {
        match self {
            Self::WebScrapeRequestBodyBlockUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url(self) -> Option<WebScrapeRequestBodyBlockUrl> {
        match self {
            Self::WebScrapeRequestBodyBlockUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_one(&self) -> Option<&WebScrapeRequestBodyOne> {
        match self {
            Self::WebScrapeRequestBodyOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_one(self) -> Option<WebScrapeRequestBodyOne> {
        match self {
            Self::WebScrapeRequestBodyOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for WebScrapeRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebScrapeRequestBodyBlockUrl(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
