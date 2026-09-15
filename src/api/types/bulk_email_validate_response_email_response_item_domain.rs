pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkEmailValidateResponseEmailResponseItemDomain {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub disposable: bool,
    #[serde(default)]
    pub spam: bool,
    #[serde(default)]
    pub free: bool,
    #[serde(rename = "catchAll")]
    #[serde(default)]
    pub catch_all: bool,
    #[serde(rename = "validDomain")]
    #[serde(default)]
    pub valid_domain: bool,
}

impl BulkEmailValidateResponseEmailResponseItemDomain {
    pub fn builder() -> BulkEmailValidateResponseEmailResponseItemDomainBuilder {
        <BulkEmailValidateResponseEmailResponseItemDomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailResponseItemDomainBuilder {
    name: Option<String>,
    disposable: Option<bool>,
    spam: Option<bool>,
    free: Option<bool>,
    catch_all: Option<bool>,
    valid_domain: Option<bool>,
}

impl BulkEmailValidateResponseEmailResponseItemDomainBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn disposable(mut self, value: bool) -> Self {
        self.disposable = Some(value);
        self
    }

    pub fn spam(mut self, value: bool) -> Self {
        self.spam = Some(value);
        self
    }

    pub fn free(mut self, value: bool) -> Self {
        self.free = Some(value);
        self
    }

    pub fn catch_all(mut self, value: bool) -> Self {
        self.catch_all = Some(value);
        self
    }

    pub fn valid_domain(mut self, value: bool) -> Self {
        self.valid_domain = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailResponseItemDomain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BulkEmailValidateResponseEmailResponseItemDomainBuilder::name)
    /// - [`disposable`](BulkEmailValidateResponseEmailResponseItemDomainBuilder::disposable)
    /// - [`spam`](BulkEmailValidateResponseEmailResponseItemDomainBuilder::spam)
    /// - [`free`](BulkEmailValidateResponseEmailResponseItemDomainBuilder::free)
    /// - [`catch_all`](BulkEmailValidateResponseEmailResponseItemDomainBuilder::catch_all)
    /// - [`valid_domain`](BulkEmailValidateResponseEmailResponseItemDomainBuilder::valid_domain)
    pub fn build(self) -> Result<BulkEmailValidateResponseEmailResponseItemDomain, BuildError> {
        Ok(BulkEmailValidateResponseEmailResponseItemDomain {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            disposable: self
                .disposable
                .ok_or_else(|| BuildError::missing_field("disposable"))?,
            spam: self.spam.ok_or_else(|| BuildError::missing_field("spam"))?,
            free: self.free.ok_or_else(|| BuildError::missing_field("free"))?,
            catch_all: self
                .catch_all
                .ok_or_else(|| BuildError::missing_field("catch_all"))?,
            valid_domain: self
                .valid_domain
                .ok_or_else(|| BuildError::missing_field("valid_domain"))?,
        })
    }
}
