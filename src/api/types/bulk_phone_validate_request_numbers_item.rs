pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkPhoneValidateRequestNumbersItem {
    /// Phone number to validate.
    #[serde(default)]
    pub number: String,
    /// Two-letter ISO country code. Required for local format numbers. Cannot be used together with dialer_region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Two-letter ISO country code of the dialing origin. Required for IDD format numbers. Cannot be used together with region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialer_region: Option<String>,
}

impl BulkPhoneValidateRequestNumbersItem {
    pub fn builder() -> BulkPhoneValidateRequestNumbersItemBuilder {
        <BulkPhoneValidateRequestNumbersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkPhoneValidateRequestNumbersItemBuilder {
    number: Option<String>,
    region: Option<String>,
    dialer_region: Option<String>,
}

impl BulkPhoneValidateRequestNumbersItemBuilder {
    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn dialer_region(mut self, value: impl Into<String>) -> Self {
        self.dialer_region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkPhoneValidateRequestNumbersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`number`](BulkPhoneValidateRequestNumbersItemBuilder::number)
    pub fn build(self) -> Result<BulkPhoneValidateRequestNumbersItem, BuildError> {
        Ok(BulkPhoneValidateRequestNumbersItem {
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            region: self.region,
            dialer_region: self.dialer_region,
        })
    }
}
