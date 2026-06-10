pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkScreenshotCaptureResponseResultsItemUrl {
    #[serde(default)]
    pub screenshot: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub ttl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omit_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_screenshot: Option<bool>,
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
    pub no_cookie_banners: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_to_element: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_languages: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub thumbnail_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fresh: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_caching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_load: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retina: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_chat_widgets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_js: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_stylesheets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_font: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_text_track: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_xhr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_fetch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_event_source: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_web_socket: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_manifest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_specific_requests: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub adjust_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub image_quality: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_html: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub grayscale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_incognito: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrolling_screenshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_scrolling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_back: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_immediately: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl BulkScreenshotCaptureResponseResultsItemUrl {
    pub fn builder() -> BulkScreenshotCaptureResponseResultsItemUrlBuilder {
        <BulkScreenshotCaptureResponseResultsItemUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureResponseResultsItemUrlBuilder {
    screenshot: Option<String>,
    url: Option<String>,
    created_at: Option<String>,
    format: Option<String>,
    ttl: Option<String>,
    file_type: Option<String>,
    extracted_html: Option<String>,
    omit_background: Option<bool>,
    destroy_screenshot: Option<bool>,
    fail_on_error: Option<bool>,
    longitude: Option<f64>,
    latitude: Option<f64>,
    proxy: Option<String>,
    no_cookie_banners: Option<bool>,
    block_ads: Option<bool>,
    headers: Option<String>,
    cookies: Option<String>,
    scroll_to_element: Option<String>,
    selector: Option<String>,
    blur_selector: Option<String>,
    remove_selector: Option<String>,
    css: Option<String>,
    css_url: Option<String>,
    js: Option<String>,
    js_url: Option<String>,
    user_agent: Option<String>,
    accept_languages: Option<String>,
    delay: Option<f64>,
    thumbnail_width: Option<f64>,
    output: Option<String>,
    fresh: Option<bool>,
    enable_caching: Option<bool>,
    lazy_load: Option<bool>,
    full_page: Option<bool>,
    retina: Option<bool>,
    height: Option<f64>,
    width: Option<f64>,
    custom_html: Option<String>,
    block_chat_widgets: Option<bool>,
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
    adjust_top: Option<f64>,
    image_quality: Option<f64>,
    extract_html: Option<bool>,
    extract_text: Option<bool>,
    dark_mode: Option<bool>,
    block_tracking: Option<bool>,
    wait_for_event: Option<String>,
    grayscale: Option<f64>,
    result_file_name: Option<String>,
    enable_incognito: Option<bool>,
    timeout: Option<f64>,
    scrolling_screenshot: Option<bool>,
    multiple_scrolling: Option<bool>,
    scroll_speed: Option<String>,
    duration: Option<f64>,
    scroll_back: Option<bool>,
    start_immediately: Option<bool>,
    clip: Option<HashMap<String, serde_json::Value>>,
    sizes: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl BulkScreenshotCaptureResponseResultsItemUrlBuilder {
    pub fn screenshot(mut self, value: impl Into<String>) -> Self {
        self.screenshot = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn ttl(mut self, value: impl Into<String>) -> Self {
        self.ttl = Some(value.into());
        self
    }

    pub fn file_type(mut self, value: impl Into<String>) -> Self {
        self.file_type = Some(value.into());
        self
    }

    pub fn extracted_html(mut self, value: impl Into<String>) -> Self {
        self.extracted_html = Some(value.into());
        self
    }

    pub fn omit_background(mut self, value: bool) -> Self {
        self.omit_background = Some(value);
        self
    }

    pub fn destroy_screenshot(mut self, value: bool) -> Self {
        self.destroy_screenshot = Some(value);
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

    pub fn no_cookie_banners(mut self, value: bool) -> Self {
        self.no_cookie_banners = Some(value);
        self
    }

    pub fn block_ads(mut self, value: bool) -> Self {
        self.block_ads = Some(value);
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

    pub fn blur_selector(mut self, value: impl Into<String>) -> Self {
        self.blur_selector = Some(value.into());
        self
    }

    pub fn remove_selector(mut self, value: impl Into<String>) -> Self {
        self.remove_selector = Some(value.into());
        self
    }

    pub fn css(mut self, value: impl Into<String>) -> Self {
        self.css = Some(value.into());
        self
    }

    pub fn css_url(mut self, value: impl Into<String>) -> Self {
        self.css_url = Some(value.into());
        self
    }

    pub fn js(mut self, value: impl Into<String>) -> Self {
        self.js = Some(value.into());
        self
    }

    pub fn js_url(mut self, value: impl Into<String>) -> Self {
        self.js_url = Some(value.into());
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

    pub fn delay(mut self, value: f64) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn thumbnail_width(mut self, value: f64) -> Self {
        self.thumbnail_width = Some(value);
        self
    }

    pub fn output(mut self, value: impl Into<String>) -> Self {
        self.output = Some(value.into());
        self
    }

    pub fn fresh(mut self, value: bool) -> Self {
        self.fresh = Some(value);
        self
    }

    pub fn enable_caching(mut self, value: bool) -> Self {
        self.enable_caching = Some(value);
        self
    }

    pub fn lazy_load(mut self, value: bool) -> Self {
        self.lazy_load = Some(value);
        self
    }

    pub fn full_page(mut self, value: bool) -> Self {
        self.full_page = Some(value);
        self
    }

    pub fn retina(mut self, value: bool) -> Self {
        self.retina = Some(value);
        self
    }

    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn custom_html(mut self, value: impl Into<String>) -> Self {
        self.custom_html = Some(value.into());
        self
    }

    pub fn block_chat_widgets(mut self, value: bool) -> Self {
        self.block_chat_widgets = Some(value);
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

    pub fn adjust_top(mut self, value: f64) -> Self {
        self.adjust_top = Some(value);
        self
    }

    pub fn image_quality(mut self, value: f64) -> Self {
        self.image_quality = Some(value);
        self
    }

    pub fn extract_html(mut self, value: bool) -> Self {
        self.extract_html = Some(value);
        self
    }

    pub fn extract_text(mut self, value: bool) -> Self {
        self.extract_text = Some(value);
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

    pub fn wait_for_event(mut self, value: impl Into<String>) -> Self {
        self.wait_for_event = Some(value.into());
        self
    }

    pub fn grayscale(mut self, value: f64) -> Self {
        self.grayscale = Some(value);
        self
    }

    pub fn result_file_name(mut self, value: impl Into<String>) -> Self {
        self.result_file_name = Some(value.into());
        self
    }

    pub fn enable_incognito(mut self, value: bool) -> Self {
        self.enable_incognito = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn scrolling_screenshot(mut self, value: bool) -> Self {
        self.scrolling_screenshot = Some(value);
        self
    }

    pub fn multiple_scrolling(mut self, value: bool) -> Self {
        self.multiple_scrolling = Some(value);
        self
    }

    pub fn scroll_speed(mut self, value: impl Into<String>) -> Self {
        self.scroll_speed = Some(value.into());
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
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

    pub fn clip(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.clip = Some(value);
        self
    }

    pub fn sizes(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.sizes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureResponseResultsItemUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`screenshot`](BulkScreenshotCaptureResponseResultsItemUrlBuilder::screenshot)
    /// - [`url`](BulkScreenshotCaptureResponseResultsItemUrlBuilder::url)
    /// - [`created_at`](BulkScreenshotCaptureResponseResultsItemUrlBuilder::created_at)
    /// - [`format`](BulkScreenshotCaptureResponseResultsItemUrlBuilder::format)
    /// - [`ttl`](BulkScreenshotCaptureResponseResultsItemUrlBuilder::ttl)
    pub fn build(self) -> Result<BulkScreenshotCaptureResponseResultsItemUrl, BuildError> {
        Ok(BulkScreenshotCaptureResponseResultsItemUrl {
            screenshot: self
                .screenshot
                .ok_or_else(|| BuildError::missing_field("screenshot"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            ttl: self.ttl.ok_or_else(|| BuildError::missing_field("ttl"))?,
            file_type: self.file_type,
            extracted_html: self.extracted_html,
            omit_background: self.omit_background,
            destroy_screenshot: self.destroy_screenshot,
            fail_on_error: self.fail_on_error,
            longitude: self.longitude,
            latitude: self.latitude,
            proxy: self.proxy,
            no_cookie_banners: self.no_cookie_banners,
            block_ads: self.block_ads,
            headers: self.headers,
            cookies: self.cookies,
            scroll_to_element: self.scroll_to_element,
            selector: self.selector,
            blur_selector: self.blur_selector,
            remove_selector: self.remove_selector,
            css: self.css,
            css_url: self.css_url,
            js: self.js,
            js_url: self.js_url,
            user_agent: self.user_agent,
            accept_languages: self.accept_languages,
            delay: self.delay,
            thumbnail_width: self.thumbnail_width,
            output: self.output,
            fresh: self.fresh,
            enable_caching: self.enable_caching,
            lazy_load: self.lazy_load,
            full_page: self.full_page,
            retina: self.retina,
            height: self.height,
            width: self.width,
            custom_html: self.custom_html,
            block_chat_widgets: self.block_chat_widgets,
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
            adjust_top: self.adjust_top,
            image_quality: self.image_quality,
            extract_html: self.extract_html,
            extract_text: self.extract_text,
            dark_mode: self.dark_mode,
            block_tracking: self.block_tracking,
            wait_for_event: self.wait_for_event,
            grayscale: self.grayscale,
            result_file_name: self.result_file_name,
            enable_incognito: self.enable_incognito,
            timeout: self.timeout,
            scrolling_screenshot: self.scrolling_screenshot,
            multiple_scrolling: self.multiple_scrolling,
            scroll_speed: self.scroll_speed,
            duration: self.duration,
            scroll_back: self.scroll_back,
            start_immediately: self.start_immediately,
            clip: self.clip,
            sizes: self.sizes,
            extra: Default::default(),
        })
    }
}
