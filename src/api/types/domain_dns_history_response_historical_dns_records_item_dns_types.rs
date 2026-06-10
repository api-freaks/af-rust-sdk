pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes {
    #[serde(rename = "A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub a: Option<f64>,
    #[serde(rename = "AAAA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub aaaa: Option<f64>,
    #[serde(rename = "CNAME")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cname: Option<f64>,
    #[serde(rename = "MX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mx: Option<f64>,
    #[serde(rename = "NS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ns: Option<f64>,
    #[serde(rename = "SOA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub soa: Option<f64>,
    #[serde(rename = "TXT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub txt: Option<f64>,
    #[serde(rename = "SPF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub spf: Option<f64>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes {
    pub fn builder() -> DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypesBuilder {
        <DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypesBuilder {
    a: Option<f64>,
    aaaa: Option<f64>,
    cname: Option<f64>,
    mx: Option<f64>,
    ns: Option<f64>,
    soa: Option<f64>,
    txt: Option<f64>,
    spf: Option<f64>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypesBuilder {
    pub fn a(mut self, value: f64) -> Self {
        self.a = Some(value);
        self
    }

    pub fn aaaa(mut self, value: f64) -> Self {
        self.aaaa = Some(value);
        self
    }

    pub fn cname(mut self, value: f64) -> Self {
        self.cname = Some(value);
        self
    }

    pub fn mx(mut self, value: f64) -> Self {
        self.mx = Some(value);
        self
    }

    pub fn ns(mut self, value: f64) -> Self {
        self.ns = Some(value);
        self
    }

    pub fn soa(mut self, value: f64) -> Self {
        self.soa = Some(value);
        self
    }

    pub fn txt(mut self, value: f64) -> Self {
        self.txt = Some(value);
        self
    }

    pub fn spf(mut self, value: f64) -> Self {
        self.spf = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes`].
    pub fn build(
        self,
    ) -> Result<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes, BuildError> {
        Ok(DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes {
            a: self.a,
            aaaa: self.aaaa,
            cname: self.cname,
            mx: self.mx,
            ns: self.ns,
            soa: self.soa,
            txt: self.txt,
            spf: self.spf,
        })
    }
}
