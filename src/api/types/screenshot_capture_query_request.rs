pub use crate::prelude::*;

/// Query parameters for screenshot_capture
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScreenshotCaptureQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Output format for screenshot results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ScreenshotCaptureRequestOutput>,
    /// File type for screenshot output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<ScreenshotCaptureRequestFileType>,
    /// URLs to capture screenshots of
    #[serde(default)]
    pub url: String,
    /// Browser viewport width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Browser viewport height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Capture a full-page screenshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// Bypass cache and take a fresh screenshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fresh: Option<bool>,
    /// Remove cookie banners from the screenshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_cookie_banners: Option<bool>,
    /// Enable caching for repeated requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_caching: Option<bool>,
    /// Block advertisements on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Block chat widget scripts from loading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_chat_widgets: Option<bool>,
    /// Extract visible text from the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_text: Option<bool>,
    /// Extract HTML content of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_html: Option<bool>,
    /// Auto-destroy screenshot after fetch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_screenshot: Option<bool>,
    /// Enable lazy-loading content before screenshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_load: Option<bool>,
    /// Capture screenshot in high-DPI (Retina) mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retina: Option<bool>,
    /// Render page in dark mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    /// Block common user-tracking scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_tracking: Option<bool>,
    /// Enable private/incognito mode for browser session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_incognito: Option<bool>,
    /// Omit background color (transparent background)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omit_background: Option<bool>,
    /// Thumbnail width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Adjust top in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_top: Option<i64>,
    /// Wait for a specific load event before capturing the screenshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_event: Option<ScreenshotCaptureRequestWaitForEvent>,
    /// Range:0 to 100 for grayscale filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscale: Option<i64>,
    /// How many milliseconds to wait before taking the screenshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// Maximum timeout in milliseconds. Defalut is `10,000`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Number of seconds the screenshot should be cached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// X position of the clipping rectangle in pixels
    #[serde(rename = "clip[x]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_x: Option<i64>,
    /// Y position of the clipping rectangle in pixels
    #[serde(rename = "clip[y]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_y: Option<i64>,
    /// Width of the clipping rectangle in pixels
    #[serde(rename = "clip[width]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_width: Option<i64>,
    /// Height of the clipping rectangle in pixels
    #[serde(rename = "clip[height]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_height: Option<i64>,
    /// URL to CSS file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_url: Option<String>,
    /// Your custom CSS code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    /// URL to JS file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js_url: Option<String>,
    /// Your JS code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<String>,
    /// Block Scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_js: Option<bool>,
    /// Block Stylesheets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_stylesheets: Option<bool>,
    /// Block Images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_images: Option<bool>,
    /// Block Media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_media: Option<bool>,
    /// Block Fonts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_font: Option<bool>,
    /// Block Text Tracks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_text_track: Option<bool>,
    /// Block XHR Requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_xhr: Option<bool>,
    /// Block Fetch Requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_fetch: Option<bool>,
    /// Block Event Source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_event_source: Option<bool>,
    /// Block Web Sockets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_web_socket: Option<bool>,
    /// Block Manifest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_manifest: Option<bool>,
    /// Comma- or newline-separated list of specific requests to block. Each line and comma are treated as separate requests for processing. Example: https://example.com, https://example.js
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_specific_requests: Option<String>,
    /// Comma-separated list of indexed CSS selectors to blur.
    /// Format: `index:<selector>`, e.g., `0:.banner,1:#ads`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_selector: Option<String>,
    /// Comma-separated list of indexed CSS selectors to blur.
    /// Format: `index:<selector>`, e.g., `0:.banner,1:#ads`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_selector: Option<String>,
    /// Specify a meaningful & unique file name to easily identify the screenshot result.
    /// Avoid using spaces or special characters; use hyphens or underscores to separate words.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_file_name: Option<String>,
    /// **`Scrolling Screenshot`**: Capture a long scrolling screenshot. When true, disable `fullPage` and `freshScreenshot`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrolling_screenshot: Option<bool>,
    /// Speed of scrolling during the screenshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<ScreenshotCaptureRequestScrollSpeed>,
    /// If true, the scroll will reverse back to the top after reaching the bottom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_back: Option<bool>,
    /// If true, the scrolling capture will start immediately upon page load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_immediately: Option<bool>,
    /// If true, multiple scrolling screenshots will be taken at different viewport sizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_scrolling: Option<bool>,
    /// Comma-separated list of viewport sizes in the format index:XXw:YYh. Example: sizes=0:120w:300h,1:240w:500h
    #[serde(default)]
    pub sizes: Vec<Option<String>>,
    /// Duration in seconds for the scrolling capture. Acceptable range: 0 to 100 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_to_element: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_languages: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub image_quality: Option<f64>,
}

impl ScreenshotCaptureQueryRequest {
    pub fn builder() -> ScreenshotCaptureQueryRequestBuilder {
        <ScreenshotCaptureQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScreenshotCaptureQueryRequestBuilder {
    api_key: Option<String>,
    output: Option<ScreenshotCaptureRequestOutput>,
    file_type: Option<ScreenshotCaptureRequestFileType>,
    url: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    full_page: Option<bool>,
    fresh: Option<bool>,
    no_cookie_banners: Option<bool>,
    enable_caching: Option<bool>,
    block_ads: Option<bool>,
    block_chat_widgets: Option<bool>,
    extract_text: Option<bool>,
    extract_html: Option<bool>,
    destroy_screenshot: Option<bool>,
    lazy_load: Option<bool>,
    retina: Option<bool>,
    dark_mode: Option<bool>,
    block_tracking: Option<bool>,
    enable_incognito: Option<bool>,
    omit_background: Option<bool>,
    thumbnail_width: Option<i64>,
    adjust_top: Option<i64>,
    wait_for_event: Option<ScreenshotCaptureRequestWaitForEvent>,
    grayscale: Option<i64>,
    delay: Option<i64>,
    timeout: Option<i64>,
    ttl: Option<i64>,
    clip_x: Option<i64>,
    clip_y: Option<i64>,
    clip_width: Option<i64>,
    clip_height: Option<i64>,
    css_url: Option<String>,
    css: Option<String>,
    js_url: Option<String>,
    js: Option<String>,
    block_js: Option<bool>,
    block_stylesheets: Option<bool>,
    block_images: Option<bool>,
    block_media: Option<bool>,
    block_font: Option<bool>,
    block_text_track: Option<bool>,
    block_xhr: Option<bool>,
    block_fetch: Option<bool>,
    block_event_source: Option<bool>,
    block_web_socket: Option<bool>,
    block_manifest: Option<bool>,
    block_specific_requests: Option<String>,
    blur_selector: Option<String>,
    remove_selector: Option<String>,
    result_file_name: Option<String>,
    scrolling_screenshot: Option<bool>,
    scroll_speed: Option<ScreenshotCaptureRequestScrollSpeed>,
    scroll_back: Option<bool>,
    start_immediately: Option<bool>,
    multiple_scrolling: Option<bool>,
    sizes: Option<Vec<Option<String>>>,
    duration: Option<f64>,
    fail_on_error: Option<bool>,
    longitude: Option<f64>,
    latitude: Option<f64>,
    proxy: Option<String>,
    headers: Option<String>,
    cookies: Option<String>,
    scroll_to_element: Option<String>,
    selector: Option<String>,
    user_agent: Option<String>,
    accept_languages: Option<String>,
    custom_html: Option<String>,
    image_quality: Option<f64>,
}

impl ScreenshotCaptureQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn output(mut self, value: ScreenshotCaptureRequestOutput) -> Self {
        self.output = Some(value);
        self
    }

    pub fn file_type(mut self, value: ScreenshotCaptureRequestFileType) -> Self {
        self.file_type = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn full_page(mut self, value: bool) -> Self {
        self.full_page = Some(value);
        self
    }

    pub fn fresh(mut self, value: bool) -> Self {
        self.fresh = Some(value);
        self
    }

    pub fn no_cookie_banners(mut self, value: bool) -> Self {
        self.no_cookie_banners = Some(value);
        self
    }

    pub fn enable_caching(mut self, value: bool) -> Self {
        self.enable_caching = Some(value);
        self
    }

    pub fn block_ads(mut self, value: bool) -> Self {
        self.block_ads = Some(value);
        self
    }

    pub fn block_chat_widgets(mut self, value: bool) -> Self {
        self.block_chat_widgets = Some(value);
        self
    }

    pub fn extract_text(mut self, value: bool) -> Self {
        self.extract_text = Some(value);
        self
    }

    pub fn extract_html(mut self, value: bool) -> Self {
        self.extract_html = Some(value);
        self
    }

    pub fn destroy_screenshot(mut self, value: bool) -> Self {
        self.destroy_screenshot = Some(value);
        self
    }

    pub fn lazy_load(mut self, value: bool) -> Self {
        self.lazy_load = Some(value);
        self
    }

    pub fn retina(mut self, value: bool) -> Self {
        self.retina = Some(value);
        self
    }

    pub fn dark_mode(mut self, value: bool) -> Self {
        self.dark_mode = Some(value);
        self
    }

    pub fn block_tracking(mut self, value: bool) -> Self {
        self.block_tracking = Some(value);
        self
    }

    pub fn enable_incognito(mut self, value: bool) -> Self {
        self.enable_incognito = Some(value);
        self
    }

    pub fn omit_background(mut self, value: bool) -> Self {
        self.omit_background = Some(value);
        self
    }

    pub fn thumbnail_width(mut self, value: i64) -> Self {
        self.thumbnail_width = Some(value);
        self
    }

    pub fn adjust_top(mut self, value: i64) -> Self {
        self.adjust_top = Some(value);
        self
    }

    pub fn wait_for_event(mut self, value: ScreenshotCaptureRequestWaitForEvent) -> Self {
        self.wait_for_event = Some(value);
        self
    }

    pub fn grayscale(mut self, value: i64) -> Self {
        self.grayscale = Some(value);
        self
    }

    pub fn delay(mut self, value: i64) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }

    pub fn clip_x(mut self, value: i64) -> Self {
        self.clip_x = Some(value);
        self
    }

    pub fn clip_y(mut self, value: i64) -> Self {
        self.clip_y = Some(value);
        self
    }

    pub fn clip_width(mut self, value: i64) -> Self {
        self.clip_width = Some(value);
        self
    }

    pub fn clip_height(mut self, value: i64) -> Self {
        self.clip_height = Some(value);
        self
    }

    pub fn css_url(mut self, value: impl Into<String>) -> Self {
        self.css_url = Some(value.into());
        self
    }

    pub fn css(mut self, value: impl Into<String>) -> Self {
        self.css = Some(value.into());
        self
    }

    pub fn js_url(mut self, value: impl Into<String>) -> Self {
        self.js_url = Some(value.into());
        self
    }

    pub fn js(mut self, value: impl Into<String>) -> Self {
        self.js = Some(value.into());
        self
    }

    pub fn block_js(mut self, value: bool) -> Self {
        self.block_js = Some(value);
        self
    }

    pub fn block_stylesheets(mut self, value: bool) -> Self {
        self.block_stylesheets = Some(value);
        self
    }

    pub fn block_images(mut self, value: bool) -> Self {
        self.block_images = Some(value);
        self
    }

    pub fn block_media(mut self, value: bool) -> Self {
        self.block_media = Some(value);
        self
    }

    pub fn block_font(mut self, value: bool) -> Self {
        self.block_font = Some(value);
        self
    }

    pub fn block_text_track(mut self, value: bool) -> Self {
        self.block_text_track = Some(value);
        self
    }

    pub fn block_xhr(mut self, value: bool) -> Self {
        self.block_xhr = Some(value);
        self
    }

    pub fn block_fetch(mut self, value: bool) -> Self {
        self.block_fetch = Some(value);
        self
    }

    pub fn block_event_source(mut self, value: bool) -> Self {
        self.block_event_source = Some(value);
        self
    }

    pub fn block_web_socket(mut self, value: bool) -> Self {
        self.block_web_socket = Some(value);
        self
    }

    pub fn block_manifest(mut self, value: bool) -> Self {
        self.block_manifest = Some(value);
        self
    }

    pub fn block_specific_requests(mut self, value: impl Into<String>) -> Self {
        self.block_specific_requests = Some(value.into());
        self
    }

    pub fn blur_selector(mut self, value: impl Into<String>) -> Self {
        self.blur_selector = Some(value.into());
        self
    }

    pub fn remove_selector(mut self, value: impl Into<String>) -> Self {
        self.remove_selector = Some(value.into());
        self
    }

    pub fn result_file_name(mut self, value: impl Into<String>) -> Self {
        self.result_file_name = Some(value.into());
        self
    }

    pub fn scrolling_screenshot(mut self, value: bool) -> Self {
        self.scrolling_screenshot = Some(value);
        self
    }

    pub fn scroll_speed(mut self, value: ScreenshotCaptureRequestScrollSpeed) -> Self {
        self.scroll_speed = Some(value);
        self
    }

    pub fn scroll_back(mut self, value: bool) -> Self {
        self.scroll_back = Some(value);
        self
    }

    pub fn start_immediately(mut self, value: bool) -> Self {
        self.start_immediately = Some(value);
        self
    }

    pub fn multiple_scrolling(mut self, value: bool) -> Self {
        self.multiple_scrolling = Some(value);
        self
    }

    pub fn sizes(mut self, value: Vec<Option<String>>) -> Self {
        self.sizes = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn fail_on_error(mut self, value: bool) -> Self {
        self.fail_on_error = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn proxy(mut self, value: impl Into<String>) -> Self {
        self.proxy = Some(value.into());
        self
    }

    pub fn headers(mut self, value: impl Into<String>) -> Self {
        self.headers = Some(value.into());
        self
    }

    pub fn cookies(mut self, value: impl Into<String>) -> Self {
        self.cookies = Some(value.into());
        self
    }

    pub fn scroll_to_element(mut self, value: impl Into<String>) -> Self {
        self.scroll_to_element = Some(value.into());
        self
    }

    pub fn selector(mut self, value: impl Into<String>) -> Self {
        self.selector = Some(value.into());
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    pub fn accept_languages(mut self, value: impl Into<String>) -> Self {
        self.accept_languages = Some(value.into());
        self
    }

    pub fn custom_html(mut self, value: impl Into<String>) -> Self {
        self.custom_html = Some(value.into());
        self
    }

    pub fn image_quality(mut self, value: f64) -> Self {
        self.image_quality = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScreenshotCaptureQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](ScreenshotCaptureQueryRequestBuilder::api_key)
    /// - [`url`](ScreenshotCaptureQueryRequestBuilder::url)
    /// - [`sizes`](ScreenshotCaptureQueryRequestBuilder::sizes)
    pub fn build(self) -> Result<ScreenshotCaptureQueryRequest, BuildError> {
        Ok(ScreenshotCaptureQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            output: self.output,
            file_type: self.file_type,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            width: self.width,
            height: self.height,
            full_page: self.full_page,
            fresh: self.fresh,
            no_cookie_banners: self.no_cookie_banners,
            enable_caching: self.enable_caching,
            block_ads: self.block_ads,
            block_chat_widgets: self.block_chat_widgets,
            extract_text: self.extract_text,
            extract_html: self.extract_html,
            destroy_screenshot: self.destroy_screenshot,
            lazy_load: self.lazy_load,
            retina: self.retina,
            dark_mode: self.dark_mode,
            block_tracking: self.block_tracking,
            enable_incognito: self.enable_incognito,
            omit_background: self.omit_background,
            thumbnail_width: self.thumbnail_width,
            adjust_top: self.adjust_top,
            wait_for_event: self.wait_for_event,
            grayscale: self.grayscale,
            delay: self.delay,
            timeout: self.timeout,
            ttl: self.ttl,
            clip_x: self.clip_x,
            clip_y: self.clip_y,
            clip_width: self.clip_width,
            clip_height: self.clip_height,
            css_url: self.css_url,
            css: self.css,
            js_url: self.js_url,
            js: self.js,
            block_js: self.block_js,
            block_stylesheets: self.block_stylesheets,
            block_images: self.block_images,
            block_media: self.block_media,
            block_font: self.block_font,
            block_text_track: self.block_text_track,
            block_xhr: self.block_xhr,
            block_fetch: self.block_fetch,
            block_event_source: self.block_event_source,
            block_web_socket: self.block_web_socket,
            block_manifest: self.block_manifest,
            block_specific_requests: self.block_specific_requests,
            blur_selector: self.blur_selector,
            remove_selector: self.remove_selector,
            result_file_name: self.result_file_name,
            scrolling_screenshot: self.scrolling_screenshot,
            scroll_speed: self.scroll_speed,
            scroll_back: self.scroll_back,
            start_immediately: self.start_immediately,
            multiple_scrolling: self.multiple_scrolling,
            sizes: self
                .sizes
                .ok_or_else(|| BuildError::missing_field("sizes"))?,
            duration: self.duration,
            fail_on_error: self.fail_on_error,
            longitude: self.longitude,
            latitude: self.latitude,
            proxy: self.proxy,
            headers: self.headers,
            cookies: self.cookies,
            scroll_to_element: self.scroll_to_element,
            selector: self.selector,
            user_agent: self.user_agent,
            accept_languages: self.accept_languages,
            custom_html: self.custom_html,
            image_quality: self.image_quality,
        })
    }
}
