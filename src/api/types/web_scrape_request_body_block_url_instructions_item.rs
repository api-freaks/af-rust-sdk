pub use crate::prelude::*;

/// A single browser scraping step. Exactly one of these action fields is set per instruction; every other field is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItem {
    /// A selector/value pair used by fill, select, and clickButtonByValue instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WebScrapeRequestBodyBlockUrlInstructionsItemFill>,
    /// Click action: CSS selector or XPath identifying the element to click.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click: Option<String>,
    /// Click if exists action: CSS selector or XPath identifying the element to click, only if it appears on the page within a short timeout.
    #[serde(rename = "clickIfExist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_if_exist: Option<String>,
    /// Enter key action: CSS selector or XPath identifying the element to send an Enter keystroke to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter: Option<String>,
    /// New tab action: set to true to switch focus to a newly opened browser tab.
    #[serde(rename = "newTab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tab: Option<bool>,
    /// Move to relative tab action: relative tab index to move to (e.g. 1 for the next tab, -1 for the previous tab).
    #[serde(rename = "moveToRelativeTab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_relative_tab: Option<i64>,
    /// Wait action: pauses execution for this many milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<i64>,
    /// Wait for element action: CSS selector or XPath identifying the element to wait for before continuing.
    #[serde(rename = "waitFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<String>,
    /// A selector/value pair used by fill, select, and clickButtonByValue instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<WebScrapeRequestBodyBlockUrlInstructionsItemSelect>,
    /// Execute JavaScript action: JavaScript code to execute inside the page context.
    #[serde(rename = "jsExe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js_exe: Option<String>,
    /// Conditional check action: list of conditional steps, each supporting if-<method> and else-<method> keys with conditions, executed based on element state.
    #[serde(rename = "conditionalCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_check: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// A selector/value pair used by fill, select, and clickButtonByValue instructions.
    #[serde(rename = "clickButtonByValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_button_by_value:
        Option<WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue>,
    /// Image captcha solver action: instructions for solving image-based captchas using a selected model.
    #[serde(rename = "generalImageCaptcha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_image_captcha:
        Option<Vec<WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaItem>>,
    /// Block element action: list of CSS selectors or XPaths for elements to block or hide on the page.
    #[serde(rename = "blockElement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_element: Option<Vec<String>>,
    /// Maps arbitrary output field names to CSS or XPath selectors. Each key becomes a field in the response's extractedData object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract: Option<HashMap<String, String>>,
    /// Fill image captcha action: captures and fills image-based captcha values automatically.
    #[serde(rename = "fillImageCaptcha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_image_captcha:
        Option<Vec<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem>>,
    /// Switch to iframe action: switches the browser context into an iframe identified by name or ID.
    #[serde(rename = "switchToIframe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_to_iframe: Option<String>,
    /// Switch to parent frame action: returns the browser context from an iframe back to its parent frame.
    #[serde(rename = "switchToParentFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_to_parent_frame: Option<bool>,
    /// Selectors for locating and solving a single audio CAPTCHA challenge.
    #[serde(rename = "resolveAudioCaptcha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_audio_captcha:
        Option<WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha>,
    /// Screenshot action: captures a screenshot of the current page state. Can be a boolean-like value or a selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<String>,
    /// Save image action: saves an image from the page by CSS selector or element ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saveimage: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItem {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemBuilder {
    fill: Option<WebScrapeRequestBodyBlockUrlInstructionsItemFill>,
    click: Option<String>,
    click_if_exist: Option<String>,
    enter: Option<String>,
    new_tab: Option<bool>,
    move_to_relative_tab: Option<i64>,
    wait: Option<i64>,
    wait_for: Option<String>,
    select: Option<WebScrapeRequestBodyBlockUrlInstructionsItemSelect>,
    js_exe: Option<String>,
    conditional_check: Option<Vec<HashMap<String, serde_json::Value>>>,
    click_button_by_value: Option<WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue>,
    general_image_captcha:
        Option<Vec<WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaItem>>,
    block_element: Option<Vec<String>>,
    extract: Option<HashMap<String, String>>,
    fill_image_captcha:
        Option<Vec<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem>>,
    switch_to_iframe: Option<String>,
    switch_to_parent_frame: Option<bool>,
    resolve_audio_captcha: Option<WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha>,
    screenshot: Option<String>,
    saveimage: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemBuilder {
    pub fn fill(mut self, value: WebScrapeRequestBodyBlockUrlInstructionsItemFill) -> Self {
        self.fill = Some(value);
        self
    }

    pub fn click(mut self, value: impl Into<String>) -> Self {
        self.click = Some(value.into());
        self
    }

    pub fn click_if_exist(mut self, value: impl Into<String>) -> Self {
        self.click_if_exist = Some(value.into());
        self
    }

    pub fn enter(mut self, value: impl Into<String>) -> Self {
        self.enter = Some(value.into());
        self
    }

    pub fn new_tab(mut self, value: bool) -> Self {
        self.new_tab = Some(value);
        self
    }

    pub fn move_to_relative_tab(mut self, value: i64) -> Self {
        self.move_to_relative_tab = Some(value);
        self
    }

    pub fn wait(mut self, value: i64) -> Self {
        self.wait = Some(value);
        self
    }

    pub fn wait_for(mut self, value: impl Into<String>) -> Self {
        self.wait_for = Some(value.into());
        self
    }

    pub fn select(mut self, value: WebScrapeRequestBodyBlockUrlInstructionsItemSelect) -> Self {
        self.select = Some(value);
        self
    }

    pub fn js_exe(mut self, value: impl Into<String>) -> Self {
        self.js_exe = Some(value.into());
        self
    }

    pub fn conditional_check(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.conditional_check = Some(value);
        self
    }

    pub fn click_button_by_value(
        mut self,
        value: WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue,
    ) -> Self {
        self.click_button_by_value = Some(value);
        self
    }

    pub fn general_image_captcha(
        mut self,
        value: Vec<WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaItem>,
    ) -> Self {
        self.general_image_captcha = Some(value);
        self
    }

    pub fn block_element(mut self, value: Vec<String>) -> Self {
        self.block_element = Some(value);
        self
    }

    pub fn extract(mut self, value: HashMap<String, String>) -> Self {
        self.extract = Some(value);
        self
    }

    pub fn fill_image_captcha(
        mut self,
        value: Vec<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItem>,
    ) -> Self {
        self.fill_image_captcha = Some(value);
        self
    }

    pub fn switch_to_iframe(mut self, value: impl Into<String>) -> Self {
        self.switch_to_iframe = Some(value.into());
        self
    }

    pub fn switch_to_parent_frame(mut self, value: bool) -> Self {
        self.switch_to_parent_frame = Some(value);
        self
    }

    pub fn resolve_audio_captcha(
        mut self,
        value: WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha,
    ) -> Self {
        self.resolve_audio_captcha = Some(value);
        self
    }

    pub fn screenshot(mut self, value: impl Into<String>) -> Self {
        self.screenshot = Some(value.into());
        self
    }

    pub fn saveimage(mut self, value: impl Into<String>) -> Self {
        self.saveimage = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItem`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItem, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItem {
            fill: self.fill,
            click: self.click,
            click_if_exist: self.click_if_exist,
            enter: self.enter,
            new_tab: self.new_tab,
            move_to_relative_tab: self.move_to_relative_tab,
            wait: self.wait,
            wait_for: self.wait_for,
            select: self.select,
            js_exe: self.js_exe,
            conditional_check: self.conditional_check,
            click_button_by_value: self.click_button_by_value,
            general_image_captcha: self.general_image_captcha,
            block_element: self.block_element,
            extract: self.extract,
            fill_image_captcha: self.fill_image_captcha,
            switch_to_iframe: self.switch_to_iframe,
            switch_to_parent_frame: self.switch_to_parent_frame,
            resolve_audio_captcha: self.resolve_audio_captcha,
            screenshot: self.screenshot,
            saveimage: self.saveimage,
        })
    }
}
