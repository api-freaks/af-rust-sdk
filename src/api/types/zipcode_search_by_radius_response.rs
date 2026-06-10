pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeSearchByRadiusResponse {
    /// Total number of ZIP/postal codes found within the radius
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ZipcodeSearchByRadiusResponseResultsItem>>,
}

impl ZipcodeSearchByRadiusResponse {
    pub fn builder() -> ZipcodeSearchByRadiusResponseBuilder {
        <ZipcodeSearchByRadiusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeSearchByRadiusResponseBuilder {
    total_results: Option<i64>,
    total_pages: Option<i64>,
    current_page: Option<i64>,
    current_page_size: Option<i64>,
    results: Option<Vec<ZipcodeSearchByRadiusResponseResultsItem>>,
}

impl ZipcodeSearchByRadiusResponseBuilder {
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

    pub fn results(mut self, value: Vec<ZipcodeSearchByRadiusResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeSearchByRadiusResponse`].
    pub fn build(self) -> Result<ZipcodeSearchByRadiusResponse, BuildError> {
        Ok(ZipcodeSearchByRadiusResponse {
            total_results: self.total_results,
            total_pages: self.total_pages,
            current_page: self.current_page,
            current_page_size: self.current_page_size,
            results: self.results,
        })
    }
}
