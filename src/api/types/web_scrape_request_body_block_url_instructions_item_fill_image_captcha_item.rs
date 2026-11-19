pub use crate::prelude::*;

/// Instructions for locating and solving a single image CAPTCHA challenge.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem {
    /// Selector for the CAPTCHA image element.
    #[serde(rename = "imagePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    /// Selector for the field where the CAPTCHA text is entered.
    #[serde(rename = "textField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<String>,
    /// Selector to refresh or update the CAPTCHA image.
    #[serde(rename = "imageUpdatePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_update_path: Option<String>,
    /// Selector indicating a CAPTCHA failure so the solver can retry.
    #[serde(rename = "captchaFailedPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_failed_path: Option<String>,
    /// Model used for CAPTCHA solving.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemModel>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemBuilder {
    image_path: Option<String>,
    text_field: Option<String>,
    image_update_path: Option<String>,
    captcha_failed_path: Option<String>,
    model: Option<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemModel>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemBuilder {
    pub fn image_path(mut self, value: impl Into<String>) -> Self {
        self.image_path = Some(value.into());
        self
    }

    pub fn text_field(mut self, value: impl Into<String>) -> Self {
        self.text_field = Some(value.into());
        self
    }

    pub fn image_update_path(mut self, value: impl Into<String>) -> Self {
        self.image_update_path = Some(value.into());
        self
    }

    pub fn captcha_failed_path(mut self, value: impl Into<String>) -> Self {
        self.captcha_failed_path = Some(value.into());
        self
    }

    pub fn model(
        mut self,
        value: WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemModel,
    ) -> Self {
        self.model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem {
                image_path: self.image_path,
                text_field: self.text_field,
                image_update_path: self.image_update_path,
                captcha_failed_path: self.captcha_failed_path,
                model: self.model,
            },
        )
    }
}
