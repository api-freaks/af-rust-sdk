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
    pub async fn geocoder_search(
        &self,
        request: &GeocoderSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GeocoderSearchResponseItem>, ApiError> {
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
                options,
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
    pub async fn geocoder_reverse(
        &self,
        request: &GeocoderReverseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeocoderReverseResponse, ApiError> {
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
                options,
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
    /// * `host_name` - Hostname or URL whose DNS records are required.
    /// * `ip_address` - The IP address for requested DNS's PTR record. 'type' parameter must be set to 'all'.
    /// * `type_` - A comma-separated list of DNS record types for lookup. Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all. When ipAddress is provided, type must be "all".
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
                    .string_array("type", request.r#type.clone())
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
    /// * `type_` - A comma-separated list of DNS record types for lookup.
    /// Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
                    .string_array("type", request.r#type.clone())
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
    /// * `type_` - A comma-separated list of DNS record types for lookup.
    /// Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all
    /// * `page` - Page number for paginated results
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
                    .string_array("type", request.r#type.clone())
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
    /// * `proxy` - Use proxy for requests
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
    /// * `count` - Number of suggestions to retrieve.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
    pub async fn pdf_merge(
        &self,
        request: &PdfMergeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfMergeResponse, ApiError> {
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
                options,
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
    pub async fn pdf_remove_pages(
        &self,
        request: &PdfRemovePagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfRemovePagesResponse, ApiError> {
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
                options,
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
    pub async fn pdf_split(
        &self,
        request: &PdfSplitRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfSplitResponse, ApiError> {
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
                options,
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
    pub async fn pdf_rotate(
        &self,
        request: &PdfRotateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfRotateResponse, ApiError> {
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
                options,
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
    pub async fn pdf_compress(
        &self,
        request: &PdfCompressRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfCompressResponse, ApiError> {
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
                options,
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
    pub async fn pdf_extract_pages(
        &self,
        request: &PdfExtractPagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfExtractPagesResponse, ApiError> {
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
                options,
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
    pub async fn pdf_linearize(
        &self,
        request: &PdfLinearizeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfLinearizeResponse, ApiError> {
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
                options,
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
    pub async fn pdf_encrypt(
        &self,
        request: &PdfEncryptRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfEncryptResponse, ApiError> {
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
                options,
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
    pub async fn pdf_decrypt(
        &self,
        request: &PdfDecryptRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfDecryptResponse, ApiError> {
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
                options,
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
    pub async fn pdf_restrict(
        &self,
        request: &PdfRestrictRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfRestrictResponse, ApiError> {
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
                options,
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
    pub async fn pdf_unrestrict(
        &self,
        request: &PdfUnrestrictRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfUnrestrictResponse, ApiError> {
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
                options,
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
    pub async fn pdf_convert_to_png(
        &self,
        request: &PdfConvertToPngRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToPngResponse, ApiError> {
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
                options,
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
    pub async fn pdf_convert_to_jpg(
        &self,
        request: &PdfConvertToJpgRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToJpgResponse, ApiError> {
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
                options,
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
    pub async fn pdf_convert_to_tiff(
        &self,
        request: &PdfConvertToTiffRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToTiffResponse, ApiError> {
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
                options,
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
    pub async fn pdf_convert_to_bmp(
        &self,
        request: &PdfConvertToBmpRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToBmpResponse, ApiError> {
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
                options,
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
    pub async fn pdf_convert_to_gif(
        &self,
        request: &PdfConvertToGifRequest,
        options: Option<RequestOptions>,
    ) -> Result<PdfConvertToGifResponse, ApiError> {
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
                options,
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

    /// Capture full-page screenshots and videos of websites with advanced options like device simulation, custom code injection, cookie banner blocking, and scrollable content recording.
    /// Supports multiple output formats including JSON, image, GIF, MP4, and WebM.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `output` - Output format for screenshot results
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
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn screenshot_capture(
        &self,
        request: &ScreenshotCaptureQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
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
                    .string_array("sizes", request.sizes.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
                    .string_array("symbols", request.symbols.clone())
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
    /// * `type_` - Type of supported country. Supported values: IBAN, SWIFT, VAT. By default, it returns all supported countries for all types.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
    pub async fn bulk_vat_rate_by_country(
        &self,
        request: &BulkVatRateByCountryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BulkVatRateByCountryResponse, ApiError> {
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
                    .build(),
                options,
            )
            .await
    }

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
    /// * `format` - Response format returned by the API.
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
    /// * `format` - Response format returned by the API.
    /// * `timezone` - Timezone for the results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
    /// * `format` - Response format returned by the API.
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
    /// * `format` - Response format returned by the API.
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
    /// * `format` - Response format returned by the API.
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
    /// * `format` - Response format returned by the API.
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
    /// * `format` - Response format returned by the API.
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
    /// * `format` - Response format returned by the API.
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
    /// * `admin_levels` - Comma-separated list to filter results by one or more administrative levels.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
                    .string_array("adminLevels", request.admin_levels.clone())
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
    pub async fn get_supported_flags(
        &self,
        request: &GetSupportedFlagsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetSupportedFlagsResponseItem>, ApiError> {
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
    pub async fn user_agent_lookup(
        &self,
        request: &UserAgentLookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<UserAgentLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1.0/user-agent/lookup",
                None,
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Parse up to `50,000 User-Agent strings` at once in a single request.
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

    /// Perform Optical Character Recognition (OCR) on images, PDFs, or ZIP archives. Supports two models: `mini-ocr-v1` for CAPTCHA-optimized OCR and `ocr-v1` for general-purpose document text extraction. Supports zonal OCR to extract text from specific regions of an image.
    ///
    /// **Notes:**
    /// - The `zone` query parameter cannot be given with .pdf and .zip types as it can only be applied to single image query.
    /// - The `page_range` query parameter cannot be given in any other type except .pdf types.
    /// - PDFs containing images in them are allowed only for processing.
    /// - The `mini-ocr-v1` model doesn’t support the following query parameters:
    /// - `page_range` (.pdf types)
    /// - `zone`
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key
    /// * `url` - URL of the image or PDF (required if `file` not provided)
    /// * `model` - OCR model to use.
    /// * `page_range` - Specify page range for multi-page PDFs (e.g., '1,3,5-10' or 'allpages'). **Note:** This parameter can only be used with .pdf file types.
    /// * `zone` - Define OCR zones using coordinates (top:left:height:width). Multiple zones can be defined using commas. Only available for model 'ocr-v1'. **Note:** This parameter cannot be used with .pdf and .zip file types as it can only be applied to single image queries.
    /// * `new_line` - Set to 1 to split output text into individual lines (default: 0)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn ocr_predict(
        &self,
        request: &OcrPredictRequest,
        options: Option<RequestOptions>,
    ) -> Result<OcrPredictResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1.0/ocr/predict",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("apiKey", request.api_key.clone())
                    .string("url", request.url.clone())
                    .serialize("model", Some(request.model.clone()))
                    .string("page_range", request.page_range.clone())
                    .string("zone", request.zone.clone())
                    .int("new_line", request.new_line.clone())
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
}
