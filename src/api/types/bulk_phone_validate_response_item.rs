pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkPhoneValidateResponseItem {
    /// The original request data provided by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_input: Option<BulkPhoneValidateResponseItemRawInput>,
    /// Whether the number passes length and format checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible: Option<bool>,
    /// Whether the number is valid according to the numbering plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    /// International dialing prefix for the number's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_prefix: Option<i64>,
    /// National significant number without the country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_number: Option<i64>,
    /// ISO-2 country code inferred from the number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Carrier name associated with the number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// Geographic description (city/region) for the number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Array of possible IANA time zones associated with the number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zones: Option<Vec<String>>,
    /// Classification of the phone line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_type: Option<BulkPhoneValidateResponseItemLineType>,
    /// The number represented in four standardized formats. Only returned for valid numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<BulkPhoneValidateResponseItemFormats>,
    /// Length of the geographic area code. Only for geographically-assigned numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code_length: Option<i64>,
    /// Length of the National Destination Code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndc_length: Option<i64>,
    /// Whether the number can be dialled internationally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_internationally_dialled: Option<bool>,
}

impl BulkPhoneValidateResponseItem {
    pub fn builder() -> BulkPhoneValidateResponseItemBuilder {
        <BulkPhoneValidateResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkPhoneValidateResponseItemBuilder {
    raw_input: Option<BulkPhoneValidateResponseItemRawInput>,
    possible: Option<bool>,
    valid: Option<bool>,
    country_prefix: Option<i64>,
    national_number: Option<i64>,
    country_code: Option<String>,
    carrier: Option<String>,
    location: Option<String>,
    time_zones: Option<Vec<String>>,
    line_type: Option<BulkPhoneValidateResponseItemLineType>,
    formats: Option<BulkPhoneValidateResponseItemFormats>,
    area_code_length: Option<i64>,
    ndc_length: Option<i64>,
    can_be_internationally_dialled: Option<bool>,
}

impl BulkPhoneValidateResponseItemBuilder {
    pub fn raw_input(mut self, value: BulkPhoneValidateResponseItemRawInput) -> Self {
        self.raw_input = Some(value);
        self
    }

    pub fn possible(mut self, value: bool) -> Self {
        self.possible = Some(value);
        self
    }

    pub fn valid(mut self, value: bool) -> Self {
        self.valid = Some(value);
        self
    }

    pub fn country_prefix(mut self, value: i64) -> Self {
        self.country_prefix = Some(value);
        self
    }

    pub fn national_number(mut self, value: i64) -> Self {
        self.national_number = Some(value);
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn carrier(mut self, value: impl Into<String>) -> Self {
        self.carrier = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn time_zones(mut self, value: Vec<String>) -> Self {
        self.time_zones = Some(value);
        self
    }

    pub fn line_type(mut self, value: BulkPhoneValidateResponseItemLineType) -> Self {
        self.line_type = Some(value);
        self
    }

    pub fn formats(mut self, value: BulkPhoneValidateResponseItemFormats) -> Self {
        self.formats = Some(value);
        self
    }

    pub fn area_code_length(mut self, value: i64) -> Self {
        self.area_code_length = Some(value);
        self
    }

    pub fn ndc_length(mut self, value: i64) -> Self {
        self.ndc_length = Some(value);
        self
    }

    pub fn can_be_internationally_dialled(mut self, value: bool) -> Self {
        self.can_be_internationally_dialled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkPhoneValidateResponseItem`].
    pub fn build(self) -> Result<BulkPhoneValidateResponseItem, BuildError> {
        Ok(BulkPhoneValidateResponseItem {
            raw_input: self.raw_input,
            possible: self.possible,
            valid: self.valid,
            country_prefix: self.country_prefix,
            national_number: self.national_number,
            country_code: self.country_code,
            carrier: self.carrier,
            location: self.location,
            time_zones: self.time_zones,
            line_type: self.line_type,
            formats: self.formats,
            area_code_length: self.area_code_length,
            ndc_length: self.ndc_length,
            can_be_internationally_dialled: self.can_be_internationally_dialled,
        })
    }
}
