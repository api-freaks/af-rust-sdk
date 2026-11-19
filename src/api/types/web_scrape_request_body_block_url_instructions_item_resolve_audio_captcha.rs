pub use crate::prelude::*;

/// Selectors for locating and solving a single audio CAPTCHA challenge.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha {
    /// Selector for the audio CAPTCHA element.
    #[serde(rename = "audioPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_path: Option<String>,
    /// Selector for the field where the resolved audio CAPTCHA text is entered.
    #[serde(rename = "textField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptchaBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptchaBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptchaBuilder {
    audio_path: Option<String>,
    text_field: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptchaBuilder {
    pub fn audio_path(mut self, value: impl Into<String>) -> Self {
        self.audio_path = Some(value.into());
        self
    }

    pub fn text_field(mut self, value: impl Into<String>) -> Self {
        self.text_field = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha {
                audio_path: self.audio_path,
                text_field: self.text_field,
            },
        )
    }
}
