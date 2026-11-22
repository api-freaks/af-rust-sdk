//! Service clients and API endpoints
//!
//! This module provides the client implementations for all available services.

use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApiFreaks {
    pub config: ClientConfig,
    pub http_client: HttpClient,
}

impl ApiFreaks {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get detailed geolocation data for an IP address including country, city, timezone, currency, and optional security and user-agent information
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `ip` - IPv4, IPv6, or hostname for geolocation lookup
    /// * `lang` - Response language for location fields
    /// * `fields` - Comma separated list of fields to include in response
    /// * `excludes` - Comma separated list of fields to exclude from response
    /// * `include` - Additional data to include (location, network, security, currency, time_zone, user_agent, country_metadata , hostname, liveHostname, hostnameFallbackLivet)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .geolocation_lookup(
    ///             &GeolocationLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 ip: None,
    ///                 lang: None,
    ///                 fields: None,
    ///                 excludes: None,
    ///                 include: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn geolocation_lookup(
        &self,
        request: &GeolocationLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeolocationLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geolocation/lookup",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("ip", request.ip.clone())
                    .serialize("lang", request.lang.clone())
                    .string("fields", request.fields.clone())
                    .string("excludes", request.excludes.clone())
                    .string("include", request.include.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve detailed geolocation data for multiple IP addresses in a single request.
    /// Supports up to `50,000` IP-addresses/host-names per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `lang` - Language of the response.
    /// * `fields` - Comma-separated list of fields to include in the response. Can include "geo".
    /// * `excludes` - Comma-separated list of fields to exclude from the response (except "ip").
    /// * `include` - Comma-separated list of additional information to include in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_geolocation_lookup(
    ///             &BulkGeolocationLookupRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 ips: vec!["ips".to_string()],
    ///                 format: None,
    ///                 lang: None,
    ///                 fields: None,
    ///                 excludes: None,
    ///                 include: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_geolocation_lookup(
        &self,
        request: &BulkGeolocationLookupRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<BulkGeolocationLookupResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/geolocation/lookup",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("lang", request.lang.clone())
                    .string("fields", request.fields.clone())
                    .string("excludes", request.excludes.clone())
                    .string("include", request.include.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get comprehensive security information for a given IP address. Detects VPNs, proxies, Tor nodes, and other security threats.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `ip` - A valid IPv4 or IPv6 address to look up. If omitted, the API uses the public IP of the requesting client.
    /// * `fields` - Comma-separated list of fields to return. Supports dot notation (e.g. security.threat_score).
    /// * `excludes` - Comma-separated list of fields to remove from the response. Supports dot notation (e.g. security.is_tor).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .ip_security_lookup(
    ///             &IpSecurityLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 ip: None,
    ///                 fields: None,
    ///                 excludes: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ip_security_lookup(
        &self,
        request: &IpSecurityLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<IpSecurityLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/ip/security",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("ip", request.ip.clone())
                    .string("fields", request.fields.clone())
                    .string("excludes", request.excludes.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The Bulk IP Security Lookup API allows you to retrieve security details for up to `50,000` IP-addresses in a single request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `fields` - Comma-separated list of fields to return. Supports dot notation (e.g. security.threat_score).
    /// * `excludes` - Comma-separated list of fields to remove from the response. Supports dot notation (e.g. security.is_tor).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_ip_security_lookup(
    ///             &BulkIpSecurityLookupRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 ips: vec!["ips".to_string()],
    ///                 format: None,
    ///                 fields: None,
    ///                 excludes: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_ip_security_lookup(
        &self,
        request: &BulkIpSecurityLookupRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<BulkIpSecurityLookupResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/ip/security",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("fields", request.fields.clone())
                    .string("excludes", request.excludes.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Convert a given address or place name into geographic coordinates (latitude and longitude).
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `query` - Free-form search query, e.g. Wembley Stadium, London
    /// * `limit` - Max number of results to return (1–40). May return fewer if matches are weak.
    /// * `min_lat` - Minimum latitude for the viewbox. Must be ≤ max_lat and between -90 and 90.
    /// * `max_lat` - Maximum latitude for the viewbox. Must be ≥ min_lat and between -90 and 90.
    /// * `min_lon` - Minimum longitude for the viewbox. Must be ≤ max_lon and between -180 and 180.
    /// * `max_lon` - Maximum longitude for the viewbox. Must be ≥ min_lon and between -180 and 180.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .geocoder_search(
    ///             &GeocoderSearchQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 query: "query".to_string(),
    ///                 format: None,
    ///                 limit: None,
    ///                 min_lat: None,
    ///                 max_lat: None,
    ///                 min_lon: None,
    ///                 max_lon: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn geocoder_search(
        &self,
        request: &GeocoderSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GeocoderSearchResponseItem>, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.accept_language {
            options
                .additional_headers
                .insert("Accept-Language".to_string(), value.clone());
        }
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geocoder/search",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .structured_query("query", request.query.clone())
                    .int("limit", request.limit.clone())
                    .float("min_lat", request.min_lat.clone())
                    .float("max_lat", request.max_lat.clone())
                    .float("min_lon", request.min_lon.clone())
                    .float("max_lon", request.max_lon.clone())
                    .build(),
                Some(options),
            )
            .await
    }

    /// Convert geographic coordinates (latitude and longitude) into a human-readable address or place name.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `lat` - WGS84 latitude value ranging from -90 to 90.
    /// * `lon` - WGS84 longitude value ranging from -180 to 180.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .geocoder_reverse(
    ///             &GeocoderReverseQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 lat: 1.1,
    ///                 lon: 1.1,
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn geocoder_reverse(
        &self,
        request: &GeocoderReverseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeocoderReverseResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.accept_language {
            options
                .additional_headers
                .insert("Accept-Language".to_string(), value.clone());
        }
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geocoder/reverse",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .float("lat", request.lat.clone())
                    .float("lon", request.lon.clone())
                    .build(),
                Some(options),
            )
            .await
    }

    /// Retrieve current WHOIS information for a domain name.
    /// This endpoint provides detailed registration information including registrar details,
    /// dates, nameservers, and registrant information.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format (defaults to json)
    /// * `domain_name` - Domain name for WHOIS lookup
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_whois_lookup(
    ///             &DomainWhoisLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_name: "domainName".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_whois_lookup(
        &self,
        request: &DomainWhoisLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainWhoisLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/whois/live",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domainName", request.domain_name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve WHOIS information for `100 Domains per Request`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_domain_whois_lookup(
    ///             &BulkDomainWhoisLookupRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_names: vec!["domainNames".to_string()],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_domain_whois_lookup(
        &self,
        request: &BulkDomainWhoisLookupRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkDomainWhoisLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/domain/whois/live",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns WHOIS registration details for a specified IP address (IPv4 or IPv6).
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `ip` - The IP address (IPv4 or IPv6) for which WHOIS data is requested.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .ip_whois_lookup(
    ///             &IpWhoisLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 ip: "ip".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ip_whois_lookup(
        &self,
        request: &IpWhoisLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<IpWhoisLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/ip/whois/live",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("ip", request.ip.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns WHOIS registration details for a specified ASN, with or without the 'as' prefix.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `asn` - The Autonomous System Number (ASN) to retrieve WHOIS data for. Can be prefixed with 'as' or not.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .asn_whois_lookup(
    ///             &AsnWhoisLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 asn: "asn".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn asn_whois_lookup(
        &self,
        request: &AsnWhoisLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsnWhoisLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/asn/whois/live",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("asn", request.asn.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve historical WHOIS records for a domain name.
    /// This endpoint provides a timeline of all recorded changes in domain registration information.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain_name` - Domain name for historical WHOIS lookup
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_whois_history(
    ///             &DomainWhoisHistoryQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_name: "domainName".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_whois_history(
        &self,
        request: &DomainWhoisHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainWhoisHistoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/whois/history",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domainName", request.domain_name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Performs a reverse WHOIS search using one or more search parameters like keyword, email, owner, or company.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `keyword` - Keyword search term for reverse WHOIS by keyword (case-insensitive pattern matching).
    /// * `email` - Email search term for reverse WHOIS by email address (case-insensitive exact or regex match; * wildcard supported).
    /// * `owner` - Registrant or owner name for reverse WHOIS (a full-text search phrase matching technique to retrieve results).
    /// * `company` - Organization or company name for reverse WHOIS (full-text search phrase matching technique to retrieve results).
    /// * `exact` - Accepts 'true' or 'false'. "true" returns only records that exactly match the input (keyword, owner/registrant, or company). "false" returns all matches and is the default when omitted.
    /// * `page` - Page number for paginated results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_whois_reverse(
    ///             &DomainWhoisReverseQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 keyword: None,
    ///                 email: None,
    ///                 owner: None,
    ///                 company: None,
    ///                 exact: None,
    ///                 mode: None,
    ///                 page: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_whois_reverse(
        &self,
        request: &DomainWhoisReverseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainWhoisReverseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/whois/reverse",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("keyword", request.keyword.clone())
                    .string("email", request.email.clone())
                    .string("owner", request.owner.clone())
                    .string("company", request.company.clone())
                    .bool("exact", request.exact.clone())
                    .serialize("mode", request.mode.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve real-time DNS records for any hostname. Supports multiple record types including A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME records.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `host_name` - Hostname or URL whose DNS records are required. Give either `host-name` or `ipAddress`.
    /// * `ip_address` - The IP address whose PTR (reverse DNS) record is required. Give either `host-name` or `ipAddress`; with `ipAddress`, set `type` to `all` or `PTR`.
    /// * `type_` - DNS record types to look up, sent as a comma-separated list (e.g. `A,MX`). Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, PTR, or all. When ipAddress is provided, use "all" or "PTR".
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_dns_lookup(
    ///             &DomainDnsLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 r#type: vec![Some("type".to_string())],
    ///                 format: None,
    ///                 host_name: None,
    ///                 ip_address: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_dns_lookup(
        &self,
        request: &DomainDnsLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainDnsLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/dns/live",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("host-name", request.host_name.clone())
                    .string("ipAddress", request.ip_address.clone())
                    .string("type", {
                        let joined = request
                            .r#type
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .build(),
                options,
            )
            .await
    }

    /// Perform DNS lookups for multiple hostnames in a single request. Supports up to `100 host-names per request`
    /// and returns DNS records including A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME records.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `type_` - DNS record types to look up, sent as a comma-separated list (e.g. `A,MX`). Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_domain_dns_lookup(
    ///             &BulkDomainDnsLookupRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 r#type: vec![Some("type".to_string())],
    ///                 domain_names: vec!["domainNames".to_string()],
    ///                 format: None,
    ///                 ip_addresses: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_domain_dns_lookup(
        &self,
        request: &BulkDomainDnsLookupRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkDomainDnsLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/domain/dns/live",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("type", {
                        let joined = request
                            .r#type
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve historical DNS records for any hostname. Access unique historical data for A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME records,
    /// including subdomains. Results are paginated with up to 100 unique records per page.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `host_name` - Hostname or URL whose historical DNS records are required
    /// * `type_` - DNS record types to look up, sent as a comma-separated list (e.g. `A,MX`). Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all.
    /// * `page` - Page number for paginated results
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_dns_history(
    ///             &DomainDnsHistoryQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 host_name: "host-name".to_string(),
    ///                 r#type: vec![Some("type".to_string())],
    ///                 format: None,
    ///                 page: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_dns_history(
        &self,
        request: &DomainDnsHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainDnsHistoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/dns/history",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("host-name", request.host_name.clone())
                    .string("type", {
                        let joined = request
                            .r#type
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve all the hostnames associated with any particular A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME DNS records. For instance, you can access all the hostnames hosted on any IP/CIDR notation, all the domain names using Cloudflare name servers, and all the domain names using Google Mailbox
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `type_` - The type of reverse DNS lookup to perform. Determines how the value parameter is interpreted:
    /// - A: IPv4 CIDR block
    /// - AAAA: IPv6 CIDR block
    /// - MX: Mail provider domain
    /// - NS: Name server provider hostname
    /// - SOA: SOA record admin domain
    /// - SPF/TXT: Target verification strings
    /// - CNAME: Target hostname
    /// * `value` - Provide an IP or CIDR for A/AAAA lookups, or a hostname/selector for MX, NS, SOA, SPF, TXT, and CNAME queries. Wildcard regex patterns are also supported (e.g., mail.google.com, m*.google.com, _spf.g*.com, s*.g*.com).
    /// * `exact` - Accepts 'true' or 'false'. "true" returns only records that exactly match the input (NS, MX, CNAME, SOA, SPF, TXT). "false" returns all matches (default when omitted).
    /// * `page` - Page number to paginate through results (defaults to 1).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_dns_reverse(
    ///             &DomainDnsReverseQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 r#type: DomainDnsReverseRequestType::A,
    ///                 value: "value".to_string(),
    ///                 format: None,
    ///                 exact: None,
    ///                 page: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_dns_reverse(
        &self,
        request: &DomainDnsReverseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainDnsReverseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/dns/reverse",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .serialize("type", Some(request.r#type.clone()))
                    .string("value", request.value.clone())
                    .bool("exact", request.exact.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Execute a series of web scraping instructions on a target URL.
    /// Supports various operations like form filling, clicking, data extraction, and CAPTCHA solving.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API.
    /// * `url` - Target URL to scrape
    /// * `text` - Set to `true` to return the data in text format else `false` for data in html format with tags.
    /// * `js_enabled` - Set  `true` to handle websites with JavaScript. Set `false` to handle static html websites.
    ///
    ///
    /// Default value is `true`.
    /// * `proxy` - Use a proxy for the request. Pass true/false to toggle the default proxy pool, or a proxy URL string to use a specific proxy.
    /// * `ssl_ignore` - Ignore SSL certificate errors.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    /// * `window_size` - Specify the browser window size in the format 'width,height' (e.g., "1920w,1080h"). Default value is the default resolutions provided by web/browser.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    /// * `ad_block` - Set to `true` to apply ad-blocker to the specified URL else false or ignore to not apply.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    /// * `captcha` - if true user can provide captcha instructions in the instructions to solve image captchas.
    ///
    ///
    /// Only works if **jsEnabled** is **true**.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .web_scrape(
    ///             &WebScrapeRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 url: "https://example.com".to_string(),
    ///                 body: WebScrapeRequestBody::WebScrapeRequestBodyBlockUrl(
    ///                     WebScrapeRequestBodyBlockUrl {
    ///                         instructions: vec![
    ///                             WebScrapeRequestBodyBlockUrlInstructionsItem {
    ///                                 ..Default::default()
    ///                             },
    ///                             WebScrapeRequestBodyBlockUrlInstructionsItem {
    ///                                 ..Default::default()
    ///                             },
    ///                             WebScrapeRequestBodyBlockUrlInstructionsItem {
    ///                                 ..Default::default()
    ///                             },
    ///                             WebScrapeRequestBodyBlockUrlInstructionsItem {
    ///                                 ..Default::default()
    ///                             },
    ///                             WebScrapeRequestBodyBlockUrlInstructionsItem {
    ///                                 extract: Some(HashMap::from([
    ///                                     ("profile".to_string(), "#profile".to_string()),
    ///                                     ("welcomeMessage".to_string(), "#welcome-message".to_string()),
    ///                                     ("userInfo".to_string(), "#user-info".to_string()),
    ///                                 ])),
    ///                                 ..Default::default()
    ///                             },
    ///                             WebScrapeRequestBodyBlockUrlInstructionsItem {
    ///                                 ..Default::default()
    ///                             },
    ///                             WebScrapeRequestBodyBlockUrlInstructionsItem {
    ///                                 ..Default::default()
    ///                             },
    ///                         ],
    ///                         ..Default::default()
    ///                     },
    ///                 ),
    ///                 format: None,
    ///                 text: None,
    ///                 js_enabled: None,
    ///                 proxy: None,
    ///                 ssl_ignore: None,
    ///                 window_size: None,
    ///                 ad_block: None,
    ///                 captcha: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn web_scrape(
        &self,
        request: &WebScrapeRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebScrapeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/scraping",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("url", request.url.clone())
                    .bool("text", request.text.clone())
                    .bool("jsEnabled", request.js_enabled.clone())
                    .serialize("proxy", request.proxy.clone())
                    .bool("sslIgnore", request.ssl_ignore.clone())
                    .string("windowSize", request.window_size.clone())
                    .bool("adBlock", request.ad_block.clone())
                    .bool("captcha", request.captcha.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates a single email address and returns result.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .email_validate(
    ///             &EmailValidateRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 email: "email".to_string(),
    ///                 format: None,
    ///                 name: None,
    ///                 ip: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn email_validate(
        &self,
        request: &EmailValidateRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmailValidateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/email-validation/single",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates a bulk of email addresses and returns result for each. Maximum `10` email addresses per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_email_validate(
    ///             &BulkEmailValidateRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 email_data: vec![BulkEmailValidateRequestEmailDataItem {
    ///                     email: "email".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_email_validate(
        &self,
        request: &BulkEmailValidateRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkEmailValidateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/email-validation/bulk",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates email syntax and checks the domain for disposable, free, spam, and role-account signals, plus MX/A DNS records. Send the email (and optionally an ip) as a JSON request body. When an ip is included, the response also carries geolocation and security data for it.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .disposable_email_check_v2(
    ///             &DisposableEmailCheckV2Request {
    ///                 api_key: "apiKey".to_string(),
    ///                 email: "test@sogetthis.com".to_string(),
    ///                 format: None,
    ///                 ip: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn disposable_email_check_v2(
        &self,
        request: &DisposableEmailCheckV2Request,
        options: Option<RequestOptions>,
    ) -> Result<DisposableEmailCheckV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2.0/email/disposable-check",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Checks a bulk of email addresses (each with an optional ip) and returns a disposable email check result for each, in the same order as the request. Maximum `100` entries per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_disposable_email_check_v2(
    ///             &BulkDisposableEmailCheckV2Request {
    ///                 api_key: "apiKey".to_string(),
    ///                 email_data: vec![
    ///                     BulkDisposableEmailCheckV2RequestEmailDataItem {
    ///                         email: "abc".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkDisposableEmailCheckV2RequestEmailDataItem {
    ///                         email: "john@microsoft.com".to_string(),
    ///                         ip: Some("8.8.8.8".to_string()),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkDisposableEmailCheckV2RequestEmailDataItem {
    ///                         email: "test@mailinator.com".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                 ],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_disposable_email_check_v2(
        &self,
        request: &BulkDisposableEmailCheckV2Request,
        options: Option<RequestOptions>,
    ) -> Result<BulkDisposableEmailCheckV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2.0/email/disposable-check/bulk",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates a single phone number and returns detailed metadata including carrier, line type, geolocation, time zones, and standardized formats.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object. If not provided, the API defaults to JSON format.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .phone_validate(
    ///             &PhoneValidateRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 number: "+14155552671".to_string(),
    ///                 format: None,
    ///                 region: None,
    ///                 dialer_region: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn phone_validate(
        &self,
        request: &PhoneValidateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PhoneValidateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/phone/validation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates up to 100 phone numbers in a single request. Each number is processed independently — invalid entries return per-number errors without affecting the rest of the batch.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object. If not provided, the API defaults to JSON format.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_phone_validate(
    ///             &BulkPhoneValidateRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 numbers: vec![
    ///                     BulkPhoneValidateRequestNumbersItem {
    ///                         number: "+14155552671".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkPhoneValidateRequestNumbersItem {
    ///                         number: "+447911123456".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkPhoneValidateRequestNumbersItem {
    ///                         number: "+919876543210".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                 ],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_phone_validate(
        &self,
        request: &BulkPhoneValidateRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<BulkPhoneValidateResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/phone/validation/bulk",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve comprehensive SSL certificate information without the certificate chain.
    /// This endpoint provides detailed information about the SSL certificate including expiry dates, issuer details, and encryption methods.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain_name` - Domain name or URL whose SSL certificate lookup is required
    /// * `ssl_raw` - Set to true to get the raw openSSL response of the domain
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_ssl_lookup(
    ///             &DomainSslLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_name: "domainName".to_string(),
    ///                 format: None,
    ///                 ssl_raw: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_ssl_lookup(
        &self,
        request: &DomainSslLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainSslLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/ssl/live",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domainName", request.domain_name.clone())
                    .bool("sslRaw", request.ssl_raw.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve the complete SSL certificate chain from root Certificate Authority (CA) to end-user certificate.
    /// This endpoint provides comprehensive information about each certificate in the chain.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain_name` - Domain name or URL whose SSL certificate chain lookup is required
    /// * `ssl_raw` - Set to true to get the raw openSSL response for each certificate in the chain
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_ssl_chain_lookup(
    ///             &DomainSslChainLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_name: "domainName".to_string(),
    ///                 format: None,
    ///                 ssl_raw: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_ssl_chain_lookup(
        &self,
        request: &DomainSslChainLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainSslChainLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/ssl/live/chain",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domainName", request.domain_name.clone())
                    .bool("sslRaw", request.ssl_raw.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The Domain Search API is designed to simplify the process of finding available domain names across all top-level domains (TLDs) and second-level domains (SLDs).
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain` - Domain name whose availability is to be checked.
    /// * `source` - Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_availability_check(
    ///             &DomainAvailabilityCheckQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain: "domain".to_string(),
    ///                 format: None,
    ///                 source: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_availability_check(
        &self,
        request: &DomainAvailabilityCheckQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainAvailabilityCheckResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/availability",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domain", request.domain.clone())
                    .serialize("source", request.source.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Perform Bulk Domain Availability checks using a list of domains. Supports upto `100 Domains Per Request`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `source` - Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_domain_availability_check(
    ///             &BulkDomainAvailabilityCheckRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_names: vec!["domainNames".to_string()],
    ///                 format: None,
    ///                 source: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_domain_availability_check(
        &self,
        request: &BulkDomainAvailabilityCheckRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkDomainAvailabilityCheckResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/domain/availability",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .serialize("source", request.source.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The Domain Search API is designed to simplify the process of finding available domain names across all top-level domains (TLDs) and second-level domains (SLDs).
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain` - Domain name for availability and suggestions.
    /// * `source` - Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    /// * `count` - Number of suggestions to retrieve. The API returns a minimum of 5 suggestions regardless of a lower value.
    /// * `sug` - Controls the response shape. When `false`, returns a single availability object for the queried domain only. When omitted or `true`, returns an array of suggested domains instead.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_availability_suggestions(
    ///             &DomainAvailabilitySuggestionsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain: "domain".to_string(),
    ///                 format: None,
    ///                 source: None,
    ///                 count: None,
    ///                 sug: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_availability_suggestions(
        &self,
        request: &DomainAvailabilitySuggestionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainAvailabilitySuggestionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/availability/suggestions",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domain", request.domain.clone())
                    .serialize("source", request.source.clone())
                    .int("count", request.count.clone())
                    .bool("sug", request.sug.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The Subdomain Lookup API is designed to retrieve subdomains related to the given domain name. It helps you explore subdomains that are available for registration or usage.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain` - Domain name for availability and suggestions.
    /// * `after` - Filter subdomains seen after this date (format YYYY-MM-DD).
    /// * `before` - Filter subdomains seen before this date( format YYYY-MM-DD).
    /// * `status` - Filter subdomains by status (active or inactive).
    /// * `page` - Page number for paginated results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .subdomains_lookup(
    ///             &SubdomainsLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain: "domain".to_string(),
    ///                 format: None,
    ///                 after: None,
    ///                 before: None,
    ///                 status: None,
    ///                 page: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn subdomains_lookup(
        &self,
        request: &SubdomainsLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubdomainsLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/subdomains/lookup",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domain", request.domain.clone())
                    .date("after", request.after.clone())
                    .date("before", request.before.clone())
                    .serialize("status", request.status.clone())
                    .string("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The Domain Typosquatting API searches for registered domains that are typo or look-alike variants of a brand keyword, or that match a wildcard pattern. Results include registration lifecycle data and drop status across 1529+ TLDs, paginated at 100 domains per page.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `keyword` - Brand or label to find typo variants for. 3-63 characters, letters, digits, or hyphens, a single label with no dots. Case-insensitive. Use either keyword or pattern, never both.
    /// * `pattern` - Wildcard search string that combines fuzzy matching with * wildcards. 3-63 characters total, * is the only supported wildcard and each one matches zero or more characters, maximum 3 asterisks per request. Use either keyword or pattern, never both.
    /// * `page_token` - Token from nextPageToken in the previous response. Required to retrieve page 2 and onward. The original keyword or pattern must be passed alongside the token on every page request. Results page at 100 domains per page.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_typosquatting(
    ///             &DomainTyposquattingQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 keyword: None,
    ///                 pattern: None,
    ///                 page_token: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_typosquatting(
        &self,
        request: &DomainTyposquattingQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainTyposquattingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/typosquatting",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("keyword", request.keyword.clone())
                    .string("pattern", request.pattern.clone())
                    .string("pageToken", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The Domain Reputation API evaluates a domain against threat intelligence sources, DGA (domain generation algorithm) scoring, trust signals, and email deliverability configuration, returning a consolidated risk assessment with a verdict, severity, and supporting evidence.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain_name` - The domain name to assess (e.g. example.com). Must contain at least one dot and be at most 253 characters. Automatically lowercased.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_reputation(
    ///             &DomainReputationQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_name: "domainName".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_reputation(
        &self,
        request: &DomainReputationQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainReputationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/domain/reputation",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domainName", request.domain_name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve sunrise and sunset times, current position of the moon, and other related information by specifying a location address, location coordinates, IP address, or using the client IP address if no parameter is passed.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response. Can be "json" or "xml".
    /// * `location` - Extract astronomy information using location (preferably city)
    /// * `lat` - Latitude to extract astronomy information using location coordinates
    /// * `long` - Longitude to extract astronomy information using location coordinates
    /// * `ip` - IPv4 or IPv6 address to extract astronomy information using IP address
    /// * `lang` - Response language of "location" field in case of lookup through IP address only.
    /// * `date` - Specific date (format YYYY-MM-DD) for which astronomy data is required
    /// * `elevation` - Elevation above sea level at the location, in meters. The value should be between 0 meter and a maximum value of 10,000 meters. Negative value is set to 0.
    /// * `time_zone` - Time zone to receive all time-based data in your preferred local time.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .astronomy_lookup_v2(
    ///             &AstronomyLookupV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 lang: None,
    ///                 date: None,
    ///                 elevation: None,
    ///                 time_zone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn astronomy_lookup_v2(
        &self,
        request: &AstronomyLookupV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AstronomyLookupV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/geolocation/astronomy",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .serialize("lang", request.lang.clone())
                    .date("date", request.date.clone())
                    .float("elevation", request.elevation.clone())
                    .string("time_zone", request.time_zone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get current time, date, and timezone details by specifying a timezone name, location address, GPS coordinates, IP address, IATA/ICAO airport code, UN/LOCODE, or use the client IP if no parameter is provided.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response. Possible values: json, xml.
    /// * `ip` - IPv4 or IPv6 address to extract timezone information.
    /// * `tz` - Timezone name in IANA format (e.g., Asia/Kolkata) to retrieve information directly.
    /// * `location` - Location string (preferably city and country) to extract timezone.
    /// * `lat` - Latitude for geolocation-based timezone lookup. Only time_zone is returned for this mode; no location object is included.
    /// * `long` - Longitude for geolocation-based timezone lookup. Only time_zone is returned for this mode; no location object is included.
    /// * `lang` - Response language for location fields. Default: en.
    /// * `iata_code` - 3-letter IATA airport code (e.g., LHR) to extract timezone.
    /// * `icao_code` - 4-letter ICAO airport code (e.g., KJFK) to extract timezone.
    /// * `lo_code` - 5-letter UN/LOCODE city code to extract timezone.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .timezone_lookup_v2(
    ///             &TimezoneLookupV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 ip: None,
    ///                 tz: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 lang: None,
    ///                 iata_code: None,
    ///                 icao_code: None,
    ///                 lo_code: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn timezone_lookup_v2(
        &self,
        request: &TimezoneLookupV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimezoneLookupV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/geolocation/timezone",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("ip", request.ip.clone())
                    .string("tz", request.tz.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .serialize("lang", request.lang.clone())
                    .string("iata_code", request.iata_code.clone())
                    .string("icao_code", request.icao_code.clone())
                    .string("lo_code", request.lo_code.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get detailed IP geolocation data for an IP address including country, city, timezone, currency, and optional threat intelligence and user-agent information.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `ip` - IPv4, IPv6, or hostname for geolocation lookup.
    /// * `lang` - Response language for location fields. Default: en.
    /// * `fields` - Comma-separated list of fields to include in response. For example, `location` includes all location fields, `location.city` is a specific field.
    /// * `excludes` - Comma-separated list of fields to exclude from response.
    /// * `include` - Comma-separated list of additional data modules to include. Possible values: security (threat intelligence), hostname (IP-Hostname lookup), liveHostname (live hostname lookup), hostnameFallbackLive (hostname with live fallback), user_agent (parse User-Agent header), abuse (abuse contact info), dma_code (DMA code), geo_accuracy (accuracy_radius, confidence, locality), * (all modules).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .geolocation_lookup_v2(
    ///             &GeolocationLookupV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 ip: None,
    ///                 lang: None,
    ///                 fields: None,
    ///                 excludes: None,
    ///                 include: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn geolocation_lookup_v2(
        &self,
        request: &GeolocationLookupV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeolocationLookupV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/geolocation/lookup",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("ip", request.ip.clone())
                    .serialize("lang", request.lang.clone())
                    .string("fields", request.fields.clone())
                    .string("excludes", request.excludes.clone())
                    .string("include", request.include.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get detailed IP geolocation data for multiple IP addresses including country, city, timezone, currency, and optional threat intelligence information. Supports up to 50,000 IP addresses per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `lang` - Response language for location fields. Default: en.
    /// * `fields` - Comma-separated list of fields to include in the response. For example, `location` includes all location fields, `location.city` is a specific field.
    /// * `excludes` - Comma-separated list of fields to exclude from response.
    /// * `include` - Comma-separated list of additional data modules to include. Possible values: security (threat intelligence), hostname (IP-Hostname lookup), liveHostname (live hostname lookup), user_agent (parse User-Agent header), abuse (abuse contact info), * (all modules).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_geolocation_lookup_v2(
    ///             &BulkGeolocationLookupV2Request {
    ///                 api_key: "apiKey".to_string(),
    ///                 ips: vec!["ips".to_string()],
    ///                 format: None,
    ///                 lang: None,
    ///                 fields: None,
    ///                 excludes: None,
    ///                 include: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_geolocation_lookup_v2(
        &self,
        request: &BulkGeolocationLookupV2Request,
        options: Option<RequestOptions>,
    ) -> Result<Vec<BulkGeolocationLookupV2ResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2.0/geolocation/lookup",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .serialize("lang", request.lang.clone())
                    .string("fields", request.fields.clone())
                    .string("excludes", request.excludes.clone())
                    .string("include", request.include.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the current WHOIS record for the specified domain, including registrar details, registrant/administrative/technical/billing/reseller contacts, name servers, status codes, and raw WHOIS text.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `domain_name` - Domain name to retrieve WHOIS data for (e.g. example.com).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .domain_whois_lookup_v2(
    ///             &DomainWhoisLookupV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_name: "domainName".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn domain_whois_lookup_v2(
        &self,
        request: &DomainWhoisLookupV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DomainWhoisLookupV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/domain/whois/live",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("domainName", request.domain_name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the current WHOIS record for each requested domain, in request order. Supports up to 100 domain names per request; a domain that fails to resolve yields an error item instead of failing the whole batch.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_domain_whois_lookup_v2(
    ///             &BulkDomainWhoisLookupV2Request {
    ///                 api_key: "apiKey".to_string(),
    ///                 domain_names: vec!["domainNames".to_string()],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_domain_whois_lookup_v2(
        &self,
        request: &BulkDomainWhoisLookupV2Request,
        options: Option<RequestOptions>,
    ) -> Result<BulkDomainWhoisLookupV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2.0/domain/whois/live",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the current live price for the requested commodity symbols. Unresolved symbols degrade to a 206 partial response instead of failing the whole request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format. Currently only `json` is supported.
    /// * `symbols` - Comma-separated list of commodity symbols (e.g., XAU, WTIOIL-SPOT). Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
    /// * `quote` - Target currency for the exchange rate. If omitted (or set to `default`), the default quote currency of each commodity is used. Requires a premium plan; ignored on lower-tier plans.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_latest_rates_v2(
    ///             &CommodityLatestRatesV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 format: None,
    ///                 quote: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_latest_rates_v2(
        &self,
        request: &CommodityLatestRatesV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityLatestRatesV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/commodity/rates/latest",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .string("quote", request.quote.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns OHLC price data for the requested commodity symbols on a specific date. Falls back to the nearest earlier rate if none exists for the exact date. Unresolved symbols degrade to a 206 partial response instead of failing the whole request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format. Currently only `json` is supported.
    /// * `symbols` - Comma-separated list of commodity symbols. Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
    /// * `date` - Date in YYYY-MM-DD format. Data available from 1990 onwards.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_historical_rates_v2(
    ///             &CommodityHistoricalRatesV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_historical_rates_v2(
        &self,
        request: &CommodityHistoricalRatesV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityHistoricalRatesV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/commodity/rates/historical",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .date("date", request.date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns price fluctuation metrics (start, end, change, percent change) for the requested commodity symbols over a date range. For monthly-updated commodities the range snaps to month boundaries. Unresolved symbols degrade to a 206 partial response instead of failing the whole request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format. Currently only `json` is supported.
    /// * `symbols` - Comma-separated list of commodity symbols. Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
    /// * `start_date` - Start date (YYYY-MM-DD)
    /// * `end_date` - End date (YYYY-MM-DD)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_fluctuation_v2(
    ///             &CommodityFluctuationV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_fluctuation_v2(
        &self,
        request: &CommodityFluctuationV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityFluctuationV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/commodity/fluctuation",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns day-by-day OHLC data for the requested commodity symbols within a date range, indexed by date. Non-trading days are excluded. Unresolved symbols degrade to a 206 partial response instead of failing the whole request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format. Currently only `json` is supported.
    /// * `symbols` - Comma-separated list of commodity symbols. Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
    /// * `start_date` - Start date (YYYY-MM-DD)
    /// * `end_date` - End date (YYYY-MM-DD). Maximum range is 365 days.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_time_series_v2(
    ///             &CommodityTimeSeriesV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_time_series_v2(
        &self,
        request: &CommodityTimeSeriesV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityTimeSeriesV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/commodity/time-series",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the list of supported commodity symbols with metadata. Deprecated symbols stay listed with status "inactive" and a deprecationDate.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format. Currently only `json` is supported.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_symbols_v2(
    ///             &CommoditySymbolsV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_symbols_v2(
        &self,
        request: &CommoditySymbolsV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommoditySymbolsV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/commodity/symbols",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the prices of one commodity through a UTC day at the requested interval, keyed by UTC timestamp. Each request costs a flat 50 credits however many points it returns.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `symbols` - Exactly one commodity symbol. Case-insensitive. Sending two or more returns a 400. Only symbols updated every second, every minute, or every 10 minutes have intraday rates; symbols updated hourly or less often return a 400. Use the Commodity Symbols endpoint for all valid values.
    /// * `interval` - Spacing between points. Case-insensitive. Symbols updated every 10 minutes accept only `10m`, `30m`, `1h`, `2h`, `4h`, `8h`, `12h`, `1d`.
    /// * `date` - The UTC day to read, in `YYYY-MM-DD` format. Cannot be in the future. For today, points stop at the current UTC minute.
    /// * `start_time` - First mark of the window, as a 24-hour UTC time in `HH:mm`. Must fall on a mark of the interval (for example, `14:30` works for `5m` or `30m`, but not for `1h`). For `1d` it can only be `00:00`.
    /// * `metadata` - Whether to include the `metadata` object. Pass `false` to receive only the prices.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_intraday_rates_v2(
    ///             &CommodityIntradayRatesV2QueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: "symbols".to_string(),
    ///                 interval: CommodityIntradayRatesV2RequestInterval::OneM,
    ///                 date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 start_time: None,
    ///                 metadata: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_intraday_rates_v2(
        &self,
        request: &CommodityIntradayRatesV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityIntradayRatesV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2.0/commodity/rates/intraday",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .string("symbols", request.symbols.clone())
                    .serialize("interval", Some(request.interval.clone()))
                    .date("date", request.date.clone())
                    .string("startTime", request.start_time.clone())
                    .bool("metadata", request.metadata.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This API merges multiple PDF files into a single PDF, in the order they are provided
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - An array of unique file IDs referencing PDF files previously uploaded to the API Freaks server. Use this parameter to merge existing files without re-uploading them. Provide multiple IDs to merge files in the specified order.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - Specifies the desired name for the resulting merged PDF file. If not provided, a default name will be assigned.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_merge(
    ///             &PdfMergeRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
    ///                 format: None,
    ///                 file_id: vec![],
    ///                 destroy: None,
    ///                 output: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_merge(
        &self,
        request: &PdfMergeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfMergeResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/merge",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string_array("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API removes a selection or range of pages from a PDF file.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique identifier of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output PDF file after pages have been removed. If not provided, a default name will be assigned.
    /// * `pages` - Specifies which pages to remove from the PDF. Accepts individual page numbers (e.g., '1,7') and/or ascending page ranges (e.g., '3-5'). Use commas to separate entries and hyphens for ranges. Reverse ranges (e.g., '5-3') are not allowed. Alternatively, you may provide only one of the following keywords: 'even' (removes all even-numbered pages), 'odd' (removes all odd-numbered pages), or 'last' (removes only the last page). The keyword 'all' is not supported for this operation. Examples: '1,3-5', 'even'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_remove_pages(
    ///             &PdfRemovePagesRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 pages: "pages".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_remove_pages(
        &self,
        request: &PdfRemovePagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfRemovePagesResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/remove-pages",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("pages", request.pages.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API splits a PDF into multiple parts based on specified page numbers or ranges.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired base name for the output PDF files after splitting. If not provided, a default naming convention will be used.
    /// * `pages` - Defines the page numbers or ranges where the PDF should be split. Provide individual pages and/or ranges in any order (for example: "1-4,9-5,16-last"). Separate entries with commas and use hyphens for ranges.
    ///
    /// Special keywords (use alone):
    ///
    /// • `even` — split at every even-numbered page
    ///
    /// • `odd` — split at every odd-numbered page
    ///
    /// • `all` — split the PDF into single-page files
    ///
    /// The keyword `last` can be used anywhere in the string, in combination with page numbers or ranges (for example: "5-last", "last-2", "1,last,9").
    ///
    /// Examples:
    /// - "1,4-2,last"
    /// - "odd"
    /// - "all"
    /// - "last,2-5"
    ///
    /// Invalid example: "1,odd" (mixing a keyword other than "last" with specific pages/ranges is not allowed). You can pass multiple pages entries to produce multiple output files.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_split(
    ///             &PdfSplitRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 pages: vec![],
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_split(
        &self,
        request: &PdfSplitRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfSplitResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/split",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string_array("pages", request.pages.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API rotates pages of a PDF by a specified angle (in multiples of 90 degrees).
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output PDF file after rotation. If not provided, a default name will be assigned.
    /// * `pages` - Specifies which pages to rotate. Accepts individual page numbers (e.g., '1,7') and/or ascending page ranges (e.g., '3-5'). Use commas to separate entries and hyphens for ranges. Reverse ranges (e.g., '5-3') are not allowed. Alternatively, provide only one of the following keywords: 'even' (rotate all even-numbered pages), 'odd' (rotate all odd-numbered pages), 'last' (rotate only the last page), or 'all' (rotate all pages). Examples: '1,3-5', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `rotate` - The angle in degrees to rotate the selected pages. Must be one of the following values: 0, 90, 180, 270, -90, -180, or -270. All rotations are applied clockwise.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_rotate(
    ///             &PdfRotateRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 rotate: 1,
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 pages: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_rotate(
        &self,
        request: &PdfRotateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfRotateResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/rotate",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("pages", request.pages.clone())
                    .int("rotate", request.rotate.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API compresses a given PDF file to reduce its file size.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file.
    /// * `output` - Name of the output PDF.
    /// * `compression_level` - Controls how aggressively the PDF is compressed. Lower levels preserve more quality, while higher levels reduce file size more.
    /// * `destroy` - If set to true, the input file(s) will be deleted from the server immediately after the output is generated.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_compress(
    ///             &PdfCompressRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 compression_level: PdfCompressRequestCompressionLevel::Low,
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 output: None,
    ///                 destroy: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_compress(
        &self,
        request: &PdfCompressRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfCompressResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/compress",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .string("output", request.output.clone())
                    .serialize("compression_level", Some(request.compression_level.clone()))
                    .bool("destroy", request.destroy.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API extracts specific pages or page ranges from a PDF file and returns them as a new PDF.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output PDF file after pages have been extracted. If not provided, a default name will be assigned.
    /// * `pages` - Specifies which pages to extract from the PDF. You can provide individual page numbers (e.g., '2') and/or page ranges in any order, including descending (e.g., '9-5', '16-last'). Use commas to separate entries and hyphens for ranges. You may alternatively pass only one of the special keywords: 'even' (extracts all even-numbered pages), 'odd' (extracts all odd-numbered pages), 'last' (extracts only the last page), or 'all' (extracts all pages into individual files). Examples: '2,6-3', 'even', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `separated` - If set to `true`, each of the specified pages will be extracted and returned as a separate PDF file.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_extract_pages(
    ///             &PdfExtractPagesRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 pages: "pages".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 separated: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_extract_pages(
        &self,
        request: &PdfExtractPagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfExtractPagesResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/extract-pages",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("pages", request.pages.clone())
                    .bool("separated", request.separated.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// API endpoint that linearizes any given PDF, restructuring it for faster loading and page-by-page viewing in web browsers.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output PDF file after pages have been extracted. If not provided, a default name will be assigned.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_linearize(
    ///             &PdfLinearizeRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_linearize(
        &self,
        request: &PdfLinearizeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfLinearizeResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/linearize",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API encrypts a PDF file by setting a password required to open it.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output encrypted PDF file. If not provided, a default name will be assigned.
    /// * `file_password` - The password to unlock the input file if it is already protected. Either the owner password or user password can be provided. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    /// * `user_password` - Sets the user password required to open and view the encrypted PDF file. Password Length should be between 6 and 128 characters.
    /// * `owner_password` - Sets the owner password for the PDF file. This password provides full access, including the ability to remove restrictions. If not provided, the `user_password` will also be used as the owner password. Password Length should be between 6 and 128 characters.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_encrypt(
    ///             &PdfEncryptRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 user_password: "user_password".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 file_password: None,
    ///                 owner_password: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_encrypt(
        &self,
        request: &PdfEncryptRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfEncryptResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/encrypt",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("file_password", request.file_password.clone())
                    .string("user_password", request.user_password.clone())
                    .string("owner_password", request.owner_password.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API decrypts PDF files, removing all encryption, including open passwords and permission restrictions.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output decrypted PDF file. If not provided, a default name will be assigned.
    /// * `file_password` - The password to unlock the input file if it is protected. Either the owner password or user password can be provided. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_decrypt(
    ///             &PdfDecryptRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file_password: "file_password".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_decrypt(
        &self,
        request: &PdfDecryptRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfDecryptResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/decrypt",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("file_password", request.file_password.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API applies permission restrictions on a PDF file, such as disabling printing, copying, or editing. This can include password protection to enforce restrictions.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output restricted PDF file. If not provided, a default name will be assigned.
    /// * `file_password` - The password to unlock the input file if it is already secured. Provide the owner password if available; otherwise, the user password. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    /// * `user_password` - Sets the password users will use to open the PDF. If this is not set, only the owner password will be configured, and anyone can open the PDF file with the provided restrictions enabled. Password Length should be between 6 and 128 characters.
    /// * `owner_password` - Sets the password that allows full access to the PDF (e.g., removing restrictions). If not provided, the `user_password` (if set) will also be used as the owner password. Password Length should be between 6 and 128 characters.
    /// * `restrictions` - A comma-separated list of restrictions to apply to the PDF. These define what the end-user is *not* allowed to do with the PDF. Available options are:
    ///
    ///
    /// * **print_high** – Disables high-quality printing.
    /// * **print_low** – Disables low-resolution printing.
    /// * **edit_document_assembly** – Prevents reordering or inserting pages.
    /// * **fill_form_fields** – Disallows filling in PDF form fields.
    /// * **edit_annotations** – Disables adding or modifying annotations or comments.
    /// * **modify_content** – Prevents modifying existing content in the PDF.
    /// * **copy_and_extract_content** – Disables copying text or images from the PDF.
    /// * **use_accessibility** – Prevents screen readers or accessibility tools from accessing content.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_restrict(
    ///             &PdfRestrictRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 user_password: "user_password".to_string(),
    ///                 restrictions: vec![Some(PdfRestrictRequestRestrictionsItem::PrintHigh)],
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 file_password: None,
    ///                 owner_password: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_restrict(
        &self,
        request: &PdfRestrictRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfRestrictResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/restrict",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("file_password", request.file_password.clone())
                    .string("user_password", request.user_password.clone())
                    .string("owner_password", request.owner_password.clone())
                    .serialize_array("restrictions", request.restrictions.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API removes permission restrictions from a PDF while keeping it encrypted. If you want to remove all security (including encryption), use the `/pdf/decrypt` endpoint instead.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    /// * `file_password` - The password to unlock the input file. Either the owner password or user password can be provided. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    /// * `user_password` - Sets the user password for the PDF file. Password Length should be between 6 and 128 characters.
    /// * `owner_password` - Sets the owner password for the PDF file. If the owner password is not provided, the `user_password` will also be used as the owner password. Password Length should be between 6 and 128 characters.
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_unrestrict(
    ///             &PdfUnrestrictRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file_password: "file_password".to_string(),
    ///                 user_password: "user_password".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 owner_password: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_unrestrict(
        &self,
        request: &PdfUnrestrictRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfUnrestrictResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/unrestrict",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("file_password", request.file_password.clone())
                    .string("user_password", request.user_password.clone())
                    .string("owner_password", request.owner_password.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API converts a given PDF file into a sequence of PNG images.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    /// * `pages` - Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `resolution` - Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    /// * `image_smoothing` - Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    /// * `profile` - Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_convert_to_png(
    ///             &PdfConvertToPngRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 pages: None,
    ///                 resolution: None,
    ///                 image_smoothing: None,
    ///                 profile: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_convert_to_png(
        &self,
        request: &PdfConvertToPngRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToPngResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/png",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("pages", request.pages.clone())
                    .int("resolution", request.resolution.clone())
                    .string("image_smoothing", request.image_smoothing.clone())
                    .serialize("profile", request.profile.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API converts a given PDF file into a sequence of JPG images.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    /// * `quality` - Controls JPG compression quality. Higher values yield sharper images with larger file sizes.
    /// * `pages` - Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `resolution` - Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    /// * `image_smoothing` - Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    /// * `profile` - Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_convert_to_jpg(
    ///             &PdfConvertToJpgRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 quality: None,
    ///                 pages: None,
    ///                 resolution: None,
    ///                 image_smoothing: None,
    ///                 profile: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_convert_to_jpg(
        &self,
        request: &PdfConvertToJpgRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToJpgResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/jpg",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .int("quality", request.quality.clone())
                    .string("pages", request.pages.clone())
                    .int("resolution", request.resolution.clone())
                    .string("image_smoothing", request.image_smoothing.clone())
                    .serialize("profile", request.profile.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API converts a given PDF file into a sequence of TIFF images. The output images can be saved as a single TIFF file, or as a sequence of TIFF files.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    /// * `pages` - Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `resolution` - Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    /// * `image_smoothing` - Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    /// * `profile` - Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_convert_to_tiff(
    ///             &PdfConvertToTiffRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 pages: None,
    ///                 resolution: None,
    ///                 image_smoothing: None,
    ///                 profile: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_convert_to_tiff(
        &self,
        request: &PdfConvertToTiffRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToTiffResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/tif",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("pages", request.pages.clone())
                    .int("resolution", request.resolution.clone())
                    .string("image_smoothing", request.image_smoothing.clone())
                    .serialize("profile", request.profile.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// Converts a PDF file to a BMP image.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    /// * `pages` - Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `resolution` - Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    /// * `image_smoothing` - Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    /// * `profile` - Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_convert_to_bmp(
    ///             &PdfConvertToBmpRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 pages: None,
    ///                 resolution: None,
    ///                 image_smoothing: None,
    ///                 profile: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_convert_to_bmp(
        &self,
        request: &PdfConvertToBmpRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToBmpResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/bmp",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("pages", request.pages.clone())
                    .int("resolution", request.resolution.clone())
                    .string("image_smoothing", request.image_smoothing.clone())
                    .serialize("profile", request.profile.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API converts a given PDF file into a sequence of GIF images.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    /// * `destroy` - If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    /// * `output` - The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    /// * `pages` - Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    /// * `resolution` - Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    /// * `image_smoothing` - Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    /// * `profile` - Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    /// * `webhook_url` - The URL to which the webhook notification will be sent after the task is completed.
    /// * `webhook_failure_notification` - If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_convert_to_gif(
    ///             &PdfConvertToGifRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 format: None,
    ///                 file_id: None,
    ///                 destroy: None,
    ///                 output: None,
    ///                 pages: None,
    ///                 resolution: None,
    ///                 image_smoothing: None,
    ///                 profile: None,
    ///                 webhook_url: None,
    ///                 webhook_failure_notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_convert_to_gif(
        &self,
        request: &PdfConvertToGifRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToGifResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if let Some(value) = &request.webhook_authorization {
            options
                .additional_headers
                .insert("X-Webhook-Authorization".to_string(), value.clone());
        }
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/gif",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .bool("destroy", request.destroy.clone())
                    .string("output", request.output.clone())
                    .string("pages", request.pages.clone())
                    .int("resolution", request.resolution.clone())
                    .string("image_smoothing", request.image_smoothing.clone())
                    .serialize("profile", request.profile.clone())
                    .string("webhook_url", request.webhook_url.clone())
                    .bool(
                        "webhook_failure_notification",
                        request.webhook_failure_notification.clone(),
                    )
                    .build(),
                Some(options),
            )
            .await
    }

    /// This API uploads multiple PDF files to the API Freaks server and generates their unique file IDs.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_upload_resources(
    ///             &PdfUploadResourcesRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_upload_resources(
        &self,
        request: &PdfUploadResourcesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfUploadResourcesResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/resource/upload",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This API uploads PDF files to the API Freaks server in binary format.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_name` - The desired name for the uploaded PDF file. This name will be used for storage on the server.
    ///
    ///
    /// **NOTE**: Please ensure file_name has extension `.pdf`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn pdf_upload_binary(
        &self,
        request: &PdfUploadBinaryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfUploadBinaryResponse, ApiError> {
        self.http_client
            .execute_bytes_request(
                Method::POST,
                "v1.0/pdf/resource/upload-binary",
                Some(request.body.to_vec()),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_name", request.file_name.clone())
                    .build(),
                "application/octet-stream",
                options,
            )
            .await
    }

    /// This API downloads PDF files or ZIP archives from the server using their unique resource ID.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `resource_id` - The unique identifier of the file or ZIP archive to download.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_download_resource(
    ///             &PdfDownloadResourceQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 resource_id: "resource_id".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_download_resource(
        &self,
        request: &PdfDownloadResourceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                "v1.0/pdf/resource/download",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("resource_id", request.resource_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This API checks the status of a previously initiated PDF processing task using its unique task ID.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `task_id` - The unique ID of the PDF processing task for which the status is requested.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_get_task_status(
    ///             &PdfGetTaskStatusQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 task_id: "task_id".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_get_task_status(
        &self,
        request: &PdfGetTaskStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfGetTaskStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/pdf/task-status",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("task_id", request.task_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This API checks the status of a PDF file using its unique file ID, providing information about its creation and potential deletion time.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of the file whose status is requested.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_get_file_status(
    ///             &PdfGetFileStatusQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file_id: "file_id".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_get_file_status(
        &self,
        request: &PdfGetFileStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfGetFileStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/pdf/file-status",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This API retrieves a list of all PDF files uploaded and generated by a specific user. Please note that if the user is part of an organization, only the Organization Administrator can access this endpoint. Organization Members cannot access this endpoint.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_list_files(
    ///             &PdfListFilesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_list_files(
        &self,
        request: &PdfListFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfListFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/pdf/files",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This API deletes a PDF file using its unique file ID.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    /// * `file_id` - The unique ID of the file to be deleted.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .pdf_delete_file(
    ///             &PdfDeleteFileQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 file_id: "file_id".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pdf_delete_file(
        &self,
        request: &PdfDeleteFileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfDeleteFileResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                "v1.0/pdf/file",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("file_id", request.file_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Renders the template identified by template_id with the JSON data supplied in the request body (or fetched from data_url). Returns a hosted URL for the generated PDF. Provide either a request body or a data_url; one of the two is required. The keys of the data object are the fields you bound when you designed the template in the APIFreaks PDF Template Builder, so the exact shape is up to you. Pin the version parameter to a specific template version (e.g. version=v2) so later design changes never touch what is already live; omit it to use the latest version.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `template_id` - The template to render, from the APIFreaks PDF Template Builder. Copy it from your dashboard when you save the template.
    /// * `version` - Template version to render, e.g. v2. Must start with v. Omit this parameter entirely to use the latest version; the literal value 'latest' is not accepted by the API.
    /// * `data_url` - A URL the API fetches your JSON data from, instead of the request body. Provide this when you are not sending a request body.
    /// * `request` - Free-form JSON object whose keys map to the fields bound in the template.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client.pdf_generate_from_template(&PdfGenerateFromTemplateRequest {
    ///         api_key: "apiKey".to_string(),
    ///         template_id: "template_id".to_string(),
    ///         body: HashMap::from([("candidate_name".to_string(), serde_json::json!("Ava Thompson")), ("candidate_title".to_string(), serde_json::json!("Senior Operations Analyst")), ("email".to_string(), serde_json::json!("ava.thompson@example.com")), ("phone".to_string(), serde_json::json!("+1 (555) 381-2048")), ("city".to_string(), serde_json::json!("Austin, TX")), ("portfolio_url".to_string(), serde_json::json!("portfolio.example.com")), ("profile_summary".to_string(), serde_json::json!("Analytical operations professional with 8+ years of experience improving reporting pipelines, streamlining process documentation, and coordinating cross-functional delivery for distributed teams across multiple time zones.")), ("company_name".to_string(), serde_json::json!("Northwind Logistics")), ("job_title".to_string(), serde_json::json!("Operations Analyst")), ("employment_period".to_string(), serde_json::json!("2021 - Present")), ("job_achievement_1".to_string(), serde_json::json!("Led weekly KPI reporting for finance, fulfillment, and customer success stakeholders across three business units.")), ("job_achievement_2".to_string(), serde_json::json!("Reduced manual reconciliation work by 35% through workflow redesign and documentation automation.")), ("job_achievement_3".to_string(), serde_json::json!("Spearheaded a cross-team initiative that improved on-time delivery metrics by 18% within six months.")), ("skills".to_string(), serde_json::json!([{"skill":"Process Improvement","level":"Advanced","years":"7"},{"skill":"Stakeholder Reporting","level":"Expert","years":"8"},{"skill":"Cross-functional Coordination","level":"Advanced","years":"5"},{"skill":"Workflow Documentation","level":"Advanced","years":"6"}])), ("degree".to_string(), serde_json::json!("B.S. in Information Systems")), ("institution".to_string(), serde_json::json!("University of Texas")), ("graduation_year".to_string(), serde_json::json!("2018")), ("certification_1".to_string(), serde_json::json!("Certified Business Analysis Professional (CBAP)")), ("certification_2".to_string(), serde_json::json!("Six Sigma Green Belt")), ("languages".to_string(), serde_json::json!("English (Native), Spanish (Professional)"))]),
    ///         version: None,
    ///         data_url: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn pdf_generate_from_template(
        &self,
        request: &PdfGenerateFromTemplateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfGenerateFromTemplateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/pdf/template/generate",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .string("template_id", request.template_id.clone())
                    .string("version", request.version.clone())
                    .string("data_url", request.data_url.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Upload a CSV file where each row is one record. The API generates one PDF per row and returns an array of results, each with its own pdf_url. Rows are processed independently, so a failure in one row does not block the rest of the batch. The response gives a row-by-row breakdown of which PDFs succeeded and which failed.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `template_id` - The template to render, from the APIFreaks PDF Template Builder. Copy it from your dashboard when you save the template.
    /// * `version` - Template version to render, e.g. v2. Must start with v. Omit this parameter entirely to use the latest version; the literal value 'latest' is not accepted by the API.
    /// * `request` - Multipart form-data request body containing the CSV file to render in bulk.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_pdf_generate_from_template(
    ///             &BulkPdfGenerateFromTemplateRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 template_id: "template_id".to_string(),
    ///                 file: b"test file content".to_vec(),
    ///                 version: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_pdf_generate_from_template(
        &self,
        request: &BulkPdfGenerateFromTemplateRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<BulkPdfGenerateFromTemplateResponseItem>, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1.0/pdf/template/generate/bulk",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .string("template_id", request.template_id.clone())
                    .string("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Capture full-page screenshots and videos of websites with advanced options like device simulation, custom code injection, cookie banner blocking, and scrollable content recording.
    /// Supports multiple output formats including JSON, image, GIF, MP4, and WebM.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `output` - Output format for screenshot results. The SDKs support only `json`, which returns metadata with the captured file's URL in `screenshot`; download the file from that URL. (The API's `output=image`, which returns the raw file, is not supported by the SDK methods.)
    /// * `file_type` - File type for screenshot output
    /// * `url` - URLs to capture screenshots of
    /// * `width` - Browser viewport width in pixels
    /// * `height` - Browser viewport height in pixels
    /// * `full_page` - Capture a full-page screenshot
    /// * `fresh` - Bypass cache and take a fresh screenshot
    /// * `no_cookie_banners` - Remove cookie banners from the screenshot
    /// * `enable_caching` - Enable caching for repeated requests
    /// * `block_ads` - Block advertisements on the page
    /// * `block_chat_widgets` - Block chat widget scripts from loading
    /// * `extract_text` - Extract visible text from the page
    /// * `extract_html` - Extract HTML content of the page
    /// * `destroy_screenshot` - Auto-destroy screenshot after fetch
    /// * `lazy_load` - Enable lazy-loading content before screenshot
    /// * `retina` - Capture screenshot in high-DPI (Retina) mode
    /// * `dark_mode` - Render page in dark mode
    /// * `block_tracking` - Block common user-tracking scripts
    /// * `enable_incognito` - Enable private/incognito mode for browser session
    /// * `omit_background` - Omit background color (transparent background)
    /// * `thumbnail_width` - Thumbnail width in pixels
    /// * `adjust_top` - Adjust top in pixels
    /// * `wait_for_event` - Wait for a specific load event before capturing the screenshot.
    /// * `grayscale` - Range:0 to 100 for grayscale filter
    /// * `delay` - How many milliseconds to wait before taking the screenshot
    /// * `timeout` - Maximum timeout in milliseconds. Defalut is `10,000`
    /// * `ttl` - Number of seconds the screenshot should be cached
    /// * `clip_x` - X position of the clipping rectangle in pixels
    /// * `clip_y` - Y position of the clipping rectangle in pixels
    /// * `clip_width` - Width of the clipping rectangle in pixels
    /// * `clip_height` - Height of the clipping rectangle in pixels
    /// * `css_url` - URL to CSS file
    /// * `css` - Your custom CSS code
    /// * `js_url` - URL to JS file
    /// * `js` - Your JS code
    /// * `block_js` - Block Scripts
    /// * `block_stylesheets` - Block Stylesheets
    /// * `block_images` - Block Images
    /// * `block_media` - Block Media
    /// * `block_font` - Block Fonts
    /// * `block_text_track` - Block Text Tracks
    /// * `block_xhr` - Block XHR Requests
    /// * `block_fetch` - Block Fetch Requests
    /// * `block_event_source` - Block Event Source
    /// * `block_web_socket` - Block Web Sockets
    /// * `block_manifest` - Block Manifest
    /// * `block_specific_requests` - Comma- or newline-separated list of specific requests to block. Each line and comma are treated as separate requests for processing. Example: https://example.com, https://example.js
    /// * `blur_selector` - Comma-separated list of indexed CSS selectors to blur.
    /// Format: `index:<selector>`, e.g., `0:.banner,1:#ads`.
    /// * `remove_selector` - Comma-separated list of indexed CSS selectors to blur.
    /// Format: `index:<selector>`, e.g., `0:.banner,1:#ads`.
    /// * `result_file_name` - Specify a meaningful & unique file name to easily identify the screenshot result.
    /// Avoid using spaces or special characters; use hyphens or underscores to separate words.
    /// * `scrolling_screenshot` - **`Scrolling Screenshot`**: Capture a long scrolling screenshot. When true, disable `fullPage` and `freshScreenshot`.
    /// * `scroll_speed` - Speed of scrolling during the screenshot.
    /// * `scroll_back` - If true, the scroll will reverse back to the top after reaching the bottom.
    /// * `start_immediately` - If true, the scrolling capture will start immediately upon page load.
    /// * `multiple_scrolling` - If true, multiple scrolling screenshots will be taken at different viewport sizes.
    /// * `sizes` - Comma-separated list of viewport sizes in the format index:XXw:YYh. Example: sizes=0:120w:300h,1:240w:500h
    /// * `duration` - Duration in seconds for the scrolling capture. Acceptable range: 0 to 100 seconds.
    /// * `captcha_error` - CSS selector of the message shown for a wrong answer. While it is visible, the solver tries again.
    /// * `captcha_image` - CSS selector of the captcha picture. Required when solve_captcha is true.
    /// * `captcha_input` - CSS selector of the box the answer is typed into. Required when solve_captcha is true.
    /// * `captcha_refresh` - CSS selector of the control that loads a new captcha picture before the next try.
    /// * `captcha_retries` - Number of guesses the solver makes, from 1 to 5.
    /// * `captcha_submit` - CSS selector of the button to click after the answer is typed.
    /// * `classic_proxy` - Loads the page through a datacenter proxy. Adds 20 credits per request. Ignored when premium_proxy is true or a custom proxy is set.
    /// * `click_recursion` - Number of times to click the selector (when set via `selector_to_click`).
    /// * `country` - Two-letter country code, such as US or DE, for the exit address of classic_proxy or premium_proxy. Ignored when you send your own proxy. Any two-letter code is sent, including one that is not listed. Classic proxy countries: BG, GE, DE, HN, MD, UA, GB, US. Premium proxy countries: AF, AL, DZ, AO, AG, AR, AM, AW, AU, AT, AZ, BS, BH, BD, BB, BY, BE, BZ, BJ, BT, BO, BQ, BA, BW, BR, BN, BG, BF, BI, CV, KH, CM, CA, KY, CF, TD, CL, CN, CO, KM, CG, CR, CI, HR, CU, CW, CY, CZ, CD, DK, DJ, DM, DO, EC, EG, SV, GQ, EE, SZ, ET, FO, FJ, FI, FR, GF, PF, GA, GM, GE, DE, GH, GI, GR, GL, GD, GP, GU, GT, GG, GN, GW, GY, HT, HN, HK, HU, IS, IN, ID, IR, IQ, IE, IL, IT, JM, JP, JE, JO, KZ, KE, XK, KW, KG, LA, LV, LB, LS, LR, LY, LT, MO, MG, MW, MY, MV, ML, MT, MQ, MR, MU, YT, MX, MD, MN, ME, MA, MZ, MM, NA, NP, NL, NC, NZ, NI, NE, NG, MK, NO, OM, PK, PS, PA, PG, PY, PE, PH, PL, PT, PR, QA, RE, RO, RU, RW, KN, LC, VC, WS, SA, SN, RS, SC, SL, SG, SK, SI, SB, SO, ZA, KR, SS, ES, LK, SD, SR, SE, CH, SY, TW, TJ, TZ, TH, TL, TG, TO, TT, TN, TR, TM, UG, UA, AE, GB, US, UY, UZ, VE, VN, YE, ZM, ZW.
    /// * `extract_markdown` - Extracts rendered page content as a structured .md (Markdown) file. Preserves headings, lists, links. Only with output=json.
    /// * `format` - Response format: json (default).
    /// * `get_image_urls` - Extracts all image URLs from the rendered DOM and returns them in structured format. Only with output=json.
    /// * `gpu` - Renders the page with a GPU, including WebGL and canvas content. Adds 20 credits per request.
    /// * `markdown` - Markdown to render instead of loading a URL. Works like custom_html, so you can capture a README or notes without hosting a page.
    /// * `ocr_model` - Recognition model used by the captcha solver: mini-ocr-v1 (default) or ocr-v1.
    /// * `pdf_options_format` - Paper size of the PDF. Applies when file_type is PDF. Without it, width and height are used if set, otherwise the page renders as one long page: A0, A1, A2, A3, A4, A5, A6, Letter, Legal, Tabloid, or Ledger.
    /// * `pdf_options_landscape` - Renders the PDF in landscape orientation, swapping the width and height of the paper size. Applies when file_type is PDF.
    /// * `pdf_options_media` - CSS media type used to render the PDF: print or screen.
    /// * `pdf_options_page_range` - Pages to keep in the PDF, as start-end, for example 1-3 or 5-5. Pages are rendered first and then trimmed. Has no effect on a single-page PDF. Applies when file_type is PDF.
    /// * `pdf_options_print_background` - Includes background colors and images in the PDF. Applies when file_type is PDF.
    /// * `premium_proxy` - Loads the page through a residential proxy. Adds 200 credits per request. Used instead of classic_proxy.
    /// * `scroll_delay` - Delay in milliseconds between scroll steps when `lazy_load` is enabled. Use higher values for heavy pages.
    /// * `selector_to_click` - CSS selector for an element to click before capturing the screenshot. Useful for opening dropdowns, modals, or triggering content.
    /// * `solve_captcha` - Solves a captcha made of text drawn in a picture, then takes the screenshot. Requires captcha_image and captcha_input. Adds 20 credits per request.
    /// * `stealth` - Makes the browser look like a regular visitor for pages that block automated traffic. No extra credits.
    /// * `timezone` - IANA timezone the browser uses while rendering, such as Asia/Karachi or Europe/London. Useful for clocks and localized dates.
    /// * `video` - Records visible viewport only for a specified duration. Use with `duration` parameter. Ideal for dashboards and animations.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .screenshot_capture(
    ///             &ScreenshotCaptureQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 url: "url".to_string(),
    ///                 output: None,
    ///                 file_type: None,
    ///                 width: None,
    ///                 height: None,
    ///                 full_page: None,
    ///                 fresh: None,
    ///                 no_cookie_banners: None,
    ///                 enable_caching: None,
    ///                 block_ads: None,
    ///                 block_chat_widgets: None,
    ///                 extract_text: None,
    ///                 extract_html: None,
    ///                 destroy_screenshot: None,
    ///                 lazy_load: None,
    ///                 retina: None,
    ///                 dark_mode: None,
    ///                 block_tracking: None,
    ///                 enable_incognito: None,
    ///                 omit_background: None,
    ///                 thumbnail_width: None,
    ///                 adjust_top: None,
    ///                 wait_for_event: None,
    ///                 grayscale: None,
    ///                 delay: None,
    ///                 timeout: None,
    ///                 ttl: None,
    ///                 clip_x: None,
    ///                 clip_y: None,
    ///                 clip_width: None,
    ///                 clip_height: None,
    ///                 css_url: None,
    ///                 css: None,
    ///                 js_url: None,
    ///                 js: None,
    ///                 block_js: None,
    ///                 block_stylesheets: None,
    ///                 block_images: None,
    ///                 block_media: None,
    ///                 block_font: None,
    ///                 block_text_track: None,
    ///                 block_xhr: None,
    ///                 block_fetch: None,
    ///                 block_event_source: None,
    ///                 block_web_socket: None,
    ///                 block_manifest: None,
    ///                 block_specific_requests: None,
    ///                 blur_selector: None,
    ///                 remove_selector: None,
    ///                 result_file_name: None,
    ///                 scrolling_screenshot: None,
    ///                 scroll_speed: None,
    ///                 scroll_back: None,
    ///                 start_immediately: None,
    ///                 multiple_scrolling: None,
    ///                 sizes: vec![],
    ///                 duration: None,
    ///                 fail_on_error: None,
    ///                 longitude: None,
    ///                 latitude: None,
    ///                 proxy: None,
    ///                 headers: None,
    ///                 cookies: None,
    ///                 scroll_to_element: None,
    ///                 selector: None,
    ///                 user_agent: None,
    ///                 accept_languages: None,
    ///                 custom_html: None,
    ///                 image_quality: None,
    ///                 captcha_error: None,
    ///                 captcha_image: None,
    ///                 captcha_input: None,
    ///                 captcha_refresh: None,
    ///                 captcha_retries: None,
    ///                 captcha_submit: None,
    ///                 classic_proxy: None,
    ///                 click_recursion: None,
    ///                 country: None,
    ///                 extract_markdown: None,
    ///                 format: None,
    ///                 get_image_urls: None,
    ///                 gpu: None,
    ///                 markdown: None,
    ///                 ocr_model: None,
    ///                 pdf_options_format: None,
    ///                 pdf_options_landscape: None,
    ///                 pdf_options_media: None,
    ///                 pdf_options_page_range: None,
    ///                 pdf_options_print_background: None,
    ///                 premium_proxy: None,
    ///                 scroll_delay: None,
    ///                 selector_to_click: None,
    ///                 solve_captcha: None,
    ///                 stealth: None,
    ///                 timezone: None,
    ///                 video: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn screenshot_capture(
        &self,
        request: &ScreenshotCaptureQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ScreenshotCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/screenshot",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("output", request.output.clone())
                    .serialize("file_type", request.file_type.clone())
                    .string("url", request.url.clone())
                    .int("width", request.width.clone())
                    .int("height", request.height.clone())
                    .bool("full_page", request.full_page.clone())
                    .bool("fresh", request.fresh.clone())
                    .bool("no_cookie_banners", request.no_cookie_banners.clone())
                    .bool("enable_caching", request.enable_caching.clone())
                    .bool("block_ads", request.block_ads.clone())
                    .bool("block_chat_widgets", request.block_chat_widgets.clone())
                    .bool("extract_text", request.extract_text.clone())
                    .bool("extract_html", request.extract_html.clone())
                    .bool("destroy_screenshot", request.destroy_screenshot.clone())
                    .bool("lazy_load", request.lazy_load.clone())
                    .bool("retina", request.retina.clone())
                    .bool("dark_mode", request.dark_mode.clone())
                    .bool("block_tracking", request.block_tracking.clone())
                    .bool("enable_incognito", request.enable_incognito.clone())
                    .bool("omit_background", request.omit_background.clone())
                    .int("thumbnail_width", request.thumbnail_width.clone())
                    .int("adjust_top", request.adjust_top.clone())
                    .serialize("wait_for_event", request.wait_for_event.clone())
                    .int("grayscale", request.grayscale.clone())
                    .int("delay", request.delay.clone())
                    .int("timeout", request.timeout.clone())
                    .int("ttl", request.ttl.clone())
                    .int("clip[x]", request.clip_x.clone())
                    .int("clip[y]", request.clip_y.clone())
                    .int("clip[width]", request.clip_width.clone())
                    .int("clip[height]", request.clip_height.clone())
                    .string("css_url", request.css_url.clone())
                    .string("css", request.css.clone())
                    .string("js_url", request.js_url.clone())
                    .string("js", request.js.clone())
                    .bool("block_js", request.block_js.clone())
                    .bool("block_stylesheets", request.block_stylesheets.clone())
                    .bool("block_images", request.block_images.clone())
                    .bool("block_media", request.block_media.clone())
                    .bool("block_font", request.block_font.clone())
                    .bool("block_text_track", request.block_text_track.clone())
                    .bool("block_xhr", request.block_xhr.clone())
                    .bool("block_fetch", request.block_fetch.clone())
                    .bool("block_event_source", request.block_event_source.clone())
                    .bool("block_web_socket", request.block_web_socket.clone())
                    .bool("block_manifest", request.block_manifest.clone())
                    .string(
                        "block_specific_requests",
                        request.block_specific_requests.clone(),
                    )
                    .string("blur_selector", request.blur_selector.clone())
                    .string("remove_selector", request.remove_selector.clone())
                    .string("result_file_name", request.result_file_name.clone())
                    .bool("scrolling_screenshot", request.scrolling_screenshot.clone())
                    .serialize("scroll_speed", request.scroll_speed.clone())
                    .bool("scroll_back", request.scroll_back.clone())
                    .bool("start_immediately", request.start_immediately.clone())
                    .bool("multiple_scrolling", request.multiple_scrolling.clone())
                    .string("sizes", {
                        let joined = request
                            .sizes
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .float("duration", request.duration.clone())
                    .bool("fail_on_error", request.fail_on_error.clone())
                    .float("longitude", request.longitude.clone())
                    .float("latitude", request.latitude.clone())
                    .string("proxy", request.proxy.clone())
                    .string("headers", request.headers.clone())
                    .string("cookies", request.cookies.clone())
                    .string("scroll_to_element", request.scroll_to_element.clone())
                    .string("selector", request.selector.clone())
                    .string("user_agent", request.user_agent.clone())
                    .string("accept_languages", request.accept_languages.clone())
                    .string("custom_html", request.custom_html.clone())
                    .float("image_quality", request.image_quality.clone())
                    .string("captcha_error", request.captcha_error.clone())
                    .string("captcha_image", request.captcha_image.clone())
                    .string("captcha_input", request.captcha_input.clone())
                    .string("captcha_refresh", request.captcha_refresh.clone())
                    .int("captcha_retries", request.captcha_retries.clone())
                    .string("captcha_submit", request.captcha_submit.clone())
                    .bool("classic_proxy", request.classic_proxy.clone())
                    .int("click_recursion", request.click_recursion.clone())
                    .string("country", request.country.clone())
                    .bool("extract_markdown", request.extract_markdown.clone())
                    .serialize("format", request.format.clone())
                    .bool("get_image_urls", request.get_image_urls.clone())
                    .bool("gpu", request.gpu.clone())
                    .string("markdown", request.markdown.clone())
                    .serialize("ocr_model", request.ocr_model.clone())
                    .serialize("pdf_options[format]", request.pdf_options_format.clone())
                    .bool(
                        "pdf_options[landscape]",
                        request.pdf_options_landscape.clone(),
                    )
                    .serialize("pdf_options[media]", request.pdf_options_media.clone())
                    .string(
                        "pdf_options[page_range]",
                        request.pdf_options_page_range.clone(),
                    )
                    .bool(
                        "pdf_options[print_background]",
                        request.pdf_options_print_background.clone(),
                    )
                    .bool("premium_proxy", request.premium_proxy.clone())
                    .int("scroll_delay", request.scroll_delay.clone())
                    .string("selector_to_click", request.selector_to_click.clone())
                    .bool("solve_captcha", request.solve_captcha.clone())
                    .bool("stealth", request.stealth.clone())
                    .string("timezone", request.timezone.clone())
                    .bool("video", request.video.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Our Bulk Screenshot API allows you to capture screenshots of multiple webpages simultaneously, saving you time and effort. Instead of manually capturing each page one by one, you can batch process URLs and receive high-quality screenshots in the format you choose.
    /// Maximum `50 URLs` per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_screenshot_capture(
    ///             &BulkScreenshotCaptureRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 urls: vec![BulkScreenshotCaptureRequestUrlsItem {
    ///                     url: "url".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_screenshot_capture(
        &self,
        request: &BulkScreenshotCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkScreenshotCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/screenshot",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get live forex rates for all world currencies with customizable update frequency
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `base` - Base currency for rate calculations
    /// * `symbols` - Comma separated list of desired currency codes
    /// * `updates` - Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_latest_rates(
    ///             &CurrencyLatestRatesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 base: None,
    ///                 symbols: vec![],
    ///                 updates: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_latest_rates(
        &self,
        request: &CurrencyLatestRatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyLatestRatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/rates/latest",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("base", request.base.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .serialize("updates", request.updates.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get historical exchange rates for any specific date
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `base` - Base currency for rate calculations
    /// * `symbols` - Comma separated list of desired currency codes
    /// * `date` - Specific date in YYYY-MM-DD format
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_historical_rates(
    ///             &CurrencyHistoricalRatesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///                 base: None,
    ///                 symbols: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_historical_rates(
        &self,
        request: &CurrencyHistoricalRatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyHistoricalRatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/rates/historical",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("base", request.base.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .date("date", request.date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Convert amount between currencies using the latest exchange rates
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `from` - Source currency code
    /// * `to` - Target currency code
    /// * `amount` - Amount to convert
    /// * `updates` - Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_convert_latest(
    ///             &CurrencyConvertLatestQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 from: "from".to_string(),
    ///                 to: "to".to_string(),
    ///                 format: None,
    ///                 amount: None,
    ///                 updates: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_convert_latest(
        &self,
        request: &CurrencyConvertLatestQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyConvertLatestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/converter/latest/prices",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("from", request.from.clone())
                    .string("to", request.to.clone())
                    .float("amount", request.amount.clone())
                    .serialize("updates", request.updates.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Convert amount between currencies using historical rates
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `from` - From currency symbol
    /// * `to` - To currency symbol
    /// * `amount` - The Amount to be converted
    /// * `date` - specific date (format YYYY-MM-DD) of which exchange rates is used.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_convert_historical(
    ///             &CurrencyConvertHistoricalQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 from: "from".to_string(),
    ///                 to: "to".to_string(),
    ///                 date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///                 amount: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_convert_historical(
        &self,
        request: &CurrencyConvertHistoricalQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyConvertHistoricalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/converter/historical/prices",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("from", request.from.clone())
                    .string("to", request.to.clone())
                    .float("amount", request.amount.clone())
                    .date("date", request.date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get exchange rates for a time range
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `start_date` - Start date (format YYYY-MM-DD) of the preferred time frame
    /// * `end_date` - End date (format YYYY-MM-DD) of the preferred time frame
    /// * `base` - Base currency
    /// * `symbols` - comma separated list of desired currencies/ commodities symbols
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_time_series(
    ///             &CurrencyTimeSeriesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///                 end_date: None,
    ///                 base: None,
    ///                 symbols: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_time_series(
        &self,
        request: &CurrencyTimeSeriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyTimeSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/time-series",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .string("base", request.base.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .build(),
                options,
            )
            .await
    }

    /// Get currency fluctuation data for a time period
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `start_date` - Start date (format YYYY-MM-DD) of the preferred time frame
    /// * `end_date` - End date (format YYYY-MM-DD) of the preferred time frame
    /// * `base` - Base currency
    /// * `symbols` - comma separated list of desired currencies/ commodities symbols
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_fluctuation(
    ///             &CurrencyFluctuationQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 base: Some("USD".to_string()),
    ///                 format: None,
    ///                 end_date: None,
    ///                 symbols: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_fluctuation(
        &self,
        request: &CurrencyFluctuationQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyFluctuationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/fluctuation",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .string("base", request.base.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .build(),
                options,
            )
            .await
    }

    /// Convert amount using user's location
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `updates` - Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    /// * `from` - From currency symbol
    /// * `ip` - IPv4 or IPv6 geolocated currency
    /// * `amount` - Amount to convert
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_convert_by_ip(
    ///             &CurrencyConvertByIpQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 from: "from".to_string(),
    ///                 format: None,
    ///                 updates: None,
    ///                 ip: None,
    ///                 amount: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_convert_by_ip(
        &self,
        request: &CurrencyConvertByIpQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyConvertByIpResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/converter/ip-to-currency",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .serialize("updates", request.updates.clone())
                    .string("from", request.from.clone())
                    .string("ip", request.ip.clone())
                    .float("amount", request.amount.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get list of all supported currencies with their metadata
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_supported(
    ///             &CurrencySupportedQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_supported(
        &self,
        request: &CurrencySupportedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencySupportedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/supported",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get currency symbols and codes
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_symbols(
    ///             &CurrencySymbolsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_symbols(
        &self,
        request: &CurrencySymbolsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencySymbolsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/symbols",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get information about historical data availability and limits
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .currency_historical_limits(
    ///             &CurrencyHistoricalLimitsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn currency_historical_limits(
        &self,
        request: &CurrencyHistoricalLimitsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrencyHistoricalLimitsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/currency/historical/data/limits",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get live commodity rates with customizable update frequency
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the Response
    /// * `symbols` - Comma separated list of desired commodities symbols *(e.g. XAU,XAG,WTI,BRENT)* **Required**
    /// * `updates` - Exchange rates update period. Possible values are: (1) `10m` - 10 minute update (2) `1m` - 1 minute update **Required**
    /// * `quote` - Specifies the target currency for the exchange rate; default quote currency is the market currency of commodity *(e.g. USD, EUR, INR)*
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_latest_rates(
    ///             &CommodityLatestRatesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 updates: CommodityLatestRatesRequestUpdates::TenM,
    ///                 format: None,
    ///                 quote: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_latest_rates(
        &self,
        request: &CommodityLatestRatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityLatestRatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/commodity/rates/latest",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .serialize("updates", Some(request.updates.clone()))
                    .string("quote", request.quote.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get historical commodity rates for a specific date
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `date` - Historical date (YYYY-MM-DD)
    /// * `symbols` - Comma-separated list of commodity symbols
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_historical_rates(
    ///             &CommodityHistoricalRatesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_historical_rates(
        &self,
        request: &CommodityHistoricalRatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityHistoricalRatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/commodity/rates/historical",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("date", request.date.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .build(),
                options,
            )
            .await
    }

    /// Get commodity price fluctuation data for a time period
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `symbols` - Comma-separated list of commodity symbols
    /// * `start_date` - Start date (YYYY-MM-DD)
    /// * `end_date` - End date (YYYY-MM-DD)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_fluctuation(
    ///             &CommodityFluctuationQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_fluctuation(
        &self,
        request: &CommodityFluctuationQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityFluctuationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/commodity/fluctuation",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get commodity rates for a time range
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `symbols` - Comma-separated list of commodity symbols
    /// * `start_date` - Start date (YYYY-MM-DD)
    /// * `end_date` - End date (YYYY-MM-DD)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_time_series(
    ///             &CommodityTimeSeriesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 symbols: vec![Some("symbols".to_string())],
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_time_series(
        &self,
        request: &CommodityTimeSeriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommodityTimeSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/commodity/time-series",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("symbols", {
                        let joined = request
                            .symbols
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get list of supported commodities
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .commodity_symbols(
    ///             &CommoditySymbolsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commodity_symbols(
        &self,
        request: &CommoditySymbolsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CommoditySymbolsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/commodity/symbols",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a list of supported countries.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response. Default is JSON.
    /// * `type_` - Which list to return: VAT (`VAT_Supported_Countries_And_States`), IBAN (`IBAN_Supported_Countries`) or SWIFT (`SWIFT_Supported_Countries`). Each value returns only its own list. If omitted (or unknown), all three lists are returned.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .vat_supported_countries(
    ///             &VatSupportedCountriesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 r#type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn vat_supported_countries(
        &self,
        request: &VatSupportedCountriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatSupportedCountriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/vat/supported-countries",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .serialize("type", request.r#type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Fetches VAT rate based on the specified or originating IP address.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specify the desired response format. Options: 'json' (default) or 'xml'.
    /// * `ip_address` - IPv4 or IPv6 address to look up VAT rate for. If omitted, the originating IP address will be used.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .vat_rate_by_ip(
    ///             &VatRateByIpQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 ip_address: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn vat_rate_by_ip(
        &self,
        request: &VatRateByIpQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<VatRateByIpResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/vat/rates/ip-address",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("ipAddress", request.ip_address.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Fetches VAT rates for a single country or state provided via query parameters.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specify the desired response format. Options: 'json' (default) or 'xml'.
    /// * `country` - Country identifier in Alpha-2 (PK), Alpha-3 (PAK), or full name (Pakistan). Combine with the optional "state" query for sub-national VAT; values are case-insensitive and may use underscores instead of spaces.
    /// * `state` - Optional state or region in Alpha-2 (NY) or full name (New_York). Use with "country" for state-level VAT; values are case-insensitive and may use underscores.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .vat_rate_by_country(
    ///             &VatRateByCountryQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///                 state: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn vat_rate_by_country(
        &self,
        request: &VatRateByCountryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<VatRateByCountryResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/vat/rates/country",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .string("state", request.state.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves VAT details for multiple countries or country-state combinations in a single request. Maximum of `100` entries per request are allowed.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specify the desired response format. Options: 'json' (default) or 'xml'.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_vat_rate_by_country(
    ///             &BulkVatRateByCountryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 countries: vec![
    ///                     BulkVatRateByCountryRequestCountriesItem {
    ///                         country: "PAK".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkVatRateByCountryRequestCountriesItem {
    ///                         country: "United_States".to_string(),
    ///                         state: Some("New_York".to_string()),
    ///                         ..Default::default()
    ///                     },
    ///                 ],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_vat_rate_by_country(
        &self,
        request: &BulkVatRateByCountryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<BulkVatRateByCountryResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/vat/rates/country",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates an EU or UK VAT number and returns registration status details.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specify the desired response format. Options: 'json' (default) or 'xml'.
    /// * `vat_number` - EU or UK VAT number to validate.
    /// * `requester_vat_number` - Requester EU or UK VAT number.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .vat_validate(
    ///             &VatValidateQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 vat_number: "vatNumber".to_string(),
    ///                 format: None,
    ///                 requester_vat_number: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn vat_validate(
        &self,
        request: &VatValidateQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatValidateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/vat/validation",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("vatNumber", request.vat_number.clone())
                    .string("requesterVatNumber", request.requester_vat_number.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Checks an IBAN for structural validity, checksum accuracy, and bank metadata.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specify the desired response format. Options: 'json' (default) or 'xml'.
    /// * `iban` - IBAN to validate.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .iban_validate(
    ///             &IbanValidateQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 iban: "iban".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn iban_validate(
        &self,
        request: &IbanValidateQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<IbanValidateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/iban/validation",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("iban", request.iban.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Fetches SWIFT codes for a given country, bank, and city.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specify the desired response format. Options: 'json' (default) or 'xml'.
    /// * `country` - Country name (accepts full name, e.g., Pakistan, United States). If only the country parameter is supplied, lists all banks in the country.
    /// * `bank` - Bank name (upper case) used to filter SWIFT codes. Should be used together with the country parameter. If only country and bank are provided (without city), returns the list of cities for that bank.
    /// * `city` - Gives SWIFT codes for a bank. Optionally specify the city (upper case) to narrow results to a specific city for that bank.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .swift_code_find(
    ///             &SwiftCodeFindQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 country: None,
    ///                 bank: None,
    ///                 city: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn swift_code_find(
        &self,
        request: &SwiftCodeFindQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<String>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/swift-code/finder",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .string("bank", request.bank.clone())
                    .string("city", request.city.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Fetches detailed information about a SWIFT code.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Specify the desired response format. Options: 'json' (default) or 'xml'.
    /// * `swift_code` - SWIFT/BIC code to lookup (must be 8 or 11 characters).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .swift_code_lookup(
    ///             &SwiftCodeLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 swift_code: "swiftCode".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn swift_code_lookup(
        &self,
        request: &SwiftCodeLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SwiftCodeLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/swift-code/lookup",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("swiftCode", request.swift_code.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .zipcode_lookup(
    ///             &ZipcodeLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 code: "code".to_string(),
    ///                 format: None,
    ///                 country: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn zipcode_lookup(
        &self,
        request: &ZipcodeLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ZipcodeLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/zipcode/lookup",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("code", request.code.clone())
                    .string("country", request.country.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates a bulk of ZIP/postal codes and returns result for each. Maximum `100` ZIP/postal codes per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_zipcode_lookup(
    ///             &BulkZipcodeLookupRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 codes: vec!["codes".to_string()],
    ///                 format: None,
    ///                 country: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_zipcode_lookup(
        &self,
        request: &BulkZipcodeLookupRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkZipcodeLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/zipcode/lookup",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .zipcode_search_by_city(
    ///             &ZipcodeSearchByCityQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 city: "city".to_string(),
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///                 state_name: None,
    ///                 page: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn zipcode_search_by_city(
        &self,
        request: &ZipcodeSearchByCityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ZipcodeSearchByCityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/zipcode/search/city",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("city", request.city.clone())
                    .string("country", request.country.clone())
                    .string("state_name", request.state_name.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .zipcode_search_by_region(
    ///             &ZipcodeSearchByRegionQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///                 region: None,
    ///                 page: None,
    ///                 region_code: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn zipcode_search_by_region(
        &self,
        request: &ZipcodeSearchByRegionQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ZipcodeSearchByRegionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/zipcode/search/region",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .string("region", request.region.clone())
                    .int("page", request.page.clone())
                    .string("region_code", request.region_code.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .zipcode_search_by_radius(
    ///             &ZipcodeSearchByRadiusQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 radius: 1.1,
    ///                 format: None,
    ///                 code: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 country: None,
    ///                 unit: None,
    ///                 page: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn zipcode_search_by_radius(
        &self,
        request: &ZipcodeSearchByRadiusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ZipcodeSearchByRadiusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/zipcode/search/radius",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("code", request.code.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("country", request.country.clone())
                    .float("radius", request.radius.clone())
                    .serialize("unit", request.unit.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get distance between postal codes. Maximum `100` postal codes per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .zipcode_distance(
    ///             &ZipcodeDistanceRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 compare: vec!["compare".to_string()],
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///                 code: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 unit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn zipcode_distance(
        &self,
        request: &ZipcodeDistanceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ZipcodeDistanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/zipcode/distance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get matching ZIP/postal code pairs within a specified distance. Maximum `100` postal codes per request.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .zipcode_distance_match(
    ///             &ZipcodeDistanceMatchRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 codes: vec!["codes".to_string()],
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///                 distance: None,
    ///                 unit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn zipcode_distance_match(
        &self,
        request: &ZipcodeDistanceMatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<ZipcodeDistanceMatchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/zipcode/distance/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get current weather data including temperature, humidity, precipitation, wind conditions, atmospheric pressure, and air quality for any location. Accepts city names, coordinates, or IP addresses. Also includes astronomy data and timezone-aware timestamps.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `location` - City name, place name, or full address.
    /// * `lat` - Latitude of the location.
    /// * `long` - Longitude of the location.
    /// * `ip` - IP(v4 or v6) address for location inference.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .current_weather(
    ///             &CurrentWeatherQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn current_weather(
        &self,
        request: &CurrentWeatherQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrentWeatherResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/weather/current",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve current weather conditions for up to `50 locations` in a single request. A maximum of 50 locations (city names, IP addresses, or geographic coordinates) can be included in the request body.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_current_weather(
    ///             &BulkCurrentWeatherRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 locations: vec![
    ///                     BulkCurrentWeatherRequestLocationsItem {
    ///                         location: Some("lahore".to_string()),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkCurrentWeatherRequestLocationsItem {
    ///                         lat: Some(32.5),
    ///                         long: Some(74.5),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkCurrentWeatherRequestLocationsItem {
    ///                         ip: Some("8.8.8.8".to_string()),
    ///                         ..Default::default()
    ///                     },
    ///                     BulkCurrentWeatherRequestLocationsItem {
    ///                         location: Some("seoul".to_string()),
    ///                         ..Default::default()
    ///                     },
    ///                 ],
    ///                 format: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_current_weather(
        &self,
        request: &BulkCurrentWeatherRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkCurrentWeatherResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/weather/current",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Access comprehensive weather forecasts with customizable precision - choose from daily overviews, hourly breakdowns, or even minute-by-minute data. Configure your date ranges or use the default 7-day forecast for standard weather planning.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `start_date` - Start date for the forecast in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between startDate and endDate must not exceed 16 days.
    /// * `end_date` - End date for the forecast in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between startDate and endDate must not exceed 16 days.
    /// * `forecast_days` - Number of days for the forecast, from 1 to 16. Default is 7. Maximum value is 16.
    /// * `location` - City name, place name, or full address.
    /// * `lat` - Latitude of the location.
    /// * `long` - Longitude of the location.
    /// * `ip` - IP(v4 or v6) address for location inference.
    /// * `precision` - Precision of the forecast data.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .weather_forecast(
    ///             &WeatherForecastQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 start_date: None,
    ///                 end_date: None,
    ///                 forecast_days: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 precision: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn weather_forecast(
        &self,
        request: &WeatherForecastQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WeatherForecastResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/weather/forecast",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .int("forecastDays", request.forecast_days.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .serialize("precision", request.precision.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Access past weather conditions for specific dates with records going back to 1940. Retrieve comprehensive historical data with both daily and hourly precision options.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `date` - Specific date for which to fetch weather data in YYYY-MM-DD format. Historical dates must be past dates only. Current or future dates are not allowed for historical data. Data available from 1940 onwards.
    /// * `location` - City name, place name, or full address.
    /// * `lat` - Latitude of the location.
    /// * `long` - Longitude of the location.
    /// * `ip` - IP(v4 or v6) address for location inference.
    /// * `precision` - Precision of the historical data. **Note:** 'daily' returns daily aggregates, 'hourly' returns hourly data.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .historical_weather(
    ///             &HistoricalWeatherQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 precision: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn historical_weather(
        &self,
        request: &HistoricalWeatherQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HistoricalWeatherResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/weather/historical",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("date", request.date.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .serialize("precision", request.precision.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Pull historical weather information for date ranges up to 90 days (daily data) or 7 days (hourly data). Get consistent formatting across your specified date range with reliable historical weather patterns.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `start_date` - Starting date for the data in YYYY-MM-DD format. Historical dates must be past dates only. Current or future dates are not allowed for historical data. Data available from 1940 onwards. For precision=daily, the difference between endDate and startDate must not exceed 90 days. For precision=hourly, the difference must not exceed 7 days.
    /// * `end_date` - End date for the data in YYYY-MM-DD format. Historical dates must be past dates only. Current or future dates are not allowed for historical data. Data available from 1940 onwards. For precision=daily, the difference between endDate and startDate must not exceed 90 days. For precision=hourly, the difference must not exceed 7 days.
    /// * `location` - City name, place name, or full address.
    /// * `lat` - Latitude of the location.
    /// * `long` - Longitude of the location.
    /// * `ip` - IP(v4 or v6) address for location inference.
    /// * `precision` - Precision of the data.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .weather_time_series(
    ///             &WeatherTimeSeriesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 precision: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn weather_time_series(
        &self,
        request: &WeatherTimeSeriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WeatherTimeSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/weather/time-series",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .serialize("precision", request.precision.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Provides hourly forecasts of marine conditions including wave heights, wave directions, wave periods, swell info, sea surface temperatures, and ocean currents. Supports multiple geographical points and returns daily max wave statistics for up to 7 days. Ideal for maritime planning, navigation, and coastal activities.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `start_date` - Starting date for marine forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    /// * `end_date` - End date for marine forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    /// * `location` - City name, place name, or full address.
    /// * `lat` - Latitude of the location.
    /// * `long` - Longitude of the location.
    /// * `ip` - IP(v4 or v6) address for location inference.
    /// * `precision` - Precision of the marine data.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .marine_weather(
    ///             &MarineWeatherQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 start_date: None,
    ///                 end_date: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 precision: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn marine_weather(
        &self,
        request: &MarineWeatherQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MarineWeatherResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/weather/marine",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .serialize("precision", request.precision.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Monitor and predict air quality conditions using European and US AQI standards. Track pollutant concentrations including PM10, PM2.5, carbon monoxide, nitrogen dioxide, sulfur dioxide, ozone, and dust particles. Get current readings plus hourly forecasts up to 5 days ahead, complete with UV index and aerosol measurements for comprehensive air quality assessment.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `start_date` - Starting date for AQI forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 5 days.
    /// * `end_date` - End date for AQI forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 5 days.
    /// * `location` - City name, place name, or full address.
    /// * `lat` - Latitude of the location.
    /// * `long` - Longitude of the location.
    /// * `ip` - IP(v4 or v6) address for location inference.
    /// * `precision` - Only hourly precision is supported; returns hourly AQI data for the selected date range.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .air_quality(
    ///             &AirQualityQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 start_date: None,
    ///                 end_date: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 precision: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn air_quality(
        &self,
        request: &AirQualityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AirQualityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/weather/air-quality",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .serialize("precision", request.precision.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Provides flood forecast data for a given location, including river discharge metrics such as mean, median, maximum, minimum, and percentile values (p25, p75). Requires a startDate and endDate, with the date range limited to 16 days. Location can be specified using city name, latitude/longitude, or IP address.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Response format returned by the API. Only JSON is supported.
    /// * `start_date` - Starting date for flood forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    /// * `end_date` - End date for flood forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    /// * `location` - City name, place name, or full address.
    /// * `lat` - Latitude of the location.
    /// * `long` - Longitude of the location.
    /// * `ip` - IP(v4 or v6) address for location inference.
    /// * `precision` - Only daily precision is supported; returns flood forecast data for the selected date range.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .flood_forecast(
    ///             &FloodForecastQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
    ///                 format: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 precision: None,
    ///                 timezone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn flood_forecast(
        &self,
        request: &FloodForecastQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FloodForecastResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/weather/flood",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .serialize("precision", request.precision.clone())
                    .string("timezone", request.timezone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve countries, optionally filtered by region or subregion.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response
    /// * `region` - Optional filter to return countries within a specific region from the region endpoint.
    /// * `subregion` - Optional filter to return countries within a specific subregion from the subregion endpoint.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_countries(
    ///             &GetCountriesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 region: None,
    ///                 subregion: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_countries(
        &self,
        request: &GetCountriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCountriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/countries",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("region", request.region.clone())
                    .string("subregion", request.subregion.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_country_details(
    ///             &GetCountryDetailsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_country_details(
        &self,
        request: &GetCountryDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCountryDetailsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/country/details",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_regions(
    ///             &GetRegionsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_regions(
        &self,
        request: &GetRegionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetRegionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/regions",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_subregions(
    ///             &GetSubregionsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 region: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_subregions(
        &self,
        request: &GetSubregionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSubregionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/subregions",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("region", request.region.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve administrative units based on ISO 3166-1 alpha-2 country code.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response
    /// * `country` - Country code in ISO 3166-1 alpha-2 format
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_admin_levels(
    ///             &GetAdminLevelsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_admin_levels(
        &self,
        request: &GetAdminLevelsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAdminLevelsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/admin-levels",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve administrative divisions for a given country using ISO 3166-1 alpha-2 country codes. You can optionally filter by administrative levels.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `country` - Country code in ISO 3166-1 alpha-2 format.
    /// * `admin_level` - Filter results by one or more administrative level names (e.g. `land`, `state`, `province`), sent as a comma-separated list. Use the admin levels endpoint to list the levels a country has.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_admin_units(
    ///             &GetAdminUnitsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///                 admin_level: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_admin_units(
        &self,
        request: &GetAdminUnitsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAdminUnitsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/admin-units",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .string("admin_level", {
                        let joined = request
                            .admin_level
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve detailed administrative unit information by country and optionally filtered by admin code.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `country` - Country code in ISO 3166-1 alpha-2 format.
    /// * `admin_unit` - Optional admin code to fetch details for a specific administrative unit.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_admin_unit_details(
    ///             &GetAdminUnitDetailsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 country: "country".to_string(),
    ///                 admin_unit: "admin_unit".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_admin_unit_details(
        &self,
        request: &GetAdminUnitDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAdminUnitDetailsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/admin-unit/details",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .string("admin_unit", request.admin_unit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a list of cities within a country, optionally filtered by an administrative unit code.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `country` - Country code in ISO 3166-1 alpha-2 format.
    /// * `admin_unit` - Administrative unit code used to filter cities within a specific region.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_cities(
    ///             &GetCitiesQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 country: "country".to_string(),
    ///                 format: None,
    ///                 admin_unit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_cities(
        &self,
        request: &GetCitiesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCitiesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geo/cities",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("country", request.country.clone())
                    .string("admin_unit", request.admin_unit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get list of all supported flags with their metadata
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_supported_flags(
    ///             &GetSupportedFlagsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_supported_flags(
        &self,
        request: &GetSupportedFlagsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSupportedFlagsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/flags/supported",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve the flag for a specific country
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `name` - Country code in ISO 3166-1 alpha-2 format.
    /// * `shape` - Flag shape. One of: `'flat'` or `'round'`.
    /// * `format` - Flag format. Applicable only for PNG or WEBP formats. Default is png.
    /// * `size` - Flag size in pixels. Valid options: `16px`, `24px`, `32px`, `48px`, `64px`. Applicable only for PNG or WEBP formats.
    /// * `type_` - Type of flag. One of: `country` or `organization`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_flags(
    ///             &GetFlagsQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 name: "name".to_string(),
    ///                 shape: GetFlagsRequestShape::Flat,
    ///                 r#type: GetFlagsRequestType::Country,
    ///                 format: None,
    ///                 size: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_flags(
        &self,
        request: &GetFlagsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                "v1.0/flags",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .string("name", request.name.clone())
                    .serialize("shape", Some(request.shape.clone()))
                    .serialize("format", request.format.clone())
                    .serialize("size", request.size.clone())
                    .serialize("type", Some(request.r#type.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve current time, date, and timezone-related information by specifying a timezone name, location address, location coordinates, IP address, or use the client IP address if no parameter is passed.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response
    /// * `ip` - IPv4 or IPv6 address to extract timezone information.
    /// * `tz` - Timezone name (e.g., "Asia/Kolkata") to retrieve information directly.
    /// * `location` - Location string (preferably city and country) to extract timezone.
    /// * `lat` - Latitude for geolocation lookup.
    /// * `long` - Longitude for geolocation lookup.
    /// * `lang` - Language code for response localization (default is "en").
    /// * `iata_code` - 3-letter IATA airport code (e.g., JFK).
    /// * `icao_code` - 4-letter ICAO airport code (e.g., KJFK).
    /// * `lo_code` - 5-letter UN/LO city code.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .timezone_lookup(
    ///             &TimezoneLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 ip: None,
    ///                 tz: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 lang: None,
    ///                 iata_code: None,
    ///                 icao_code: None,
    ///                 lo_code: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn timezone_lookup(
        &self,
        request: &TimezoneLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimezoneLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geolocation/timezone",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("ip", request.ip.clone())
                    .string("tz", request.tz.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .serialize("lang", request.lang.clone())
                    .string("iata_code", request.iata_code.clone())
                    .string("icao_code", request.icao_code.clone())
                    .string("lo_code", request.lo_code.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Converts a given time from one timezone to another using various input types like timezone name, coordinates, location, or codes.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response .
    /// * `time` - Time to convert in `yyyy-MM-dd HH:mm` or `yyyy-MM-dd HH:mm:ss` format.
    /// * `tz_from` - Source timezone name (e.g., `Asia/Kolkata`).
    /// * `tz_to` - Target timezone name (e.g., `America/New_York`).
    /// * `lat_from` - Latitude of source location.
    /// * `long_from` - Longitude of source location.
    /// * `lat_to` - Latitude of target location.
    /// * `long_to` - Longitude of target location.
    /// * `location_from` - From location (city/country).
    /// * `location_to` - To location (city/country).
    /// * `iata_from` - From IATA airport code (e.g., JFK).
    /// * `iata_to` - To IATA airport code.
    /// * `icao_from` - From ICAO airport code (e.g., KJFK).
    /// * `icao_to` - To ICAO airport code.
    /// * `locode_from` - From UN/LO CODE.
    /// * `locode_to` - To UN/LO CODE.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .timezone_convert(
    ///             &TimezoneConvertQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 time: None,
    ///                 tz_from: None,
    ///                 tz_to: None,
    ///                 lat_from: None,
    ///                 long_from: None,
    ///                 lat_to: None,
    ///                 long_to: None,
    ///                 location_from: None,
    ///                 location_to: None,
    ///                 iata_from: None,
    ///                 iata_to: None,
    ///                 icao_from: None,
    ///                 icao_to: None,
    ///                 locode_from: None,
    ///                 locode_to: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn timezone_convert(
        &self,
        request: &TimezoneConvertQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimezoneConvertResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/timezone/converter",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("time", request.time.clone())
                    .string("tz_from", request.tz_from.clone())
                    .string("tz_to", request.tz_to.clone())
                    .float("lat_from", request.lat_from.clone())
                    .float("long_from", request.long_from.clone())
                    .float("lat_to", request.lat_to.clone())
                    .float("long_to", request.long_to.clone())
                    .string("location_from", request.location_from.clone())
                    .string("location_to", request.location_to.clone())
                    .string("iata_from", request.iata_from.clone())
                    .string("iata_to", request.iata_to.clone())
                    .string("icao_from", request.icao_from.clone())
                    .string("icao_to", request.icao_to.clone())
                    .string("locode_from", request.locode_from.clone())
                    .string("locode_to", request.locode_to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Parse User Agent string to get detailed browser, device, and operating system information
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .user_agent_lookup(
    ///             &UserAgentLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn user_agent_lookup(
        &self,
        request: &UserAgentLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<UserAgentLookupResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        options
            .additional_headers
            .insert("User-Agent".to_string(), request.user_agent.clone());
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/user-agent/lookup",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                Some(options),
            )
            .await
    }

    /// Parse up to `20000 User-Agent strings` at once in a single request; exceeding that returns a 413, not a 400.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .bulk_user_agent_lookup(
    ///             &BulkUserAgentLookupRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 ua_strings: vec!["uaStrings".to_string()],
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_user_agent_lookup(
        &self,
        request: &BulkUserAgentLookupRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<BulkUserAgentLookupResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/user-agent/lookup",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Analyze text for grammar errors and return the exact words flagged as grammatically incorrect with zero-based word positions.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client.grammar_detect(&GrammarDetectRequest {
    ///         api_key: "apiKey".to_string(),
    ///         text: "The global mental is health crisis is now a serious and compelex problem. It need quick and ongoing action from policymakers, healthcare workers, and the whole society.".to_string()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn grammar_detect(
        &self,
        request: &GrammarDetectRequest,
        options: Option<RequestOptions>,
    ) -> Result<GrammarDetectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/readability/grammar/detect",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Submit text with grammatical issues and receive a clean grammar-corrected result for proofreading and content workflows.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client.grammar_correct(&GrammarCorrectRequest {
    ///         api_key: "apiKey".to_string(),
    ///         text: "The global mental is health crisis is now a serious and compelex problem. It need quick and ongoing action from policymakers, healthcare workers, and the whole society.".to_string()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn grammar_correct(
        &self,
        request: &GrammarCorrectRequest,
        options: Option<RequestOptions>,
    ) -> Result<GrammarCorrectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/readability/grammar/correct",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Analyze text and return weak, vague, or filler words with zero-based word positions to help writers produce clearer and more concise content.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .weak_words_detect(
    ///             &WeakWordsDetectRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 text:
    ///                     "Many people cannot get the support they need to handle their conditions well."
    ///                         .to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn weak_words_detect(
        &self,
        request: &WeakWordsDetectRequest,
        options: Option<RequestOptions>,
    ) -> Result<WeakWordsDetectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/readability/weak-words",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Analyze text readability using industry-standard formulas including Flesch Reading Ease, Flesch-Kincaid Grade Level, Gunning Fog Index, SMOG Index, Coleman-Liau Index, and Automated Readability Index.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `target` - Target audience used to tune sentence difficulty levels
    /// * `exclude` - Comma-separated response sections to omit. Possible values are readability_scores, sentence_readability, readability_grade
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client.readability_score(&ReadabilityScoreRequest {
    ///         api_key: "apiKey".to_string(),
    ///         text: "The global mental is health crisis is now a serious and compelex problem. It needs quick and ongoing action from policymakers, healthcare workers, and the whole society.".to_string(),
    ///         target: None,
    ///         exclude: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn readability_score(
        &self,
        request: &ReadabilityScoreRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReadabilityScoreResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/readability/score",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("target", request.target.clone())
                    .string("exclude", request.exclude.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve sunrise and sunset times, current position of the moon, and other related information by specifying a location address, location coordinates, IP address, or using the client IP address if no parameter is passed.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response.
    /// * `location` - Location name or address
    /// * `lat` - Latitude for location coordinates
    /// * `long` - Longitude for location coordinates
    /// * `ip` - IP address for location detection
    /// * `date` - Date for astronomy data (YYYY-MM-DD)
    /// * `elevation` - Timezone of the location for which astronomy data is required
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .astronomy_lookup(
    ///             &AstronomyLookupQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///                 location: None,
    ///                 lat: None,
    ///                 long: None,
    ///                 ip: None,
    ///                 lang: None,
    ///                 date: None,
    ///                 elevation: None,
    ///                 time_zone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn astronomy_lookup(
        &self,
        request: &AstronomyLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AstronomyLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/geolocation/astronomy",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .string("location", request.location.clone())
                    .float("lat", request.lat.clone())
                    .float("long", request.long.clone())
                    .string("ip", request.ip.clone())
                    .string("lang", request.lang.clone())
                    .date("date", request.date.clone())
                    .float("elevation", request.elevation.clone())
                    .string("time_zone", request.time_zone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns subscription, subscription-surcharge, and one-off credit balances for the authenticated organization's account.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `format` - Format of the response. Possible values: json, xml.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apifreaks_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiFreaks::new(config).expect("Failed to build client");
    ///     client
    ///         .get_credits_usage(
    ///             &GetCreditsUsageQueryRequest {
    ///                 api_key: "apiKey".to_string(),
    ///                 format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_credits_usage(
        &self,
        request: &GetCreditsUsageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCreditsUsageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/credits/usage/info",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }
}
