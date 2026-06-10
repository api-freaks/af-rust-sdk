pub use crate::prelude::*;

/// Request for web_scrape (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebScrapeRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Response format returned by the API.
    #[serde(skip_serializing)]
    pub format: Option<WebScrapeRequestFormat>,
    /// Target URL to scrape
    #[serde(skip_serializing)]
    #[serde(default)]
    pub url: String,
    /// Set to `true` to return the data in text format else `false` for data in html format with tags.
    #[serde(skip_serializing)]
    pub text: Option<bool>,
    /// Set  `true` to handle websites with JavaScript. Set `false` to handle static html websites.
    ///
    ///
    /// Default value is `true`.
    #[serde(rename = "jsEnabled")]
    #[serde(skip_serializing)]
    pub js_enabled: Option<bool>,
    /// Use proxy for requests
    #[serde(skip_serializing)]
    pub proxy: Option<WebScrapeRequestProxy>,
    /// Ignore SSL certificate errors.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    #[serde(rename = "sslIgnore")]
    #[serde(skip_serializing)]
    pub ssl_ignore: Option<bool>,
    /// Specify the browser window size in the format 'width,height' (e.g., "1920w,1080h"). Default value is the default resolutions provided by web/browser.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    #[serde(rename = "windowSize")]
    #[serde(skip_serializing)]
    pub window_size: Option<String>,
    /// Set to `true` to apply ad-blocker to the specified URL else false or ignore to not apply.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    #[serde(rename = "adBlock")]
    #[serde(skip_serializing)]
    pub ad_block: Option<bool>,
    /// if true user can provide captcha instructions in the instructions to solve image captchas.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    #[serde(skip_serializing)]
    pub captcha: Option<bool>,
    pub body: WebScrapeRequestBody,
}

impl WebScrapeRequest {
    pub fn builder() -> WebScrapeRequestBuilder {
        <WebScrapeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBuilder {
    api_key: Option<String>,
    format: Option<WebScrapeRequestFormat>,
    url: Option<String>,
    text: Option<bool>,
    js_enabled: Option<bool>,
    proxy: Option<WebScrapeRequestProxy>,
    ssl_ignore: Option<bool>,
    window_size: Option<String>,
    ad_block: Option<bool>,
    captcha: Option<bool>,
    body: Option<WebScrapeRequestBody>,
}

impl WebScrapeRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: WebScrapeRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn text(mut self, value: bool) -> Self {
        self.text = Some(value);
        self
    }

    pub fn js_enabled(mut self, value: bool) -> Self {
        self.js_enabled = Some(value);
        self
    }

    pub fn proxy(mut self, value: WebScrapeRequestProxy) -> Self {
        self.proxy = Some(value);
        self
    }

    pub fn ssl_ignore(mut self, value: bool) -> Self {
        self.ssl_ignore = Some(value);
        self
    }

    pub fn window_size(mut self, value: impl Into<String>) -> Self {
        self.window_size = Some(value.into());
        self
    }

    pub fn ad_block(mut self, value: bool) -> Self {
        self.ad_block = Some(value);
        self
    }

    pub fn captcha(mut self, value: bool) -> Self {
        self.captcha = Some(value);
        self
    }

    pub fn body(mut self, value: WebScrapeRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](WebScrapeRequestBuilder::api_key)
    /// - [`url`](WebScrapeRequestBuilder::url)
    /// - [`body`](WebScrapeRequestBuilder::body)
    pub fn build(self) -> Result<WebScrapeRequest, BuildError> {
        Ok(WebScrapeRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            text: self.text,
            js_enabled: self.js_enabled,
            proxy: self.proxy,
            ssl_ignore: self.ssl_ignore,
            window_size: self.window_size,
            ad_block: self.ad_block,
            captcha: self.captcha,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
