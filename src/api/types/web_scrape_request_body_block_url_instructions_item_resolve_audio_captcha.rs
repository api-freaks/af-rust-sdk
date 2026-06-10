pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha {
    /// Solves audio CAPTCHA challenges.
    pub resolve_audio_captcha: Option<HashMap<String, serde_json::Value>>,
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
    resolve_audio_captcha: Option<HashMap<String, serde_json::Value>>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptchaBuilder {
    pub fn resolve_audio_captcha(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.resolve_audio_captcha = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha {
                resolve_audio_captcha: self.resolve_audio_captcha,
            },
        )
    }
}
