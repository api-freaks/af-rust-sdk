pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem {
    #[serde(rename = "imagePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(rename = "textField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<String>,
    #[serde(rename = "imageUpdatePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_update_path: Option<String>,
    #[serde(rename = "captchaFailedPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_failed_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<
        WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemModel,
    >,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem {
    pub fn builder(
    ) -> WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemBuilder
    {
        <WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemBuilder
{
    image_path: Option<String>,
    text_field: Option<String>,
    image_update_path: Option<String>,
    captcha_failed_path: Option<String>,
    model: Option<
        WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemModel,
    >,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemBuilder {
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
        value: WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemModel,
    ) -> Self {
        self.model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem`].
    pub fn build(
        self,
    ) -> Result<
        WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem,
        BuildError,
    > {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem {
            image_path: self.image_path,
            text_field: self.text_field,
            image_update_path: self.image_update_path,
            captcha_failed_path: self.captcha_failed_path,
            model: self.model,
        })
    }
}
