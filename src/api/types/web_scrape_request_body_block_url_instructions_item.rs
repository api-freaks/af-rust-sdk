pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WebScrapeRequestBodyBlockUrlInstructionsItem {
    WebScrapeRequestBodyBlockUrlInstructionsItemFill(
        WebScrapeRequestBodyBlockUrlInstructionsItemFill,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemClick(
        WebScrapeRequestBodyBlockUrlInstructionsItemClick,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist(
        WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemEnter(
        WebScrapeRequestBodyBlockUrlInstructionsItemEnter,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemNewTab(
        WebScrapeRequestBodyBlockUrlInstructionsItemNewTab,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab(
        WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemWait(
        WebScrapeRequestBodyBlockUrlInstructionsItemWait,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor(
        WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemSelect(
        WebScrapeRequestBodyBlockUrlInstructionsItemSelect,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemJsExe(
        WebScrapeRequestBodyBlockUrlInstructionsItemJsExe,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck(
        WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue(
        WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha(
        WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement(
        WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemExtract(
        WebScrapeRequestBodyBlockUrlInstructionsItemExtract,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha(
        WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe(
        WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame(
        WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha(
        WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot(
        WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot,
    ),

    WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage(
        WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage,
    ),
}

impl WebScrapeRequestBodyBlockUrlInstructionsItem {
    pub fn is_web_scrape_request_body_block_url_instructions_item_fill(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFill(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_click(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClick(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_click_if_exist(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_enter(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemEnter(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_new_tab(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemNewTab(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_move_to_relative_tab(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_wait(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWait(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_wait_for(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_select(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSelect(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_js_exe(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemJsExe(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_conditional_check(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_click_button_by_value(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_general_image_captcha(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_block_element(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_extract(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemExtract(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_fill_image_captcha(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_switch_to_iframe(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_switch_to_parent_frame(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_resolve_audio_captcha(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_screenshot(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot(_)
        )
    }

    pub fn is_web_scrape_request_body_block_url_instructions_item_saveimage(&self) -> bool {
        matches!(
            self,
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage(_)
        )
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_fill(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemFill> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFill(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_fill(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemFill> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFill(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_click(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemClick> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClick(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_click(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemClick> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClick(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_click_if_exist(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_click_if_exist(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_enter(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemEnter> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemEnter(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_enter(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemEnter> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemEnter(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_new_tab(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemNewTab> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemNewTab(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_new_tab(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemNewTab> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemNewTab(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_move_to_relative_tab(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_move_to_relative_tab(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_wait(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemWait> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWait(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_wait(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemWait> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWait(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_wait_for(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_wait_for(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_select(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemSelect> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSelect(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_select(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemSelect> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSelect(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_js_exe(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemJsExe> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemJsExe(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_js_exe(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemJsExe> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemJsExe(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_conditional_check(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_conditional_check(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_click_button_by_value(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_click_button_by_value(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_general_image_captcha(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_general_image_captcha(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_block_element(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_block_element(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_extract(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemExtract> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemExtract(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_extract(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemExtract> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemExtract(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_fill_image_captcha(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_fill_image_captcha(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_switch_to_iframe(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_switch_to_iframe(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_switch_to_parent_frame(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_switch_to_parent_frame(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_resolve_audio_captcha(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_resolve_audio_captcha(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_screenshot(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_screenshot(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_web_scrape_request_body_block_url_instructions_item_saveimage(
        &self,
    ) -> Option<&WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_web_scrape_request_body_block_url_instructions_item_saveimage(
        self,
    ) -> Option<WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage> {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for WebScrapeRequestBodyBlockUrlInstructionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFill(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClick(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickIfExist(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemEnter(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemNewTab(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWait(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemWaitFor(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSelect(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemJsExe(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptcha(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemExtract(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptcha(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToIframe(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSwitchToParentFrame(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemResolveAudioCaptcha(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WebScrapeRequestBodyBlockUrlInstructionsItemSaveimage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
