pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScreenshotCaptureResponseClip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}

impl ScreenshotCaptureResponseClip {
    pub fn builder() -> ScreenshotCaptureResponseClipBuilder {
        <ScreenshotCaptureResponseClipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScreenshotCaptureResponseClipBuilder {
    height: Option<i64>,
    width: Option<i64>,
    x: Option<i64>,
    y: Option<i64>,
}

impl ScreenshotCaptureResponseClipBuilder {
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn x(mut self, value: i64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: i64) -> Self {
        self.y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScreenshotCaptureResponseClip`].
    pub fn build(self) -> Result<ScreenshotCaptureResponseClip, BuildError> {
        Ok(ScreenshotCaptureResponseClip {
            height: self.height,
            width: self.width,
            x: self.x,
            y: self.y,
        })
    }
}
