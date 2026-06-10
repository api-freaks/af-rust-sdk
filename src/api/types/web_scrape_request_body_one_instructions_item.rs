pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum WebScrapeRequestBodyOneInstructionsItem {
    WebScrapeRequestBodyOneInstructionsItemPostForm(
        WebScrapeRequestBodyOneInstructionsItemPostForm,
    ),

    WebScrapeRequestBodyOneInstructionsItemGetForm(WebScrapeRequestBodyOneInstructionsItemGetForm),

    WebScrapeRequestBodyOneInstructionsItemExtract(WebScrapeRequestBodyOneInstructionsItemExtract),

    WebScrapeRequestBodyOneInstructionsItemGetPage(WebScrapeRequestBodyOneInstructionsItemGetPage),
}

impl WebScrapeRequestBodyOneInstructionsItem {
    pub fn is_web_scrape_request_body_one_instructions_item_post_form(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyOneInstructionsItemPostForm(_)
        )
    }

    pub fn is_web_scrape_request_body_one_instructions_item_get_form(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyOneInstructionsItemGetForm(_)
        )
    }

    pub fn is_web_scrape_request_body_one_instructions_item_extract(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyOneInstructionsItemExtract(_)
        )
    }

    pub fn is_web_scrape_request_body_one_instructions_item_get_page(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyOneInstructionsItemGetPage(_)
        )
    }

    pub fn as_web_scrape_request_body_one_instructions_item_post_form(
        &self,
    ) -> Option<&WebScrapeRequestBodyOneInstructionsItemPostForm> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemPostForm(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_one_instructions_item_post_form(
        self,
    ) -> Option<WebScrapeRequestBodyOneInstructionsItemPostForm> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemPostForm(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_one_instructions_item_get_form(
        &self,
    ) -> Option<&WebScrapeRequestBodyOneInstructionsItemGetForm> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemGetForm(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_one_instructions_item_get_form(
        self,
    ) -> Option<WebScrapeRequestBodyOneInstructionsItemGetForm> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemGetForm(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_one_instructions_item_extract(
        &self,
    ) -> Option<&WebScrapeRequestBodyOneInstructionsItemExtract> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemExtract(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_one_instructions_item_extract(
        self,
    ) -> Option<WebScrapeRequestBodyOneInstructionsItemExtract> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemExtract(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_one_instructions_item_get_page(
        &self,
    ) -> Option<&WebScrapeRequestBodyOneInstructionsItemGetPage> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemGetPage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_one_instructions_item_get_page(
        self,
    ) -> Option<WebScrapeRequestBodyOneInstructionsItemGetPage> {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemGetPage(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for WebScrapeRequestBodyOneInstructionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebScrapeRequestBodyOneInstructionsItemPostForm(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyOneInstructionsItemGetForm(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyOneInstructionsItemExtract(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyOneInstructionsItemGetPage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
