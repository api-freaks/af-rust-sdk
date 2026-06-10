pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IpWhoisLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet_nums: Option<Vec<IpWhoisLookupResponseInetNumsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irt: Option<IpWhoisLookupResponseIrt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<IpWhoisLookupResponseOrganization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_contacts: Option<Vec<IpWhoisLookupResponseTechnicalContactsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contacts: Option<Vec<IpWhoisLookupResponseAbuseContactsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_contacts: Option<Vec<IpWhoisLookupResponseAdministrativeContactsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_response: Option<String>,
}

impl IpWhoisLookupResponse {
    pub fn builder() -> IpWhoisLookupResponseBuilder {
        <IpWhoisLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseBuilder {
    status: Option<bool>,
    ip_address: Option<String>,
    query_time: Option<String>,
    whois_server: Option<String>,
    inet_nums: Option<Vec<IpWhoisLookupResponseInetNumsItem>>,
    irt: Option<IpWhoisLookupResponseIrt>,
    organization: Option<IpWhoisLookupResponseOrganization>,
    technical_contacts: Option<Vec<IpWhoisLookupResponseTechnicalContactsItem>>,
    abuse_contacts: Option<Vec<IpWhoisLookupResponseAbuseContactsItem>>,
    administrative_contacts: Option<Vec<IpWhoisLookupResponseAdministrativeContactsItem>>,
    whois_raw_response: Option<String>,
}

impl IpWhoisLookupResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn query_time(mut self, value: impl Into<String>) -> Self {
        self.query_time = Some(value.into());
        self
    }

    pub fn whois_server(mut self, value: impl Into<String>) -> Self {
        self.whois_server = Some(value.into());
        self
    }

    pub fn inet_nums(mut self, value: Vec<IpWhoisLookupResponseInetNumsItem>) -> Self {
        self.inet_nums = Some(value);
        self
    }

    pub fn irt(mut self, value: IpWhoisLookupResponseIrt) -> Self {
        self.irt = Some(value);
        self
    }

    pub fn organization(mut self, value: IpWhoisLookupResponseOrganization) -> Self {
        self.organization = Some(value);
        self
    }

    pub fn technical_contacts(
        mut self,
        value: Vec<IpWhoisLookupResponseTechnicalContactsItem>,
    ) -> Self {
        self.technical_contacts = Some(value);
        self
    }

    pub fn abuse_contacts(mut self, value: Vec<IpWhoisLookupResponseAbuseContactsItem>) -> Self {
        self.abuse_contacts = Some(value);
        self
    }

    pub fn administrative_contacts(
        mut self,
        value: Vec<IpWhoisLookupResponseAdministrativeContactsItem>,
    ) -> Self {
        self.administrative_contacts = Some(value);
        self
    }

    pub fn whois_raw_response(mut self, value: impl Into<String>) -> Self {
        self.whois_raw_response = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IpWhoisLookupResponse`].
    pub fn build(self) -> Result<IpWhoisLookupResponse, BuildError> {
        Ok(IpWhoisLookupResponse {
            status: self.status,
            ip_address: self.ip_address,
            query_time: self.query_time,
            whois_server: self.whois_server,
            inet_nums: self.inet_nums,
            irt: self.irt,
            organization: self.organization,
            technical_contacts: self.technical_contacts,
            abuse_contacts: self.abuse_contacts,
            administrative_contacts: self.administrative_contacts,
            whois_raw_response: self.whois_raw_response,
        })
    }
}
