pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha {
    /// Instructions for solving image captchas.
    pub general_image_captcha: Option<
        Vec<WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem>,
    >,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaBuilder {
    general_image_captcha: Option<
        Vec<WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem>,
    >,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaBuilder {
    pub fn general_image_captcha(
        mut self,
        value: Vec<
            WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItem,
        >,
    ) -> Self {
        self.general_image_captcha = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha {
                general_image_captcha: self.general_image_captcha,
            },
        )
    }
}
