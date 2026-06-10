pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseParsedWhoisResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub whois_server: String,
    #[serde(default)]
    pub aut_nums: Vec<AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem>,
    #[serde(default)]
    pub organization: AsnWhoisLookupResponseParsedWhoisResponseOrganization,
    #[serde(default)]
    pub technical_contacts: Vec<AsnWhoisLookupResponseParsedWhoisResponseTechnicalContactsItem>,
    #[serde(default)]
    pub abuse_contacts: Vec<AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem>,
}

impl AsnWhoisLookupResponseParsedWhoisResponse {
    pub fn builder() -> AsnWhoisLookupResponseParsedWhoisResponseBuilder {
        <AsnWhoisLookupResponseParsedWhoisResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseParsedWhoisResponseBuilder {
    status: Option<bool>,
    whois_server: Option<String>,
    aut_nums: Option<Vec<AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem>>,
    organization: Option<AsnWhoisLookupResponseParsedWhoisResponseOrganization>,
    technical_contacts: Option<Vec<AsnWhoisLookupResponseParsedWhoisResponseTechnicalContactsItem>>,
    abuse_contacts: Option<Vec<AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem>>,
}

impl AsnWhoisLookupResponseParsedWhoisResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn whois_server(mut self, value: impl Into<String>) -> Self {
        self.whois_server = Some(value.into());
        self
    }

    pub fn aut_nums(
        mut self,
        value: Vec<AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem>,
    ) -> Self {
        self.aut_nums = Some(value);
        self
    }

    pub fn organization(
        mut self,
        value: AsnWhoisLookupResponseParsedWhoisResponseOrganization,
    ) -> Self {
        self.organization = Some(value);
        self
    }

    pub fn technical_contacts(
        mut self,
        value: Vec<AsnWhoisLookupResponseParsedWhoisResponseTechnicalContactsItem>,
    ) -> Self {
        self.technical_contacts = Some(value);
        self
    }

    pub fn abuse_contacts(
        mut self,
        value: Vec<AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem>,
    ) -> Self {
        self.abuse_contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseParsedWhoisResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AsnWhoisLookupResponseParsedWhoisResponseBuilder::status)
    /// - [`whois_server`](AsnWhoisLookupResponseParsedWhoisResponseBuilder::whois_server)
    /// - [`aut_nums`](AsnWhoisLookupResponseParsedWhoisResponseBuilder::aut_nums)
    /// - [`organization`](AsnWhoisLookupResponseParsedWhoisResponseBuilder::organization)
    /// - [`technical_contacts`](AsnWhoisLookupResponseParsedWhoisResponseBuilder::technical_contacts)
    /// - [`abuse_contacts`](AsnWhoisLookupResponseParsedWhoisResponseBuilder::abuse_contacts)
    pub fn build(self) -> Result<AsnWhoisLookupResponseParsedWhoisResponse, BuildError> {
        Ok(AsnWhoisLookupResponseParsedWhoisResponse {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            whois_server: self
                .whois_server
                .ok_or_else(|| BuildError::missing_field("whois_server"))?,
            aut_nums: self
                .aut_nums
                .ok_or_else(|| BuildError::missing_field("aut_nums"))?,
            organization: self
                .organization
                .ok_or_else(|| BuildError::missing_field("organization"))?,
            technical_contacts: self
                .technical_contacts
                .ok_or_else(|| BuildError::missing_field("technical_contacts"))?,
            abuse_contacts: self
                .abuse_contacts
                .ok_or_else(|| BuildError::missing_field("abuse_contacts"))?,
        })
    }
}
