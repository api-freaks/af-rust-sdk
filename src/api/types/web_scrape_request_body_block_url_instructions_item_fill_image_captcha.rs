pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha {
    /// Captures and fills CAPTCHA values automatically.
    pub fill_image_captcha: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaBuilder {
    fill_image_captcha: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaBuilder {
    pub fn fill_image_captcha(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.fill_image_captcha = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha {
                fill_image_captcha: self.fill_image_captcha,
            },
        )
    }
}
