pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ZipcodeSearchByRegionResponse {
    /// Total number of ZIP/postal codes found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i64>,
    /// Total number of pages available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Current page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
    /// Number of ZIP/postal codes in the current page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page_size: Option<i64>,
    /// List of ZIP/postal codes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<String>>,
}

impl ZipcodeSearchByRegionResponse {
    pub fn builder() -> ZipcodeSearchByRegionResponseBuilder {
        <ZipcodeSearchByRegionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeSearchByRegionResponseBuilder {
    total_results: Option<i64>,
    total_pages: Option<i64>,
    current_page: Option<i64>,
    current_page_size: Option<i64>,
    codes: Option<Vec<String>>,
}

impl ZipcodeSearchByRegionResponseBuilder {
    pub fn total_results(mut self, value: i64) -> Self {
        self.total_results = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn current_page_size(mut self, value: i64) -> Self {
        self.current_page_size = Some(value);
        self
    }

    pub fn codes(mut self, value: Vec<String>) -> Self {
        self.codes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeSearchByRegionResponse`].
    pub fn build(self) -> Result<ZipcodeSearchByRegionResponse, BuildError> {
        Ok(ZipcodeSearchByRegionResponse {
            total_results: self.total_results,
            total_pages: self.total_pages,
            current_page: self.current_page,
            current_page_size: self.current_page_size,
            codes: self.codes,
        })
    }
}
