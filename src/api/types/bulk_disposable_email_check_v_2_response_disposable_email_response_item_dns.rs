pub use crate::prelude::*;

/// DNS records for the email domain. Omitted entirely when validSyntax is false or the domain does not exist (NXDOMAIN).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns {
    /// Every MX hostname at the domain's lowest preference (not just one). ["."]  means the domain publishes a null MX and accepts no mail at all. Absent entirely when the domain has no MX record of any kind — check aRecord in that case.
    #[serde(rename = "mxRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx_record: Option<Vec<String>>,
    /// Collection of A (Address) records for the domain. Present when the domain has no MX record but does resolve to a web server or other host.
    #[serde(rename = "aRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_record: Option<Vec<String>>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns {
    pub fn builder() -> BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDnsBuilder {
        <BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDnsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDnsBuilder {
    mx_record: Option<Vec<String>>,
    a_record: Option<Vec<String>>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDnsBuilder {
    pub fn mx_record(mut self, value: Vec<String>) -> Self {
        self.mx_record = Some(value);
        self
    }

    pub fn a_record(mut self, value: Vec<String>) -> Self {
        self.a_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns`].
    pub fn build(
        self,
    ) -> Result<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns, BuildError> {
        Ok(
            BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns {
                mx_record: self.mx_record,
                a_record: self.a_record,
            },
        )
    }
}
