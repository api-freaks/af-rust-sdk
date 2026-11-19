pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkScreenshotCaptureRequestUrlsItem {
    #[serde(default)]
    pub url: String,
    /// Sets the accepted languages for the HTTP request. This is sent as the Accept-Language header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_languages: Option<String>,
    /// Initial vertical scroll position in pixels before capture. Use to position mid-page content in viewport. Not supported for PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_top: Option<i64>,
    /// Blocks requests to known ad networks for cleaner, distraction-free screenshots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Blocks live chat popups, support bubbles, and messaging interfaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_chat_widgets: Option<bool>,
    /// Blocks EventSource (SSE) connections - prevents real-time data streams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_event_source: Option<bool>,
    /// Blocks all Fetch API requests - stops async data loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_fetch: Option<bool>,
    /// Blocks external font files - page falls back to system defaults. Supported: woff, woff2, ttf, otf, eot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_font: Option<bool>,
    /// Blocks all image resources (jpg, png, gif, webp, svg, etc.) for faster rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_images: Option<bool>,
    /// Disables all JavaScript execution. Renders a static version of the page. Speeds up capture but may break dynamic content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_js: Option<bool>,
    /// Blocks manifest files (.json, .webmanifest) - disables PWA-related functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_manifest: Option<bool>,
    /// Blocks audio and video resources (mp3, mp4, webm, etc.) from loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_media: Option<bool>,
    /// Fine-grained blocking of specific URLs or patterns. Separate multiple rules by comma, space, or newline. Default: empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_specific_requests: Option<String>,
    /// Blocks all stylesheets - page renders as raw HTML without visual formatting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_stylesheets: Option<bool>,
    /// Blocks subtitles and caption overlays from loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_text_track: Option<bool>,
    /// Blocks tracking scripts and analytics services for cleaner, privacy-oriented renders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_tracking: Option<bool>,
    /// Blocks WebSocket connections - prevents real-time bidirectional communication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_web_socket: Option<bool>,
    /// Blocks all XMLHttpRequest (AJAX) calls - prevents dynamic API-driven content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_xhr: Option<bool>,
    /// CSS selectors to blur before capture. Masks sensitive data like emails, payments, or personal info. Multiple selectors supported. Default: empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_selector: Option<String>,
    /// CSS selector of the message shown for a wrong answer. While it is visible, the solver tries again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_error: Option<String>,
    /// CSS selector of the captcha picture. Required when solve_captcha is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_image: Option<String>,
    /// CSS selector of the box the answer is typed into. Required when solve_captcha is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_input: Option<String>,
    /// CSS selector of the control that loads a new captcha picture before the next try.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_refresh: Option<String>,
    /// Number of guesses the solver makes, from 1 to 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_retries: Option<i64>,
    /// CSS selector of the button to click after the answer is typed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_submit: Option<String>,
    /// Loads the page through a datacenter proxy. Adds 20 credits per request. Ignored when premium_proxy is true or a custom proxy is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_proxy: Option<bool>,
    /// Number of times to click the selector (when set via `selector_to_click`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_recursion: Option<i64>,
    /// Height of the clip area in pixels. Default: null.
    #[serde(rename = "clip[height]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_height: Option<i64>,
    /// Width of the clip area in pixels. Default: null.
    #[serde(rename = "clip[width]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_width: Option<i64>,
    /// X coordinate (in pixels) for the top-left corner of the clip area. Default: null.
    #[serde(rename = "clip[x]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_x: Option<i64>,
    /// Y coordinate (in pixels) for the top-left corner of the clip area. Default: null.
    #[serde(rename = "clip[y]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_y: Option<i64>,
    /// Cookies to include in the request before rendering. Supports single or multiple cookies. Example: `cookie=value; otherCookie=otherValue;`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    /// Two-letter country code, such as US or DE, for the exit address of classic_proxy or premium_proxy. Ignored when you send your own proxy. Any two-letter code is sent, including one that is not listed. Classic proxy countries: BG, GE, DE, HN, MD, UA, GB, US. Premium proxy countries: AF, AL, DZ, AO, AG, AR, AM, AW, AU, AT, AZ, BS, BH, BD, BB, BY, BE, BZ, BJ, BT, BO, BQ, BA, BW, BR, BN, BG, BF, BI, CV, KH, CM, CA, KY, CF, TD, CL, CN, CO, KM, CG, CR, CI, HR, CU, CW, CY, CZ, CD, DK, DJ, DM, DO, EC, EG, SV, GQ, EE, SZ, ET, FO, FJ, FI, FR, GF, PF, GA, GM, GE, DE, GH, GI, GR, GL, GD, GP, GU, GT, GG, GN, GW, GY, HT, HN, HK, HU, IS, IN, ID, IR, IQ, IE, IL, IT, JM, JP, JE, JO, KZ, KE, XK, KW, KG, LA, LV, LB, LS, LR, LY, LT, MO, MG, MW, MY, MV, ML, MT, MQ, MR, MU, YT, MX, MD, MN, ME, MA, MZ, MM, NA, NP, NL, NC, NZ, NI, NE, NG, MK, NO, OM, PK, PS, PA, PG, PY, PE, PH, PL, PT, PR, QA, RE, RO, RU, RW, KN, LC, VC, WS, SA, SN, RS, SC, SL, SG, SK, SI, SB, SO, ZA, KR, SS, ES, LK, SD, SR, SE, CH, SY, TW, TJ, TZ, TH, TL, TG, TO, TT, TN, TR, TM, UG, UA, AE, GB, US, UY, UZ, VE, VN, YE, ZM, ZW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Allows injecting custom CSS code into the page before rendering. Pass the CSS code directly as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    /// Allows loading external CSS into the page. Provide a URL pointing to the CSS file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_url: Option<String>,
    /// Custom HTML to render instead of loading a URL. Overrides the url option. Use for capturing specific HTML structures or dynamic content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_html: Option<String>,
    /// When set to `true`, websites that support prefers-color-scheme: dark will render in dark mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    /// Wait period in milliseconds before capture. Allows animations, API responses, and dynamic elements to finish rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// If `true`, screenshot/PDF is not stored on server and must be downloaded immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_screenshot: Option<bool>,
    /// Duration of scrolling action in seconds (0-60). Higher = longer video. Default: null (full page scroll).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Stores the render in cache for reuse on subsequent identical requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_caching: Option<bool>,
    /// Enables private/incognito browsing. Each render starts with a clean slate - no cookies, cache, or session data persisted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_incognito: Option<bool>,
    /// Returns rendered HTML as a .html file (includes JS-executed DOM). Only available with output=json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_html: Option<bool>,
    /// Extracts rendered page content as a structured .md (Markdown) file. Preserves headings, lists, links. Only with output=json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_markdown: Option<bool>,
    /// Strips all HTML and returns visible text as a .txt file. Ideal for text analysis, indexing, or AI pipelines. Only with output=json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_text: Option<bool>,
    /// If `true`, returns an error when the target page responds with 4xx or 5xx status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_error: Option<bool>,
    /// Output format. PNG (lossless, transparency), JPEG (compressed, smaller size), WebP (modern, efficient), PDF (document). For scrolling captures: MP4, GIF, WebM. Scrolling capture formats: WEBM (web-optimized), MP4 (widely compatible, default), GIF (short loops).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<BulkScreenshotCaptureRequestUrlsItemFileType>,
    /// Response format: json (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<BulkScreenshotCaptureRequestUrlsItemFormat>,
    /// If `true`, bypasses cache and captures a fresh render. If `false`, returns cached version when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fresh: Option<bool>,
    /// Captures the entire webpage from top to bottom, including scrollable content outside the viewport. Ideal for long-form pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// Extracts all image URLs from the rendered DOM and returns them in structured format. Only with output=json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_image_urls: Option<bool>,
    /// Renders the page with a GPU, including WebGL and canvas content. Adds 20 credits per request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<bool>,
    /// Applies a grayscale filter to the screenshot. Requires a value from 0 (no effect) to 100 (fully grayscale).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscale: Option<i64>,
    /// Custom HTTP headers to include as key=value pairs. Use a semicolon to separate multiple headers (e.g., `key1=value1;key2=value2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    /// Viewport height in pixels. Controls how much vertical area is visible. Max: 4320.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Compression level for JPEG output (0-100). Lower = smaller file, higher = better quality. Only applies to JPEG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_quality: Option<i64>,
    /// Allows injecting custom JavaScript code into the page before rendering. Pass the JavaScript code directly as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<String>,
    /// Allows loading external JavaScript into the page. Provide a URL pointing to the script file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js_url: Option<String>,
    /// Specify latitude for the browser Geolocation API. Provide a numeric value. Default: null (not set).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    /// Automatically scrolls the page to trigger lazy-loaded images and components. Ensures complete content capture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_load: Option<bool>,
    /// Specify longitude for the browser Geolocation API. Provide a numeric value. Default: null (not set).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    /// Markdown to render instead of loading a URL. Works like custom_html, so you can capture a README or notes without hosting a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    /// Hides or blocks cookie consent banners and popups before capture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_cookie_banners: Option<bool>,
    /// Recognition model used by the captcha solver: mini-ocr-v1 (default) or ocr-v1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_model: Option<BulkScreenshotCaptureRequestUrlsItemOcrModel>,
    /// Replaces simple white/plain backgrounds with transparency for a cut-out effect. Only works with PNG output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omit_background: Option<bool>,
    /// Set the output format. `json` returns metadata with a hosted file URL. `image` streams the raw file bytes directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<BulkScreenshotCaptureRequestUrlsItemOutput>,
    /// PDF output options. Applies when file_type is PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_options: Option<BulkScreenshotCaptureRequestUrlsItemPdfOptions>,
    /// Loads the page through a residential proxy. Adds 200 credits per request. Used instead of classic_proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_proxy: Option<bool>,
    /// Allows defining a proxy for the connection. Provide a JSON string with keys: server (include port number), username, and password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// CSS selectors to remove before capture. Deletes matched elements from DOM. Ideal for cleaning ads, popups, sidebars. Multiple selectors supported. Default: empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_selector: Option<String>,
    /// Custom name for the output file. Helps with organized storage and retrieval. Note: non-unique names may overwrite existing files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_file_name: Option<String>,
    /// Captures at 2x resolution for sharper, high-DPI output. Increases file size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retina: Option<bool>,
    /// If `true`, scrolls to bottom then back to top, creating a seamless loop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_back: Option<bool>,
    /// Delay in milliseconds between scroll steps when `lazy_load` is enabled. Use higher values for heavy pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_delay: Option<i64>,
    /// Scrolling speed: fast, normal (default), or slow. Affects render time and output pacing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<BulkScreenshotCaptureRequestUrlsItemScrollSpeed>,
    /// Scrolls the page to bring the specified CSS selector into view before capture. Ideal for below-the-fold content. Default: empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_to_element: Option<String>,
    /// Enables scrolling screenshot mode to capture a video (webm, gif, mp4) of the webpage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrolling_screenshot: Option<bool>,
    /// CSS selector to capture a specific element. Only that element is rendered. Falls back to full page if not found. Default: empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// CSS selector for an element to click before capturing the screenshot. Useful for opening dropdowns, modals, or triggering content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_to_click: Option<String>,
    /// Solves a captcha made of text drawn in a picture, then takes the screenshot. Requires captcha_image and captcha_input. Adds 20 credits per request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solve_captcha: Option<bool>,
    /// Starts recording immediately without waiting for full page load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_immediately: Option<bool>,
    /// Makes the browser look like a regular visitor for pages that block automated traffic. No extra credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<bool>,
    /// Specifies the width of the thumbnail in pixels. Default value is 0 (meaning no specific value is set, and the screenshot is returned at its original resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Maximum wait in milliseconds for page load before aborting. Adjust for slow/heavy pages. Default: 100000 (100s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// IANA timezone the browser uses while rendering, such as Asia/Karachi or Europe/London. Useful for clocks and localized dates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Time in seconds the render stays cached before auto-invalidation. Use higher values for static pages, lower for dynamic content. Default: 2592000 (30 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Custom User-Agent string sent in the request headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// Records visible viewport only for a specified duration. Use with `duration` parameter. Ideal for dashboards and animations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    /// Controls when rendering starts based on page load lifecycle. `load` (all resources), `domcontentloaded` (HTML parsed), `networkidle` (no network activity for 500ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_event: Option<BulkScreenshotCaptureRequestUrlsItemWaitForEvent>,
    /// Viewport width in pixels. Simulates device screen sizes. Max: 7680.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl BulkScreenshotCaptureRequestUrlsItem {
    pub fn builder() -> BulkScreenshotCaptureRequestUrlsItemBuilder {
        <BulkScreenshotCaptureRequestUrlsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureRequestUrlsItemBuilder {
    url: Option<String>,
    accept_languages: Option<String>,
    adjust_top: Option<i64>,
    block_ads: Option<bool>,
    block_chat_widgets: Option<bool>,
    block_event_source: Option<bool>,
    block_fetch: Option<bool>,
    block_font: Option<bool>,
    block_images: Option<bool>,
    block_js: Option<bool>,
    block_manifest: Option<bool>,
    block_media: Option<bool>,
    block_specific_requests: Option<String>,
    block_stylesheets: Option<bool>,
    block_text_track: Option<bool>,
    block_tracking: Option<bool>,
    block_web_socket: Option<bool>,
    block_xhr: Option<bool>,
    blur_selector: Option<String>,
    captcha_error: Option<String>,
    captcha_image: Option<String>,
    captcha_input: Option<String>,
    captcha_refresh: Option<String>,
    captcha_retries: Option<i64>,
    captcha_submit: Option<String>,
    classic_proxy: Option<bool>,
    click_recursion: Option<i64>,
    clip_height: Option<i64>,
    clip_width: Option<i64>,
    clip_x: Option<i64>,
    clip_y: Option<i64>,
    cookies: Option<String>,
    country: Option<String>,
    css: Option<String>,
    css_url: Option<String>,
    custom_html: Option<String>,
    dark_mode: Option<bool>,
    delay: Option<i64>,
    destroy_screenshot: Option<bool>,
    duration: Option<i64>,
    enable_caching: Option<bool>,
    enable_incognito: Option<bool>,
    extract_html: Option<bool>,
    extract_markdown: Option<bool>,
    extract_text: Option<bool>,
    fail_on_error: Option<bool>,
    file_type: Option<BulkScreenshotCaptureRequestUrlsItemFileType>,
    format: Option<BulkScreenshotCaptureRequestUrlsItemFormat>,
    fresh: Option<bool>,
    full_page: Option<bool>,
    get_image_urls: Option<bool>,
    gpu: Option<bool>,
    grayscale: Option<i64>,
    headers: Option<String>,
    height: Option<i64>,
    image_quality: Option<i64>,
    js: Option<String>,
    js_url: Option<String>,
    latitude: Option<f64>,
    lazy_load: Option<bool>,
    longitude: Option<f64>,
    markdown: Option<String>,
    no_cookie_banners: Option<bool>,
    ocr_model: Option<BulkScreenshotCaptureRequestUrlsItemOcrModel>,
    omit_background: Option<bool>,
    output: Option<BulkScreenshotCaptureRequestUrlsItemOutput>,
    pdf_options: Option<BulkScreenshotCaptureRequestUrlsItemPdfOptions>,
    premium_proxy: Option<bool>,
    proxy: Option<String>,
    remove_selector: Option<String>,
    result_file_name: Option<String>,
    retina: Option<bool>,
    scroll_back: Option<bool>,
    scroll_delay: Option<i64>,
    scroll_speed: Option<BulkScreenshotCaptureRequestUrlsItemScrollSpeed>,
    scroll_to_element: Option<String>,
    scrolling_screenshot: Option<bool>,
    selector: Option<String>,
    selector_to_click: Option<String>,
    solve_captcha: Option<bool>,
    start_immediately: Option<bool>,
    stealth: Option<bool>,
    thumbnail_width: Option<i64>,
    timeout: Option<i64>,
    timezone: Option<String>,
    ttl: Option<i64>,
    user_agent: Option<String>,
    video: Option<bool>,
    wait_for_event: Option<BulkScreenshotCaptureRequestUrlsItemWaitForEvent>,
    width: Option<i64>,
}

impl BulkScreenshotCaptureRequestUrlsItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn accept_languages(mut self, value: impl Into<String>) -> Self {
        self.accept_languages = Some(value.into());
        self
    }

    pub fn adjust_top(mut self, value: i64) -> Self {
        self.adjust_top = Some(value);
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

    pub fn block_event_source(mut self, value: bool) -> Self {
        self.block_event_source = Some(value);
        self
    }

    pub fn block_fetch(mut self, value: bool) -> Self {
        self.block_fetch = Some(value);
        self
    }

    pub fn block_font(mut self, value: bool) -> Self {
        self.block_font = Some(value);
        self
    }

    pub fn block_images(mut self, value: bool) -> Self {
        self.block_images = Some(value);
        self
    }

    pub fn block_js(mut self, value: bool) -> Self {
        self.block_js = Some(value);
        self
    }

    pub fn block_manifest(mut self, value: bool) -> Self {
        self.block_manifest = Some(value);
        self
    }

    pub fn block_media(mut self, value: bool) -> Self {
        self.block_media = Some(value);
        self
    }

    pub fn block_specific_requests(mut self, value: impl Into<String>) -> Self {
        self.block_specific_requests = Some(value.into());
        self
    }

    pub fn block_stylesheets(mut self, value: bool) -> Self {
        self.block_stylesheets = Some(value);
        self
    }

    pub fn block_text_track(mut self, value: bool) -> Self {
        self.block_text_track = Some(value);
        self
    }

    pub fn block_tracking(mut self, value: bool) -> Self {
        self.block_tracking = Some(value);
        self
    }

    pub fn block_web_socket(mut self, value: bool) -> Self {
        self.block_web_socket = Some(value);
        self
    }

    pub fn block_xhr(mut self, value: bool) -> Self {
        self.block_xhr = Some(value);
        self
    }

    pub fn blur_selector(mut self, value: impl Into<String>) -> Self {
        self.blur_selector = Some(value.into());
        self
    }

    pub fn captcha_error(mut self, value: impl Into<String>) -> Self {
        self.captcha_error = Some(value.into());
        self
    }

    pub fn captcha_image(mut self, value: impl Into<String>) -> Self {
        self.captcha_image = Some(value.into());
        self
    }

    pub fn captcha_input(mut self, value: impl Into<String>) -> Self {
        self.captcha_input = Some(value.into());
        self
    }

    pub fn captcha_refresh(mut self, value: impl Into<String>) -> Self {
        self.captcha_refresh = Some(value.into());
        self
    }

    pub fn captcha_retries(mut self, value: i64) -> Self {
        self.captcha_retries = Some(value);
        self
    }

    pub fn captcha_submit(mut self, value: impl Into<String>) -> Self {
        self.captcha_submit = Some(value.into());
        self
    }

    pub fn classic_proxy(mut self, value: bool) -> Self {
        self.classic_proxy = Some(value);
        self
    }

    pub fn click_recursion(mut self, value: i64) -> Self {
        self.click_recursion = Some(value);
        self
    }

    pub fn clip_height(mut self, value: i64) -> Self {
        self.clip_height = Some(value);
        self
    }

    pub fn clip_width(mut self, value: i64) -> Self {
        self.clip_width = Some(value);
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

    pub fn cookies(mut self, value: impl Into<String>) -> Self {
        self.cookies = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn custom_html(mut self, value: impl Into<String>) -> Self {
        self.custom_html = Some(value.into());
        self
    }

    pub fn dark_mode(mut self, value: bool) -> Self {
        self.dark_mode = Some(value);
        self
    }

    pub fn delay(mut self, value: i64) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn destroy_screenshot(mut self, value: bool) -> Self {
        self.destroy_screenshot = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn enable_caching(mut self, value: bool) -> Self {
        self.enable_caching = Some(value);
        self
    }

    pub fn enable_incognito(mut self, value: bool) -> Self {
        self.enable_incognito = Some(value);
        self
    }

    pub fn extract_html(mut self, value: bool) -> Self {
        self.extract_html = Some(value);
        self
    }

    pub fn extract_markdown(mut self, value: bool) -> Self {
        self.extract_markdown = Some(value);
        self
    }

    pub fn extract_text(mut self, value: bool) -> Self {
        self.extract_text = Some(value);
        self
    }

    pub fn fail_on_error(mut self, value: bool) -> Self {
        self.fail_on_error = Some(value);
        self
    }

    pub fn file_type(mut self, value: BulkScreenshotCaptureRequestUrlsItemFileType) -> Self {
        self.file_type = Some(value);
        self
    }

    pub fn format(mut self, value: BulkScreenshotCaptureRequestUrlsItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn fresh(mut self, value: bool) -> Self {
        self.fresh = Some(value);
        self
    }

    pub fn full_page(mut self, value: bool) -> Self {
        self.full_page = Some(value);
        self
    }

    pub fn get_image_urls(mut self, value: bool) -> Self {
        self.get_image_urls = Some(value);
        self
    }

    pub fn gpu(mut self, value: bool) -> Self {
        self.gpu = Some(value);
        self
    }

    pub fn grayscale(mut self, value: i64) -> Self {
        self.grayscale = Some(value);
        self
    }

    pub fn headers(mut self, value: impl Into<String>) -> Self {
        self.headers = Some(value.into());
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn image_quality(mut self, value: i64) -> Self {
        self.image_quality = Some(value);
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

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn lazy_load(mut self, value: bool) -> Self {
        self.lazy_load = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn markdown(mut self, value: impl Into<String>) -> Self {
        self.markdown = Some(value.into());
        self
    }

    pub fn no_cookie_banners(mut self, value: bool) -> Self {
        self.no_cookie_banners = Some(value);
        self
    }

    pub fn ocr_model(mut self, value: BulkScreenshotCaptureRequestUrlsItemOcrModel) -> Self {
        self.ocr_model = Some(value);
        self
    }

    pub fn omit_background(mut self, value: bool) -> Self {
        self.omit_background = Some(value);
        self
    }

    pub fn output(mut self, value: BulkScreenshotCaptureRequestUrlsItemOutput) -> Self {
        self.output = Some(value);
        self
    }

    pub fn pdf_options(mut self, value: BulkScreenshotCaptureRequestUrlsItemPdfOptions) -> Self {
        self.pdf_options = Some(value);
        self
    }

    pub fn premium_proxy(mut self, value: bool) -> Self {
        self.premium_proxy = Some(value);
        self
    }

    pub fn proxy(mut self, value: impl Into<String>) -> Self {
        self.proxy = Some(value.into());
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

    pub fn retina(mut self, value: bool) -> Self {
        self.retina = Some(value);
        self
    }

    pub fn scroll_back(mut self, value: bool) -> Self {
        self.scroll_back = Some(value);
        self
    }

    pub fn scroll_delay(mut self, value: i64) -> Self {
        self.scroll_delay = Some(value);
        self
    }

    pub fn scroll_speed(mut self, value: BulkScreenshotCaptureRequestUrlsItemScrollSpeed) -> Self {
        self.scroll_speed = Some(value);
        self
    }

    pub fn scroll_to_element(mut self, value: impl Into<String>) -> Self {
        self.scroll_to_element = Some(value.into());
        self
    }

    pub fn scrolling_screenshot(mut self, value: bool) -> Self {
        self.scrolling_screenshot = Some(value);
        self
    }

    pub fn selector(mut self, value: impl Into<String>) -> Self {
        self.selector = Some(value.into());
        self
    }

    pub fn selector_to_click(mut self, value: impl Into<String>) -> Self {
        self.selector_to_click = Some(value.into());
        self
    }

    pub fn solve_captcha(mut self, value: bool) -> Self {
        self.solve_captcha = Some(value);
        self
    }

    pub fn start_immediately(mut self, value: bool) -> Self {
        self.start_immediately = Some(value);
        self
    }

    pub fn stealth(mut self, value: bool) -> Self {
        self.stealth = Some(value);
        self
    }

    pub fn thumbnail_width(mut self, value: i64) -> Self {
        self.thumbnail_width = Some(value);
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    pub fn video(mut self, value: bool) -> Self {
        self.video = Some(value);
        self
    }

    pub fn wait_for_event(
        mut self,
        value: BulkScreenshotCaptureRequestUrlsItemWaitForEvent,
    ) -> Self {
        self.wait_for_event = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureRequestUrlsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](BulkScreenshotCaptureRequestUrlsItemBuilder::url)
    pub fn build(self) -> Result<BulkScreenshotCaptureRequestUrlsItem, BuildError> {
        Ok(BulkScreenshotCaptureRequestUrlsItem {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            accept_languages: self.accept_languages,
            adjust_top: self.adjust_top,
            block_ads: self.block_ads,
            block_chat_widgets: self.block_chat_widgets,
            block_event_source: self.block_event_source,
            block_fetch: self.block_fetch,
            block_font: self.block_font,
            block_images: self.block_images,
            block_js: self.block_js,
            block_manifest: self.block_manifest,
            block_media: self.block_media,
            block_specific_requests: self.block_specific_requests,
            block_stylesheets: self.block_stylesheets,
            block_text_track: self.block_text_track,
            block_tracking: self.block_tracking,
            block_web_socket: self.block_web_socket,
            block_xhr: self.block_xhr,
            blur_selector: self.blur_selector,
            captcha_error: self.captcha_error,
            captcha_image: self.captcha_image,
            captcha_input: self.captcha_input,
            captcha_refresh: self.captcha_refresh,
            captcha_retries: self.captcha_retries,
            captcha_submit: self.captcha_submit,
            classic_proxy: self.classic_proxy,
            click_recursion: self.click_recursion,
            clip_height: self.clip_height,
            clip_width: self.clip_width,
            clip_x: self.clip_x,
            clip_y: self.clip_y,
            cookies: self.cookies,
            country: self.country,
            css: self.css,
            css_url: self.css_url,
            custom_html: self.custom_html,
            dark_mode: self.dark_mode,
            delay: self.delay,
            destroy_screenshot: self.destroy_screenshot,
            duration: self.duration,
            enable_caching: self.enable_caching,
            enable_incognito: self.enable_incognito,
            extract_html: self.extract_html,
            extract_markdown: self.extract_markdown,
            extract_text: self.extract_text,
            fail_on_error: self.fail_on_error,
            file_type: self.file_type,
            format: self.format,
            fresh: self.fresh,
            full_page: self.full_page,
            get_image_urls: self.get_image_urls,
            gpu: self.gpu,
            grayscale: self.grayscale,
            headers: self.headers,
            height: self.height,
            image_quality: self.image_quality,
            js: self.js,
            js_url: self.js_url,
            latitude: self.latitude,
            lazy_load: self.lazy_load,
            longitude: self.longitude,
            markdown: self.markdown,
            no_cookie_banners: self.no_cookie_banners,
            ocr_model: self.ocr_model,
            omit_background: self.omit_background,
            output: self.output,
            pdf_options: self.pdf_options,
            premium_proxy: self.premium_proxy,
            proxy: self.proxy,
            remove_selector: self.remove_selector,
            result_file_name: self.result_file_name,
            retina: self.retina,
            scroll_back: self.scroll_back,
            scroll_delay: self.scroll_delay,
            scroll_speed: self.scroll_speed,
            scroll_to_element: self.scroll_to_element,
            scrolling_screenshot: self.scrolling_screenshot,
            selector: self.selector,
            selector_to_click: self.selector_to_click,
            solve_captcha: self.solve_captcha,
            start_immediately: self.start_immediately,
            stealth: self.stealth,
            thumbnail_width: self.thumbnail_width,
            timeout: self.timeout,
            timezone: self.timezone,
            ttl: self.ttl,
            user_agent: self.user_agent,
            video: self.video,
            wait_for_event: self.wait_for_event,
            width: self.width,
            extra: Default::default(),
        })
    }
}
