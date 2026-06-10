# Reference
<details><summary><code>client.<a href="/src/client.rs">geolocation_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GeolocationLookupRequestFormat&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, lang: Option&lt;Option&lt;GeolocationLookupRequestLang&gt;&gt;, fields: Option&lt;Option&lt;String&gt;&gt;, excludes: Option&lt;Option&lt;String&gt;&gt;, include: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GeolocationLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get detailed geolocation data for an IP address including country, city, timezone, currency, and optional security and user-agent information
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .geolocation_lookup(
            &GeolocationLookupQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                ip: None,
                lang: None,
                fields: None,
                excludes: None,
                include: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GeolocationLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IPv4, IPv6, or hostname for geolocation lookup
    
</dd>
</dl>

<dl>
<dd>

**lang:** `Option<GeolocationLookupRequestLang>` — Response language for location fields
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — Comma separated list of fields to include in response
    
</dd>
</dl>

<dl>
<dd>

**excludes:** `Option<String>` — Comma separated list of fields to exclude from response
    
</dd>
</dl>

<dl>
<dd>

**include:** `Option<String>` — Additional data to include (location, network, security, currency, time_zone, user_agent, country_metadata , hostname, liveHostname, hostnameFallbackLivet)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_geolocation_lookup</a>(request: BulkGeolocationLookupRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkGeolocationLookupRequestFormat&gt;&gt;, lang: Option&lt;Option&lt;String&gt;&gt;, fields: Option&lt;Option&lt;String&gt;&gt;, excludes: Option&lt;Option&lt;String&gt;&gt;, include: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;BulkGeolocationLookupResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed geolocation data for multiple IP addresses in a single request.
Supports up to `50,000` IP-addresses/host-names per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_geolocation_lookup(
            &BulkGeolocationLookupRequest {
                api_key: "apiKey".to_string(),
                ips: vec!["ips".to_string()],
                format: None,
                lang: None,
                fields: None,
                excludes: None,
                include: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ips:** `Vec<String>` — List of IP addresses or hostnames to lookup
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkGeolocationLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**lang:** `Option<String>` — Language of the response.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — Comma-separated list of fields to include in the response. Can include "geo".
    
</dd>
</dl>

<dl>
<dd>

**excludes:** `Option<String>` — Comma-separated list of fields to exclude from the response (except "ip").
    
</dd>
</dl>

<dl>
<dd>

**include:** `Option<String>` — Comma-separated list of additional information to include in the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">ip_security_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;IpSecurityLookupRequestFormat&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, fields: Option&lt;Option&lt;String&gt;&gt;, excludes: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;IpSecurityLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get comprehensive security information for a given IP address. Detects VPNs, proxies, Tor nodes, and other security threats.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .ip_security_lookup(
            &IPSecurityLookupQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                ip: None,
                fields: None,
                excludes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<IpSecurityLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — A valid IPv4 or IPv6 address to look up. If omitted, the API uses the public IP of the requesting client.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — Comma-separated list of fields to return. Supports dot notation (e.g. security.threat_score).
    
</dd>
</dl>

<dl>
<dd>

**excludes:** `Option<String>` — Comma-separated list of fields to remove from the response. Supports dot notation (e.g. security.is_tor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_ip_security_lookup</a>(request: BulkIpSecurityLookupRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkIpSecurityLookupRequestFormat&gt;&gt;, fields: Option&lt;Option&lt;String&gt;&gt;, excludes: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;BulkIpSecurityLookupResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The Bulk IP Security Lookup API allows you to retrieve security details for up to `50,000` IP-addresses in a single request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_ip_security_lookup(
            &BulkIPSecurityLookupRequest {
                api_key: "apiKey".to_string(),
                ips: vec!["ips".to_string()],
                format: None,
                fields: None,
                excludes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ips:** `Vec<String>` — List of IP addresses to lookup
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkIpSecurityLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — Comma-separated list of fields to return. Supports dot notation (e.g. security.threat_score).
    
</dd>
</dl>

<dl>
<dd>

**excludes:** `Option<String>` — Comma-separated list of fields to remove from the response. Supports dot notation (e.g. security.is_tor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">geocoder_search</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GeocoderSearchRequestFormat&gt;&gt;, query: Option&lt;String&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, min_lat: Option&lt;Option&lt;String&gt;&gt;, max_lat: Option&lt;Option&lt;String&gt;&gt;, min_lon: Option&lt;Option&lt;String&gt;&gt;, max_lon: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;GeocoderSearchResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Convert a given address or place name into geographic coordinates (latitude and longitude).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .geocoder_search(
            &GeocoderSearchQueryRequest {
                api_key: "apiKey".to_string(),
                query: "query".to_string(),
                format: None,
                limit: None,
                min_lat: None,
                max_lat: None,
                min_lon: None,
                max_lon: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GeocoderSearchRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**query:** `String` — Free-form search query, e.g. Wembley Stadium, London
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Max number of results to return (1–40). May return fewer if matches are weak.
    
</dd>
</dl>

<dl>
<dd>

**min_lat:** `Option<String>` — Minimum latitude for the viewbox. Must be ≤ max_lat and between -90 and 90.
    
</dd>
</dl>

<dl>
<dd>

**max_lat:** `Option<String>` — Maximum latitude for the viewbox. Must be ≥ min_lat and between -90 and 90.
    
</dd>
</dl>

<dl>
<dd>

**min_lon:** `Option<String>` — Minimum longitude for the viewbox. Must be ≤ max_lon and between -180 and 180.
    
</dd>
</dl>

<dl>
<dd>

**max_lon:** `Option<String>` — Maximum longitude for the viewbox. Must be ≥ min_lon and between -180 and 180.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">geocoder_reverse</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GeocoderReverseRequestFormat&gt;&gt;, lat: Option&lt;f64&gt;, lon: Option&lt;f64&gt;) -> Result&lt;GeocoderReverseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Convert geographic coordinates (latitude and longitude) into a human-readable address or place name.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .geocoder_reverse(
            &GeocoderReverseQueryRequest {
                api_key: "apiKey".to_string(),
                lat: 1.1,
                lon: 1.1,
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GeocoderReverseRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `f64` — WGS84 latitude value ranging from -90 to 90.
    
</dd>
</dl>

<dl>
<dd>

**lon:** `f64` — WGS84 longitude value ranging from -180 to 180.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_whois_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainWhoisLookupRequestFormat&gt;&gt;, domain_name: Option&lt;String&gt;) -> Result&lt;DomainWhoisLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve current WHOIS information for a domain name.
This endpoint provides detailed registration information including registrar details,
dates, nameservers, and registrant information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_whois_lookup(
            &DomainWhoisLookupQueryRequest {
                api_key: "apiKey".to_string(),
                domain_name: "domainName".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainWhoisLookupRequestFormat>` — Response format (defaults to json)
    
</dd>
</dl>

<dl>
<dd>

**domain_name:** `String` — Domain name for WHOIS lookup
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_domain_whois_lookup</a>(request: BulkDomainWhoisLookupRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkDomainWhoisLookupRequestFormat&gt;&gt;) -> Result&lt;BulkDomainWhoisLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve WHOIS information for `100 Domains per Request`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_domain_whois_lookup(
            &BulkDomainWhoisLookupRequest {
                api_key: "apiKey".to_string(),
                domain_names: vec!["domainNames".to_string()],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_names:** `Vec<String>` — A list of domain names for which WHOIS data is requested.
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkDomainWhoisLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">ip_whois_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;IpWhoisLookupRequestFormat&gt;&gt;, ip: Option&lt;String&gt;) -> Result&lt;IpWhoisLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns WHOIS registration details for a specified IP address (IPv4 or IPv6).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .ip_whois_lookup(
            &IPWhoisLookupQueryRequest {
                api_key: "apiKey".to_string(),
                ip: "ip".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<IpWhoisLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `String` — The IP address (IPv4 or IPv6) for which WHOIS data is requested.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">asn_whois_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;AsnWhoisLookupRequestFormat&gt;&gt;, asn: Option&lt;String&gt;) -> Result&lt;AsnWhoisLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns WHOIS registration details for a specified ASN, with or without the 'as' prefix.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .asn_whois_lookup(
            &AsnWhoisLookupQueryRequest {
                api_key: "apiKey".to_string(),
                asn: "asn".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<AsnWhoisLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**asn:** `String` — The Autonomous System Number (ASN) to retrieve WHOIS data for. Can be prefixed with 'as' or not.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_whois_history</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainWhoisHistoryRequestFormat&gt;&gt;, domain_name: Option&lt;String&gt;) -> Result&lt;DomainWhoisHistoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve historical WHOIS records for a domain name.
This endpoint provides a timeline of all recorded changes in domain registration information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_whois_history(
            &DomainWhoisHistoryQueryRequest {
                api_key: "apiKey".to_string(),
                domain_name: "domainName".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainWhoisHistoryRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**domain_name:** `String` — Domain name for historical WHOIS lookup
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_whois_reverse</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainWhoisReverseRequestFormat&gt;&gt;, keyword: Option&lt;Option&lt;String&gt;&gt;, email: Option&lt;Option&lt;String&gt;&gt;, owner: Option&lt;Option&lt;String&gt;&gt;, company: Option&lt;Option&lt;String&gt;&gt;, exact: Option&lt;Option&lt;bool&gt;&gt;, mode: Option&lt;Option&lt;DomainWhoisReverseRequestMode&gt;&gt;, page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;DomainWhoisReverseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Performs a reverse WHOIS search using one or more search parameters like keyword, email, owner, or company.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_whois_reverse(
            &DomainWhoisReverseQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                keyword: None,
                email: None,
                owner: None,
                company: None,
                exact: None,
                mode: None,
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainWhoisReverseRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**keyword:** `Option<String>` — Keyword search term for reverse WHOIS by keyword (case-insensitive pattern matching).
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Email search term for reverse WHOIS by email address (case-insensitive exact or regex match; * wildcard supported).
    
</dd>
</dl>

<dl>
<dd>

**owner:** `Option<String>` — Registrant or owner name for reverse WHOIS (a full-text search phrase matching technique to retrieve results).
    
</dd>
</dl>

<dl>
<dd>

**company:** `Option<String>` — Organization or company name for reverse WHOIS (full-text search phrase matching technique to retrieve results).
    
</dd>
</dl>

<dl>
<dd>

**exact:** `Option<bool>` — Accepts 'true' or 'false'. "true" returns only records that exactly match the input (keyword, owner/registrant, or company). "false" returns all matches and is the default when omitted.
    
</dd>
</dl>

<dl>
<dd>

**mode:** `Option<DomainWhoisReverseRequestMode>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — Page number for paginated results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_dns_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainDnsLookupRequestFormat&gt;&gt;, host_name: Option&lt;Option&lt;String&gt;&gt;, ip_address: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;DomainDnsLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve real-time DNS records for any hostname. Supports multiple record types including A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME records.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_dns_lookup(
            &DomainDNSLookupQueryRequest {
                api_key: "apiKey".to_string(),
                r#type: vec![Some("type".to_string())],
                format: None,
                host_name: None,
                ip_address: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainDnsLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**host_name:** `Option<String>` — Hostname or URL whose DNS records are required.
    
</dd>
</dl>

<dl>
<dd>

**ip_address:** `Option<String>` — The IP address for requested DNS's PTR record. 'type' parameter must be set to 'all'.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — A comma-separated list of DNS record types for lookup. Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all. When ipAddress is provided, type must be "all".
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_domain_dns_lookup</a>(request: BulkDomainDnsLookupRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkDomainDnsLookupRequestFormat&gt;&gt;) -> Result&lt;BulkDomainDnsLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Perform DNS lookups for multiple hostnames in a single request. Supports up to `100 host-names per request`
and returns DNS records including A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME records.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_domain_dns_lookup(
            &BulkDomainDNSLookupRequest {
                api_key: "apiKey".to_string(),
                r#type: vec![Some("type".to_string())],
                domain_names: vec!["domainNames".to_string()],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_names:** `Vec<String>` — List of hostnames to lookup DNS records for
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkDomainDnsLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` 

A comma-separated list of DNS record types for lookup.
Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_dns_history</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainDnsHistoryRequestFormat&gt;&gt;, host_name: Option&lt;String&gt;, page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;DomainDnsHistoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve historical DNS records for any hostname. Access unique historical data for A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME records,
including subdomains. Results are paginated with up to 100 unique records per page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_dns_history(
            &DomainDNSHistoryQueryRequest {
                api_key: "apiKey".to_string(),
                host_name: "host-name".to_string(),
                r#type: vec![Some("type".to_string())],
                format: None,
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainDnsHistoryRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**host_name:** `String` — Hostname or URL whose historical DNS records are required
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` 

A comma-separated list of DNS record types for lookup.
Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — Page number for paginated results
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_dns_reverse</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainDnsReverseRequestFormat&gt;&gt;, type_: Option&lt;DomainDnsReverseRequestType&gt;, value: Option&lt;String&gt;, exact: Option&lt;Option&lt;bool&gt;&gt;, page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;DomainDnsReverseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all the hostnames associated with any particular A, AAAA, MX, NS, SOA, SPF, TXT, and CNAME DNS records. For instance, you can access all the hostnames hosted on any IP/CIDR notation, all the domain names using Cloudflare name servers, and all the domain names using Google Mailbox
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_dns_reverse(
            &DomainDNSReverseQueryRequest {
                api_key: "apiKey".to_string(),
                r#type: DomainDNSReverseRequestType::A,
                value: "value".to_string(),
                format: None,
                exact: None,
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainDnsReverseRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `DomainDnsReverseRequestType` 

The type of reverse DNS lookup to perform. Determines how the value parameter is interpreted:
- A: IPv4 CIDR block
- AAAA: IPv6 CIDR block
- MX: Mail provider domain
- NS: Name server provider hostname
- SOA: SOA record admin domain
- SPF/TXT: Target verification strings
- CNAME: Target hostname
    
</dd>
</dl>

<dl>
<dd>

**value:** `String` — Provide an IP or CIDR for A/AAAA lookups, or a hostname/selector for MX, NS, SOA, SPF, TXT, and CNAME queries. Wildcard regex patterns are also supported (e.g., mail.google.com, m*.google.com, _spf.g*.com, s*.g*.com).
    
</dd>
</dl>

<dl>
<dd>

**exact:** `Option<bool>` — Accepts 'true' or 'false'. "true" returns only records that exactly match the input (NS, MX, CNAME, SOA, SPF, TXT). "false" returns all matches (default when omitted).
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — Page number to paginate through results (defaults to 1).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">web_scrape</a>(request: WebScrapeRequestBody, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;WebScrapeRequestFormat&gt;&gt;, url: Option&lt;String&gt;, text: Option&lt;Option&lt;bool&gt;&gt;, js_enabled: Option&lt;Option&lt;bool&gt;&gt;, proxy: Option&lt;Option&lt;WebScrapeRequestProxy&gt;&gt;, ssl_ignore: Option&lt;Option&lt;bool&gt;&gt;, window_size: Option&lt;Option&lt;String&gt;&gt;, ad_block: Option&lt;Option&lt;bool&gt;&gt;, captcha: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;WebScrapeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Execute a series of web scraping instructions on a target URL. 
Supports various operations like form filling, clicking, data extraction, and CAPTCHA solving.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client.web_scrape(&WebScrapeRequest {
        api_key: "apiKey".to_string(),
        url: "https://example.com".to_string(),
        body: WebScrapeRequestBody::WebScrapeRequestBodyBlockURL(WebScrapeRequestBodyBlockURL {
            block_url: Some(vec!["https://example.com/ads.js".to_string(), "https://tracker.example.com/*".to_string()]),
            cookies: Some(vec![WebScrapeRequestBodyBlockURLCookiesItem {
                name: "sessionid".to_string(),
                value: "abc123".to_string(),
                ..Default::default()
            }, WebScrapeRequestBodyBlockURLCookiesItem {
                name: "user_pref".to_string(),
                value: "darkmode".to_string(),
                ..Default::default()
            }]),
            instructions: vec![WebScrapeRequestBodyBlockURLInstructionsItem::WebScrapeRequestBodyBlockURLInstructionsItemFill(WebScrapeRequestBodyBlockURLInstructionsItemFill {
                fill: Some(WebScrapeRequestBodyBlockURLInstructionsItemFillFill {
                    place: "#username".to_string(),
                    value: "myuser".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }), WebScrapeRequestBodyBlockURLInstructionsItem::WebScrapeRequestBodyBlockURLInstructionsItemFill(WebScrapeRequestBodyBlockURLInstructionsItemFill {
                fill: Some(WebScrapeRequestBodyBlockURLInstructionsItemFillFill {
                    place: "#password".to_string(),
                    value: "mypassword".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }), WebScrapeRequestBodyBlockURLInstructionsItem::WebScrapeRequestBodyBlockURLInstructionsItemFill(WebScrapeRequestBodyBlockURLInstructionsItemFill {
                ..Default::default()
            }), WebScrapeRequestBodyBlockURLInstructionsItem::WebScrapeRequestBodyBlockURLInstructionsItemFill(WebScrapeRequestBodyBlockURLInstructionsItemFill {
                ..Default::default()
            }), WebScrapeRequestBodyBlockURLInstructionsItem::WebScrapeRequestBodyBlockURLInstructionsItemFill(WebScrapeRequestBodyBlockURLInstructionsItemFill {
                ..Default::default()
            }), WebScrapeRequestBodyBlockURLInstructionsItem::WebScrapeRequestBodyBlockURLInstructionsItemFill(WebScrapeRequestBodyBlockURLInstructionsItemFill {
                ..Default::default()
            }), WebScrapeRequestBodyBlockURLInstructionsItem::WebScrapeRequestBodyBlockURLInstructionsItemFill(WebScrapeRequestBodyBlockURLInstructionsItemFill {
                ..Default::default()
            })],
            ..Default::default()
        }),
        format: None,
        text: None,
        js_enabled: None,
        proxy: None,
        ssl_ignore: None,
        window_size: None,
        ad_block: None,
        captcha: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<WebScrapeRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — Target URL to scrape
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<bool>` — Set to `true` to return the data in text format else `false` for data in html format with tags.
    
</dd>
</dl>

<dl>
<dd>

**js_enabled:** `Option<bool>` 

Set  `true` to handle websites with JavaScript. Set `false` to handle static html websites.


 Default value is `true`.
    
</dd>
</dl>

<dl>
<dd>

**proxy:** `Option<WebScrapeRequestProxy>` — Use proxy for requests
    
</dd>
</dl>

<dl>
<dd>

**ssl_ignore:** `Option<bool>` 

Ignore SSL certificate errors.


 Only works if **jsEnabled** is **true**.
    
</dd>
</dl>

<dl>
<dd>

**window_size:** `Option<String>` 

Specify the browser window size in the format 'width,height' (e.g., "1920w,1080h"). Default value is the default resolutions provided by web/browser.


 Only works if **jsEnabled** is **true**.
    
</dd>
</dl>

<dl>
<dd>

**ad_block:** `Option<bool>` 

Set to `true` to apply ad-blocker to the specified URL else false or ignore to not apply.


 Only works if **jsEnabled** is **true**.
    
</dd>
</dl>

<dl>
<dd>

**captcha:** `Option<bool>` 

if true user can provide captcha instructions in the instructions to solve image captchas.


  Only works if **jsEnabled** is **true**.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">email_validate</a>(request: EmailValidateRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;EmailValidateRequestFormat&gt;&gt;) -> Result&lt;EmailValidateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates a single email address and returns result.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .email_validate(
            &EmailValidateRequest {
                api_key: "apiKey".to_string(),
                email: "email".to_string(),
                format: None,
                name: None,
                ip: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email address to validate
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of the email address
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP address of the email address
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<EmailValidateRequestFormat>` — Format of the response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_email_validate</a>(request: BulkEmailValidateRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkEmailValidateRequestFormat&gt;&gt;) -> Result&lt;BulkEmailValidateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates a bulk of email addresses and returns result for each. Maximum `10` email addresses per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_email_validate(
            &BulkEmailValidateRequest {
                api_key: "apiKey".to_string(),
                email_data: vec![BulkEmailValidateRequestEmailDataItem {
                    email: "email".to_string(),
                    ..Default::default()
                }],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email_data:** `Vec<BulkEmailValidateRequestEmailDataItem>` — Array of email objects for bulk validation
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkEmailValidateRequestFormat>` — Format of the response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">phone_validate</a>(request: PhoneValidateRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PhoneValidateRequestFormat&gt;&gt;) -> Result&lt;PhoneValidateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates a single phone number and returns detailed metadata including carrier, line type, geolocation, time zones, and standardized formats.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .phone_validate(
            &PhoneValidateRequest {
                api_key: "apiKey".to_string(),
                number: "+14155552671".to_string(),
                format: None,
                region: None,
                dialer_region: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**number:** `String` — Phone number to validate. Accepts international format (+14155552671), local format (4155552671) with region, or IDD format (0014155552671) with dialer_region.
    
</dd>
</dl>

<dl>
<dd>

**region:** `Option<String>` — Two-letter ISO country code (e.g., US, GB). Required when number is in local format without + prefix. Cannot be used together with dialer_region.
    
</dd>
</dl>

<dl>
<dd>

**dialer_region:** `Option<String>` — Two-letter ISO country code indicating the country the number is being dialed from. Required when number uses IDD exit code. Cannot be used together with region.
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PhoneValidateRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object. If not provided, the API defaults to JSON format.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_phone_validate</a>(request: BulkPhoneValidateRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkPhoneValidateRequestFormat&gt;&gt;) -> Result&lt;Vec&lt;BulkPhoneValidateResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates up to 100 phone numbers in a single request. Each number is processed independently — invalid entries return per-number errors without affecting the rest of the batch.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_phone_validate(
            &BulkPhoneValidateRequest {
                api_key: "apiKey".to_string(),
                numbers: vec![
                    BulkPhoneValidateRequestNumbersItem {
                        number: "+14155552671".to_string(),
                        ..Default::default()
                    },
                    BulkPhoneValidateRequestNumbersItem {
                        number: "+447911123456".to_string(),
                        ..Default::default()
                    },
                    BulkPhoneValidateRequestNumbersItem {
                        number: "+919876543210".to_string(),
                        ..Default::default()
                    },
                ],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**numbers:** `Vec<BulkPhoneValidateRequestNumbersItem>` — Array of phone number objects. Maximum 100 per request.
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkPhoneValidateRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object. If not provided, the API defaults to JSON format.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_ssl_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainSslLookupRequestFormat&gt;&gt;, domain_name: Option&lt;String&gt;, ssl_raw: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;DomainSslLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve comprehensive SSL certificate information without the certificate chain.
This endpoint provides detailed information about the SSL certificate including expiry dates, issuer details, and encryption methods.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_ssl_lookup(
            &DomainSslLookupQueryRequest {
                api_key: "apiKey".to_string(),
                domain_name: "domainName".to_string(),
                format: None,
                ssl_raw: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainSslLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**domain_name:** `String` — Domain name or URL whose SSL certificate lookup is required
    
</dd>
</dl>

<dl>
<dd>

**ssl_raw:** `Option<bool>` — Set to true to get the raw openSSL response of the domain
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_ssl_chain_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainSslChainLookupRequestFormat&gt;&gt;, domain_name: Option&lt;String&gt;, ssl_raw: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;DomainSslChainLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the complete SSL certificate chain from root Certificate Authority (CA) to end-user certificate.
This endpoint provides comprehensive information about each certificate in the chain.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_ssl_chain_lookup(
            &DomainSslChainLookupQueryRequest {
                api_key: "apiKey".to_string(),
                domain_name: "domainName".to_string(),
                format: None,
                ssl_raw: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainSslChainLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**domain_name:** `String` — Domain name or URL whose SSL certificate chain lookup is required
    
</dd>
</dl>

<dl>
<dd>

**ssl_raw:** `Option<bool>` — Set to true to get the raw openSSL response for each certificate in the chain
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_availability_check</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainAvailabilityCheckRequestFormat&gt;&gt;, domain: Option&lt;String&gt;, source: Option&lt;Option&lt;DomainAvailabilityCheckRequestSource&gt;&gt;) -> Result&lt;DomainAvailabilityCheckResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The Domain Search API is designed to simplify the process of finding available domain names across all top-level domains (TLDs) and second-level domains (SLDs).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_availability_check(
            &DomainAvailabilityCheckQueryRequest {
                api_key: "apiKey".to_string(),
                domain: "domain".to_string(),
                format: None,
                source: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainAvailabilityCheckRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**domain:** `String` — Domain name whose availability is to be checked.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<DomainAvailabilityCheckRequestSource>` — Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_domain_availability_check</a>(request: BulkDomainAvailabilityCheckRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkDomainAvailabilityCheckRequestFormat&gt;&gt;, source: Option&lt;Option&lt;BulkDomainAvailabilityCheckRequestSource&gt;&gt;) -> Result&lt;BulkDomainAvailabilityCheckResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Perform Bulk Domain Availability checks using a list of domains. Supports upto `100 Domains Per Request`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_domain_availability_check(
            &BulkDomainAvailabilityCheckRequest {
                api_key: "apiKey".to_string(),
                domain_names: vec!["domainNames".to_string()],
                format: None,
                source: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_names:** `Vec<String>` — List of domain names to check.
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkDomainAvailabilityCheckRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<BulkDomainAvailabilityCheckRequestSource>` — Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">domain_availability_suggestions</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;DomainAvailabilitySuggestionsRequestFormat&gt;&gt;, domain: Option&lt;String&gt;, source: Option&lt;Option&lt;DomainAvailabilitySuggestionsRequestSource&gt;&gt;, count: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;DomainAvailabilitySuggestionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The Domain Search API is designed to simplify the process of finding available domain names across all top-level domains (TLDs) and second-level domains (SLDs).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .domain_availability_suggestions(
            &DomainAvailabilitySuggestionsQueryRequest {
                api_key: "apiKey".to_string(),
                domain: "domain".to_string(),
                format: None,
                source: None,
                count: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<DomainAvailabilitySuggestionsRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**domain:** `String` — Domain name for availability and suggestions.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<DomainAvailabilitySuggestionsRequestSource>` — Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — Number of suggestions to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">subdomains_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;SubdomainsLookupRequestFormat&gt;&gt;, domain: Option&lt;String&gt;, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;SubdomainsLookupRequestStatus&gt;&gt;, page: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;SubdomainsLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The Subdomain Lookup API is designed to retrieve subdomains related to the given domain name. It helps you explore subdomains that are available for registration or usage.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .subdomains_lookup(
            &SubdomainsLookupQueryRequest {
                api_key: "apiKey".to_string(),
                domain: "domain".to_string(),
                format: None,
                after: None,
                before: None,
                status: None,
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<SubdomainsLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**domain:** `String` — Domain name for availability and suggestions.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Filter subdomains seen after this date (format YYYY-MM-DD).
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Filter subdomains seen before this date( format YYYY-MM-DD).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<SubdomainsLookupRequestStatus>` — Filter subdomains by status (active or inactive).
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<String>` — Page number for paginated results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_merge</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfMergeRequestFormat&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfMergeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API merges multiple PDF files into a single PDF, in the order they are provided
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_merge(
            &PdfMergeRequest {
                api_key: "apiKey".to_string(),
                file: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
                format: None,
                file_id: vec![],
                destroy: None,
                output: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfMergeRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — An array of unique file IDs referencing PDF files previously uploaded to the API Freaks server. Use this parameter to merge existing files without re-uploading them. Provide multiple IDs to merge files in the specified order.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — Specifies the desired name for the resulting merged PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_remove_pages</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfRemovePagesRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, pages: Option&lt;String&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfRemovePagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API removes a selection or range of pages from a PDF file.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_remove_pages(
            &PdfRemovePagesRequest {
                api_key: "apiKey".to_string(),
                pages: "pages".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfRemovePagesRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique identifier of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output PDF file after pages have been removed. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `String` — Specifies which pages to remove from the PDF. Accepts individual page numbers (e.g., '1,7') and/or ascending page ranges (e.g., '3-5'). Use commas to separate entries and hyphens for ranges. Reverse ranges (e.g., '5-3') are not allowed. Alternatively, you may provide only one of the following keywords: 'even' (removes all even-numbered pages), 'odd' (removes all odd-numbered pages), or 'last' (removes only the last page). The keyword 'all' is not supported for this operation. Examples: '1,3-5', 'even'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_split</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfSplitRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfSplitResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API splits a PDF into multiple parts based on specified page numbers or ranges.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_split(
            &PdfSplitRequest {
                api_key: "apiKey".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                pages: vec![],
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfSplitRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired base name for the output PDF files after splitting. If not provided, a default naming convention will be used.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `Option<String>` 

Defines the page numbers or ranges where the PDF should be split. Provide individual pages and/or ranges in any order (for example: "1-4,9-5,16-last"). Separate entries with commas and use hyphens for ranges.

Special keywords (use alone):

• `even` — split at every even-numbered page

• `odd` — split at every odd-numbered page

• `all` — split the PDF into single-page files

The keyword `last` can be used anywhere in the string, in combination with page numbers or ranges (for example: "5-last", "last-2", "1,last,9").

Examples:
- "1,4-2,last"
- "odd"
- "all"
- "last,2-5"

Invalid example: "1,odd" (mixing a keyword other than "last" with specific pages/ranges is not allowed). You can pass multiple pages entries to produce multiple output files.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_rotate</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfRotateRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, pages: Option&lt;Option&lt;String&gt;&gt;, rotate: Option&lt;i64&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfRotateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API rotates pages of a PDF by a specified angle (in multiples of 90 degrees).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_rotate(
            &PdfRotateRequest {
                api_key: "apiKey".to_string(),
                rotate: 1,
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                pages: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfRotateRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output PDF file after rotation. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `Option<String>` — Specifies which pages to rotate. Accepts individual page numbers (e.g., '1,7') and/or ascending page ranges (e.g., '3-5'). Use commas to separate entries and hyphens for ranges. Reverse ranges (e.g., '5-3') are not allowed. Alternatively, provide only one of the following keywords: 'even' (rotate all even-numbered pages), 'odd' (rotate all odd-numbered pages), 'last' (rotate only the last page), or 'all' (rotate all pages). Examples: '1,3-5', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**rotate:** `i64` — The angle in degrees to rotate the selected pages. Must be one of the following values: 0, 90, 180, 270, -90, -180, or -270. All rotations are applied clockwise.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_compress</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfCompressRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, compression_level: Option&lt;PdfCompressRequestCompressionLevel&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfCompressResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API compresses a given PDF file to reduce its file size.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_compress(
            &PdfCompressRequest {
                api_key: "apiKey".to_string(),
                compression_level: PdfCompressRequestCompressionLevel::Low,
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                output: None,
                destroy: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfCompressRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — Name of the output PDF.
    
</dd>
</dl>

<dl>
<dd>

**compression_level:** `PdfCompressRequestCompressionLevel` — Controls how aggressively the PDF is compressed. Lower levels preserve more quality, while higher levels reduce file size more.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to true, the input file(s) will be deleted from the server immediately after the output is generated.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_extract_pages</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfExtractPagesRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, pages: Option&lt;String&gt;, separated: Option&lt;Option&lt;bool&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfExtractPagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API extracts specific pages or page ranges from a PDF file and returns them as a new PDF.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_extract_pages(
            &PdfExtractPagesRequest {
                api_key: "apiKey".to_string(),
                pages: "pages".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                separated: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfExtractPagesRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output PDF file after pages have been extracted. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `String` — Specifies which pages to extract from the PDF. You can provide individual page numbers (e.g., '2') and/or page ranges in any order, including descending (e.g., '9-5', '16-last'). Use commas to separate entries and hyphens for ranges. You may alternatively pass only one of the special keywords: 'even' (extracts all even-numbered pages), 'odd' (extracts all odd-numbered pages), 'last' (extracts only the last page), or 'all' (extracts all pages into individual files). Examples: '2,6-3', 'even', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**separated:** `Option<bool>` — If set to `true`, each of the specified pages will be extracted and returned as a separate PDF file.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_linearize</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfLinearizeRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfLinearizeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

API endpoint that linearizes any given PDF, restructuring it for faster loading and page-by-page viewing in web browsers.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_linearize(
            &PdfLinearizeRequest {
                api_key: "apiKey".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfLinearizeRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output PDF file after pages have been extracted. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_encrypt</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfEncryptRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, file_password: Option&lt;Option&lt;String&gt;&gt;, user_password: Option&lt;String&gt;, owner_password: Option&lt;Option&lt;String&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfEncryptResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API encrypts a PDF file by setting a password required to open it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_encrypt(
            &PdfEncryptRequest {
                api_key: "apiKey".to_string(),
                user_password: "user_password".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                file_password: None,
                owner_password: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfEncryptRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output encrypted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**file_password:** `Option<String>` — The password to unlock the input file if it is already protected. Either the owner password or user password can be provided. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**user_password:** `String` — Sets the user password required to open and view the encrypted PDF file. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**owner_password:** `Option<String>` — Sets the owner password for the PDF file. This password provides full access, including the ability to remove restrictions. If not provided, the `user_password` will also be used as the owner password. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_decrypt</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfDecryptRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, file_password: Option&lt;String&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfDecryptResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API decrypts PDF files, removing all encryption, including open passwords and permission restrictions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_decrypt(
            &PdfDecryptRequest {
                api_key: "apiKey".to_string(),
                file_password: "file_password".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfDecryptRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output decrypted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**file_password:** `String` — The password to unlock the input file if it is protected. Either the owner password or user password can be provided. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_restrict</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfRestrictRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, file_password: Option&lt;Option&lt;String&gt;&gt;, user_password: Option&lt;String&gt;, owner_password: Option&lt;Option&lt;String&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfRestrictResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API applies permission restrictions on a PDF file, such as disabling printing, copying, or editing. This can include password protection to enforce restrictions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_restrict(
            &PdfRestrictRequest {
                api_key: "apiKey".to_string(),
                user_password: "user_password".to_string(),
                restrictions: vec![Some(PdfRestrictRequestRestrictionsItem::PrintHigh)],
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                file_password: None,
                owner_password: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfRestrictRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output restricted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**file_password:** `Option<String>` — The password to unlock the input file if it is already secured. Provide the owner password if available; otherwise, the user password. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**user_password:** `String` — Sets the password users will use to open the PDF. If this is not set, only the owner password will be configured, and anyone can open the PDF file with the provided restrictions enabled. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**owner_password:** `Option<String>` — Sets the password that allows full access to the PDF (e.g., removing restrictions). If not provided, the `user_password` (if set) will also be used as the owner password. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**restrictions:** `Option<PdfRestrictRequestRestrictionsItem>` 

A comma-separated list of restrictions to apply to the PDF. These define what the end-user is *not* allowed to do with the PDF. Available options are:


* **print_high** – Disables high-quality printing.
* **print_low** – Disables low-resolution printing.
* **edit_document_assembly** – Prevents reordering or inserting pages.
* **fill_form_fields** – Disallows filling in PDF form fields.
* **edit_annotations** – Disables adding or modifying annotations or comments.
* **modify_content** – Prevents modifying existing content in the PDF.
* **copy_and_extract_content** – Disables copying text or images from the PDF.
* **use_accessibility** – Prevents screen readers or accessibility tools from accessing content.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_unrestrict</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfUnrestrictRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, file_password: Option&lt;String&gt;, user_password: Option&lt;Option&lt;String&gt;&gt;, owner_password: Option&lt;Option&lt;String&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfUnrestrictResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API removes permission restrictions from a PDF while keeping it encrypted. If you want to remove all security (including encryption), use the `/pdf/decrypt` endpoint instead.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_unrestrict(
            &PdfUnrestrictRequest {
                api_key: "apiKey".to_string(),
                file_password: "file_password".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                user_password: None,
                owner_password: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfUnrestrictRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**file_password:** `String` — The password to unlock the input file. Either the owner password or user password can be provided. The owner password takes precedence. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**user_password:** `Option<String>` — Sets the user password for the PDF file. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**owner_password:** `Option<String>` — Sets the owner password for the PDF file. If the owner password is not provided, the `user_password` will also be used as the owner password. Password Length should be between 6 and 128 characters.
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_convert_to_png</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfConvertToPngRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, pages: Option&lt;Option&lt;String&gt;&gt;, resolution: Option&lt;Option&lt;i64&gt;&gt;, image_smoothing: Option&lt;Option&lt;String&gt;&gt;, profile: Option&lt;Option&lt;PdfConvertToPngRequestProfile&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfConvertToPngResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API converts a given PDF file into a sequence of PNG images.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_convert_to_png(
            &PdfConvertToPngRequest {
                api_key: "apiKey".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                pages: None,
                resolution: None,
                image_smoothing: None,
                profile: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfConvertToPngRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `Option<String>` — Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**resolution:** `Option<i64>` — Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    
</dd>
</dl>

<dl>
<dd>

**image_smoothing:** `Option<String>` — Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    
</dd>
</dl>

<dl>
<dd>

**profile:** `Option<PdfConvertToPngRequestProfile>` — Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_convert_to_jpg</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfConvertToJpgRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, quality: Option&lt;Option&lt;i64&gt;&gt;, pages: Option&lt;Option&lt;String&gt;&gt;, resolution: Option&lt;Option&lt;i64&gt;&gt;, image_smoothing: Option&lt;Option&lt;String&gt;&gt;, profile: Option&lt;Option&lt;PdfConvertToJpgRequestProfile&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfConvertToJpgResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API converts a given PDF file into a sequence of JPG images.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_convert_to_jpg(
            &PdfConvertToJpgRequest {
                api_key: "apiKey".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                quality: None,
                pages: None,
                resolution: None,
                image_smoothing: None,
                profile: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfConvertToJpgRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**quality:** `Option<i64>` — Controls JPG compression quality. Higher values yield sharper images with larger file sizes.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `Option<String>` — Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**resolution:** `Option<i64>` — Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    
</dd>
</dl>

<dl>
<dd>

**image_smoothing:** `Option<String>` — Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    
</dd>
</dl>

<dl>
<dd>

**profile:** `Option<PdfConvertToJpgRequestProfile>` — Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_convert_to_tiff</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfConvertToTiffRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, pages: Option&lt;Option&lt;String&gt;&gt;, resolution: Option&lt;Option&lt;i64&gt;&gt;, image_smoothing: Option&lt;Option&lt;String&gt;&gt;, profile: Option&lt;Option&lt;PdfConvertToTiffRequestProfile&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfConvertToTiffResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API converts a given PDF file into a sequence of TIFF images. The output images can be saved as a single TIFF file, or as a sequence of TIFF files.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_convert_to_tiff(
            &PdfConvertToTiffRequest {
                api_key: "apiKey".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                pages: None,
                resolution: None,
                image_smoothing: None,
                profile: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfConvertToTiffRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `Option<String>` — Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**resolution:** `Option<i64>` — Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    
</dd>
</dl>

<dl>
<dd>

**image_smoothing:** `Option<String>` — Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    
</dd>
</dl>

<dl>
<dd>

**profile:** `Option<PdfConvertToTiffRequestProfile>` — Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_convert_to_bmp</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfConvertToBmpRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, pages: Option&lt;Option&lt;String&gt;&gt;, resolution: Option&lt;Option&lt;i64&gt;&gt;, image_smoothing: Option&lt;Option&lt;String&gt;&gt;, profile: Option&lt;Option&lt;PdfConvertToBmpRequestProfile&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfConvertToBmpResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Converts a PDF file to a BMP image.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_convert_to_bmp(
            &PdfConvertToBmpRequest {
                api_key: "apiKey".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                pages: None,
                resolution: None,
                image_smoothing: None,
                profile: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfConvertToBmpRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `Option<String>` — Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**resolution:** `Option<i64>` — Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    
</dd>
</dl>

<dl>
<dd>

**image_smoothing:** `Option<String>` — Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    
</dd>
</dl>

<dl>
<dd>

**profile:** `Option<PdfConvertToBmpRequestProfile>` — Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_convert_to_gif</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfConvertToGifRequestFormat&gt;&gt;, file_id: Option&lt;Option&lt;String&gt;&gt;, destroy: Option&lt;Option&lt;bool&gt;&gt;, output: Option&lt;Option&lt;String&gt;&gt;, pages: Option&lt;Option&lt;String&gt;&gt;, resolution: Option&lt;Option&lt;i64&gt;&gt;, image_smoothing: Option&lt;Option&lt;String&gt;&gt;, profile: Option&lt;Option&lt;PdfConvertToGifRequestProfile&gt;&gt;, webhook_url: Option&lt;Option&lt;String&gt;&gt;, webhook_failure_notification: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;PdfConvertToGifResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API converts a given PDF file into a sequence of GIF images.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_convert_to_gif(
            &PdfConvertToGifRequest {
                api_key: "apiKey".to_string(),
                file: b"test file content".to_vec(),
                format: None,
                file_id: None,
                destroy: None,
                output: None,
                pages: None,
                resolution: None,
                image_smoothing: None,
                profile: None,
                webhook_url: None,
                webhook_failure_notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfConvertToGifRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — The unique ID of a PDF file already uploaded to the API Freaks server. Use this as an alternative to uploading a new file directly.
    
</dd>
</dl>

<dl>
<dd>

**destroy:** `Option<bool>` — If set to `true`, the input file(s) will be permanently deleted from the server immediately after the output PDF is generated.
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<String>` — The desired name for the output unrestricted PDF file. If not provided, a default name will be assigned.
    
</dd>
</dl>

<dl>
<dd>

**pages:** `Option<String>` — Specifies the pages or ranges at which to split the PDF. Accepts individual page numbers (e.g., '1') and/or page ranges (e.g., '4-2', 'last'). Ranges can be ascending or descending. Use commas to separate entries and hyphens for ranges. Alternatively, provide only one of the following keywords: 'even' (split at every even-numbered page), 'odd' (split at every odd-numbered page), 'last' (split at the last page only), or 'all' (split into single pages). Examples: '1,4-2,last', 'odd', 'all'. Mixing special keywords with specific pages/ranges is not allowed.
    
</dd>
</dl>

<dl>
<dd>

**resolution:** `Option<i64>` — Specifies the resolution (in DPI) for the output images. Acceptable Range is from 20 to 1200.
    
</dd>
</dl>

<dl>
<dd>

**image_smoothing:** `Option<String>` — Determines the smoothing options to apply during image conversion. Valid values are 'none', 'all' or a combination of 'text', 'line', and 'image' (comma-separated).If not provided, no smoothing will be applied.
    
</dd>
</dl>

<dl>
<dd>

**profile:** `Option<PdfConvertToGifRequestProfile>` — Specifies the color profile for the output PNG images. Acceptable values: bw (1-bit black & white, smallest size, no grayscale or color), gray (8-bit grayscale), rgb (24-bit RGB color, default), rgba (32-bit RGB color with alpha channel for transparency), 4-bit (4-bit indexed color, up to 16 colors, smaller size), or 8-bit (8-bit indexed color, up to 256 colors).
    
</dd>
</dl>

<dl>
<dd>

**webhook_url:** `Option<String>` — The URL to which the webhook notification will be sent after the task is completed.
    
</dd>
</dl>

<dl>
<dd>

**webhook_failure_notification:** `Option<bool>` — If true, a notification will also be sent by email in case the webhook request fails all the retries.  The email notification will be sent to the requesting user or their organization’s admin if part of one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_upload_resources</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfUploadResourcesRequestFormat&gt;&gt;) -> Result&lt;PdfUploadResourcesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API uploads multiple PDF files to the API Freaks server and generates their unique file IDs.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_upload_resources(
            &PdfUploadResourcesRequest {
                api_key: "apiKey".to_string(),
                file: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfUploadResourcesRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_download_resource</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfDownloadResourceRequestFormat&gt;&gt;, resource_id: Option&lt;String&gt;) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API downloads PDF files or ZIP archives from the server using their unique resource ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_download_resource(
            &PdfDownloadResourceQueryRequest {
                api_key: "apiKey".to_string(),
                resource_id: "resource_id".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfDownloadResourceRequestFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**resource_id:** `String` — The unique identifier of the file or ZIP archive to download.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_get_task_status</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfGetTaskStatusRequestFormat&gt;&gt;, task_id: Option&lt;String&gt;) -> Result&lt;PdfGetTaskStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API checks the status of a previously initiated PDF processing task using its unique task ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_get_task_status(
            &PdfGetTaskStatusQueryRequest {
                api_key: "apiKey".to_string(),
                task_id: "task_id".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfGetTaskStatusRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**task_id:** `String` — The unique ID of the PDF processing task for which the status is requested.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_get_file_status</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfGetFileStatusRequestFormat&gt;&gt;, file_id: Option&lt;String&gt;) -> Result&lt;PdfGetFileStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API checks the status of a PDF file using its unique file ID, providing information about its creation and potential deletion time.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_get_file_status(
            &PdfGetFileStatusQueryRequest {
                api_key: "apiKey".to_string(),
                file_id: "file_id".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfGetFileStatusRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `String` — The unique ID of the file whose status is requested.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_list_files</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfListFilesRequestFormat&gt;&gt;) -> Result&lt;PdfListFilesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API retrieves a list of all PDF files uploaded and generated by a specific user. Please note that if the user is part of an organization, only the Organization Administrator can access this endpoint. Organization Members cannot access this endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_list_files(
            &PdfListFilesQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfListFilesRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">pdf_delete_file</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;PdfDeleteFileRequestFormat&gt;&gt;, file_id: Option&lt;String&gt;) -> Result&lt;PdfDeleteFileResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This API deletes a PDF file using its unique file ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .pdf_delete_file(
            &PdfDeleteFileQueryRequest {
                api_key: "apiKey".to_string(),
                file_id: "file_id".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PdfDeleteFileRequestFormat>` — Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `String` — The unique ID of the file to be deleted.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">screenshot_capture</a>(api_key: Option&lt;String&gt;, output: Option&lt;Option&lt;ScreenshotCaptureRequestOutput&gt;&gt;, file_type: Option&lt;Option&lt;ScreenshotCaptureRequestFileType&gt;&gt;, url: Option&lt;String&gt;, width: Option&lt;Option&lt;i64&gt;&gt;, height: Option&lt;Option&lt;i64&gt;&gt;, full_page: Option&lt;Option&lt;bool&gt;&gt;, fresh: Option&lt;Option&lt;bool&gt;&gt;, no_cookie_banners: Option&lt;Option&lt;bool&gt;&gt;, enable_caching: Option&lt;Option&lt;bool&gt;&gt;, block_ads: Option&lt;Option&lt;bool&gt;&gt;, block_chat_widgets: Option&lt;Option&lt;bool&gt;&gt;, extract_text: Option&lt;Option&lt;bool&gt;&gt;, extract_html: Option&lt;Option&lt;bool&gt;&gt;, destroy_screenshot: Option&lt;Option&lt;bool&gt;&gt;, lazy_load: Option&lt;Option&lt;bool&gt;&gt;, retina: Option&lt;Option&lt;bool&gt;&gt;, dark_mode: Option&lt;Option&lt;bool&gt;&gt;, block_tracking: Option&lt;Option&lt;bool&gt;&gt;, enable_incognito: Option&lt;Option&lt;bool&gt;&gt;, omit_background: Option&lt;Option&lt;bool&gt;&gt;, thumbnail_width: Option&lt;Option&lt;i64&gt;&gt;, adjust_top: Option&lt;Option&lt;i64&gt;&gt;, wait_for_event: Option&lt;Option&lt;ScreenshotCaptureRequestWaitForEvent&gt;&gt;, grayscale: Option&lt;Option&lt;i64&gt;&gt;, delay: Option&lt;Option&lt;i64&gt;&gt;, timeout: Option&lt;Option&lt;i64&gt;&gt;, ttl: Option&lt;Option&lt;i64&gt;&gt;, clip_x: Option&lt;Option&lt;i64&gt;&gt;, clip_y: Option&lt;Option&lt;i64&gt;&gt;, clip_width: Option&lt;Option&lt;i64&gt;&gt;, clip_height: Option&lt;Option&lt;i64&gt;&gt;, css_url: Option&lt;Option&lt;String&gt;&gt;, css: Option&lt;Option&lt;String&gt;&gt;, js_url: Option&lt;Option&lt;String&gt;&gt;, js: Option&lt;Option&lt;String&gt;&gt;, block_js: Option&lt;Option&lt;bool&gt;&gt;, block_stylesheets: Option&lt;Option&lt;bool&gt;&gt;, block_images: Option&lt;Option&lt;bool&gt;&gt;, block_media: Option&lt;Option&lt;bool&gt;&gt;, block_font: Option&lt;Option&lt;bool&gt;&gt;, block_text_track: Option&lt;Option&lt;bool&gt;&gt;, block_xhr: Option&lt;Option&lt;bool&gt;&gt;, block_fetch: Option&lt;Option&lt;bool&gt;&gt;, block_event_source: Option&lt;Option&lt;bool&gt;&gt;, block_web_socket: Option&lt;Option&lt;bool&gt;&gt;, block_manifest: Option&lt;Option&lt;bool&gt;&gt;, block_specific_requests: Option&lt;Option&lt;String&gt;&gt;, blur_selector: Option&lt;Option&lt;String&gt;&gt;, remove_selector: Option&lt;Option&lt;String&gt;&gt;, result_file_name: Option&lt;Option&lt;String&gt;&gt;, scrolling_screenshot: Option&lt;Option&lt;bool&gt;&gt;, scroll_speed: Option&lt;Option&lt;ScreenshotCaptureRequestScrollSpeed&gt;&gt;, scroll_back: Option&lt;Option&lt;bool&gt;&gt;, start_immediately: Option&lt;Option&lt;bool&gt;&gt;, multiple_scrolling: Option&lt;Option&lt;bool&gt;&gt;, duration: Option&lt;Option&lt;f64&gt;&gt;, fail_on_error: Option&lt;Option&lt;bool&gt;&gt;, longitude: Option&lt;Option&lt;f64&gt;&gt;, latitude: Option&lt;Option&lt;f64&gt;&gt;, proxy: Option&lt;Option&lt;String&gt;&gt;, headers: Option&lt;Option&lt;String&gt;&gt;, cookies: Option&lt;Option&lt;String&gt;&gt;, scroll_to_element: Option&lt;Option&lt;String&gt;&gt;, selector: Option&lt;Option&lt;String&gt;&gt;, user_agent: Option&lt;Option&lt;String&gt;&gt;, accept_languages: Option&lt;Option&lt;String&gt;&gt;, custom_html: Option&lt;Option&lt;String&gt;&gt;, image_quality: Option&lt;Option&lt;f64&gt;&gt;) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Capture full-page screenshots and videos of websites with advanced options like device simulation, custom code injection, cookie banner blocking, and scrollable content recording.
Supports multiple output formats including JSON, image, GIF, MP4, and WebM.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .screenshot_capture(
            &ScreenshotCaptureQueryRequest {
                api_key: "apiKey".to_string(),
                url: "url".to_string(),
                output: None,
                file_type: None,
                width: None,
                height: None,
                full_page: None,
                fresh: None,
                no_cookie_banners: None,
                enable_caching: None,
                block_ads: None,
                block_chat_widgets: None,
                extract_text: None,
                extract_html: None,
                destroy_screenshot: None,
                lazy_load: None,
                retina: None,
                dark_mode: None,
                block_tracking: None,
                enable_incognito: None,
                omit_background: None,
                thumbnail_width: None,
                adjust_top: None,
                wait_for_event: None,
                grayscale: None,
                delay: None,
                timeout: None,
                ttl: None,
                clip_x: None,
                clip_y: None,
                clip_width: None,
                clip_height: None,
                css_url: None,
                css: None,
                js_url: None,
                js: None,
                block_js: None,
                block_stylesheets: None,
                block_images: None,
                block_media: None,
                block_font: None,
                block_text_track: None,
                block_xhr: None,
                block_fetch: None,
                block_event_source: None,
                block_web_socket: None,
                block_manifest: None,
                block_specific_requests: None,
                blur_selector: None,
                remove_selector: None,
                result_file_name: None,
                scrolling_screenshot: None,
                scroll_speed: None,
                scroll_back: None,
                start_immediately: None,
                multiple_scrolling: None,
                sizes: vec![],
                duration: None,
                fail_on_error: None,
                longitude: None,
                latitude: None,
                proxy: None,
                headers: None,
                cookies: None,
                scroll_to_element: None,
                selector: None,
                user_agent: None,
                accept_languages: None,
                custom_html: None,
                image_quality: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**output:** `Option<ScreenshotCaptureRequestOutput>` — Output format for screenshot results
    
</dd>
</dl>

<dl>
<dd>

**file_type:** `Option<ScreenshotCaptureRequestFileType>` — File type for screenshot output
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — URLs to capture screenshots of
    
</dd>
</dl>

<dl>
<dd>

**width:** `Option<i64>` — Browser viewport width in pixels
    
</dd>
</dl>

<dl>
<dd>

**height:** `Option<i64>` — Browser viewport height in pixels
    
</dd>
</dl>

<dl>
<dd>

**full_page:** `Option<bool>` — Capture a full-page screenshot
    
</dd>
</dl>

<dl>
<dd>

**fresh:** `Option<bool>` — Bypass cache and take a fresh screenshot
    
</dd>
</dl>

<dl>
<dd>

**no_cookie_banners:** `Option<bool>` — Remove cookie banners from the screenshot
    
</dd>
</dl>

<dl>
<dd>

**enable_caching:** `Option<bool>` — Enable caching for repeated requests
    
</dd>
</dl>

<dl>
<dd>

**block_ads:** `Option<bool>` — Block advertisements on the page
    
</dd>
</dl>

<dl>
<dd>

**block_chat_widgets:** `Option<bool>` — Block chat widget scripts from loading
    
</dd>
</dl>

<dl>
<dd>

**extract_text:** `Option<bool>` — Extract visible text from the page
    
</dd>
</dl>

<dl>
<dd>

**extract_html:** `Option<bool>` — Extract HTML content of the page
    
</dd>
</dl>

<dl>
<dd>

**destroy_screenshot:** `Option<bool>` — Auto-destroy screenshot after fetch
    
</dd>
</dl>

<dl>
<dd>

**lazy_load:** `Option<bool>` — Enable lazy-loading content before screenshot
    
</dd>
</dl>

<dl>
<dd>

**retina:** `Option<bool>` — Capture screenshot in high-DPI (Retina) mode
    
</dd>
</dl>

<dl>
<dd>

**dark_mode:** `Option<bool>` — Render page in dark mode
    
</dd>
</dl>

<dl>
<dd>

**block_tracking:** `Option<bool>` — Block common user-tracking scripts
    
</dd>
</dl>

<dl>
<dd>

**enable_incognito:** `Option<bool>` — Enable private/incognito mode for browser session
    
</dd>
</dl>

<dl>
<dd>

**omit_background:** `Option<bool>` — Omit background color (transparent background)
    
</dd>
</dl>

<dl>
<dd>

**thumbnail_width:** `Option<i64>` — Thumbnail width in pixels
    
</dd>
</dl>

<dl>
<dd>

**adjust_top:** `Option<i64>` — Adjust top in pixels
    
</dd>
</dl>

<dl>
<dd>

**wait_for_event:** `Option<ScreenshotCaptureRequestWaitForEvent>` — Wait for a specific load event before capturing the screenshot.
    
</dd>
</dl>

<dl>
<dd>

**grayscale:** `Option<i64>` — Range:0 to 100 for grayscale filter
    
</dd>
</dl>

<dl>
<dd>

**delay:** `Option<i64>` — How many milliseconds to wait before taking the screenshot
    
</dd>
</dl>

<dl>
<dd>

**timeout:** `Option<i64>` — Maximum timeout in milliseconds. Defalut is `10,000`
    
</dd>
</dl>

<dl>
<dd>

**ttl:** `Option<i64>` — Number of seconds the screenshot should be cached
    
</dd>
</dl>

<dl>
<dd>

**clip_x:** `Option<i64>` — X position of the clipping rectangle in pixels
    
</dd>
</dl>

<dl>
<dd>

**clip_y:** `Option<i64>` — Y position of the clipping rectangle in pixels
    
</dd>
</dl>

<dl>
<dd>

**clip_width:** `Option<i64>` — Width of the clipping rectangle in pixels
    
</dd>
</dl>

<dl>
<dd>

**clip_height:** `Option<i64>` — Height of the clipping rectangle in pixels
    
</dd>
</dl>

<dl>
<dd>

**css_url:** `Option<String>` — URL to CSS file
    
</dd>
</dl>

<dl>
<dd>

**css:** `Option<String>` — Your custom CSS code
    
</dd>
</dl>

<dl>
<dd>

**js_url:** `Option<String>` — URL to JS file
    
</dd>
</dl>

<dl>
<dd>

**js:** `Option<String>` — Your JS code
    
</dd>
</dl>

<dl>
<dd>

**block_js:** `Option<bool>` — Block Scripts
    
</dd>
</dl>

<dl>
<dd>

**block_stylesheets:** `Option<bool>` — Block Stylesheets
    
</dd>
</dl>

<dl>
<dd>

**block_images:** `Option<bool>` — Block Images
    
</dd>
</dl>

<dl>
<dd>

**block_media:** `Option<bool>` — Block Media
    
</dd>
</dl>

<dl>
<dd>

**block_font:** `Option<bool>` — Block Fonts
    
</dd>
</dl>

<dl>
<dd>

**block_text_track:** `Option<bool>` — Block Text Tracks
    
</dd>
</dl>

<dl>
<dd>

**block_xhr:** `Option<bool>` — Block XHR Requests
    
</dd>
</dl>

<dl>
<dd>

**block_fetch:** `Option<bool>` — Block Fetch Requests
    
</dd>
</dl>

<dl>
<dd>

**block_event_source:** `Option<bool>` — Block Event Source
    
</dd>
</dl>

<dl>
<dd>

**block_web_socket:** `Option<bool>` — Block Web Sockets
    
</dd>
</dl>

<dl>
<dd>

**block_manifest:** `Option<bool>` — Block Manifest
    
</dd>
</dl>

<dl>
<dd>

**block_specific_requests:** `Option<String>` — Comma- or newline-separated list of specific requests to block. Each line and comma are treated as separate requests for processing. Example: https://example.com, https://example.js
    
</dd>
</dl>

<dl>
<dd>

**blur_selector:** `Option<String>` 

Comma-separated list of indexed CSS selectors to blur.
Format: `index:<selector>`, e.g., `0:.banner,1:#ads`.
    
</dd>
</dl>

<dl>
<dd>

**remove_selector:** `Option<String>` 

Comma-separated list of indexed CSS selectors to blur.
Format: `index:<selector>`, e.g., `0:.banner,1:#ads`.
    
</dd>
</dl>

<dl>
<dd>

**result_file_name:** `Option<String>` 

Specify a meaningful & unique file name to easily identify the screenshot result.
Avoid using spaces or special characters; use hyphens or underscores to separate words.
    
</dd>
</dl>

<dl>
<dd>

**scrolling_screenshot:** `Option<bool>` — **`Scrolling Screenshot`**: Capture a long scrolling screenshot. When true, disable `fullPage` and `freshScreenshot`.
    
</dd>
</dl>

<dl>
<dd>

**scroll_speed:** `Option<ScreenshotCaptureRequestScrollSpeed>` — Speed of scrolling during the screenshot.
    
</dd>
</dl>

<dl>
<dd>

**scroll_back:** `Option<bool>` — If true, the scroll will reverse back to the top after reaching the bottom.
    
</dd>
</dl>

<dl>
<dd>

**start_immediately:** `Option<bool>` — If true, the scrolling capture will start immediately upon page load.
    
</dd>
</dl>

<dl>
<dd>

**multiple_scrolling:** `Option<bool>` — If true, multiple scrolling screenshots will be taken at different viewport sizes.
    
</dd>
</dl>

<dl>
<dd>

**sizes:** `Option<String>` — Comma-separated list of viewport sizes in the format index:XXw:YYh. Example: sizes=0:120w:300h,1:240w:500h
    
</dd>
</dl>

<dl>
<dd>

**duration:** `Option<f64>` — Duration in seconds for the scrolling capture. Acceptable range: 0 to 100 seconds.
    
</dd>
</dl>

<dl>
<dd>

**fail_on_error:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**longitude:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**latitude:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**proxy:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cookies:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**scroll_to_element:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**selector:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**user_agent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accept_languages:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**custom_html:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**image_quality:** `Option<f64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_screenshot_capture</a>(request: BulkScreenshotCaptureRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkScreenshotCaptureRequestFormat&gt;&gt;) -> Result&lt;BulkScreenshotCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Our Bulk Screenshot API allows you to capture screenshots of multiple webpages simultaneously, saving you time and effort. Instead of manually capturing each page one by one, you can batch process URLs and receive high-quality screenshots in the format you choose.
 Maximum `50 URLs` per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_screenshot_capture(
            &BulkScreenshotCaptureRequest {
                api_key: "apiKey".to_string(),
                urls: vec![BulkScreenshotCaptureRequestURLsItem {
                    url: "url".to_string(),
                    ..Default::default()
                }],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**urls:** `Vec<BulkScreenshotCaptureRequestUrlsItem>` — List of website URLs to capture screenshots of
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkScreenshotCaptureRequestFormat>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_latest_rates</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyLatestRatesRequestFormat&gt;&gt;, base: Option&lt;Option&lt;String&gt;&gt;, updates: Option&lt;Option&lt;CurrencyLatestRatesRequestUpdates&gt;&gt;) -> Result&lt;CurrencyLatestRatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get live forex rates for all world currencies with customizable update frequency
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_latest_rates(
            &CurrencyLatestRatesQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                base: None,
                symbols: vec![],
                updates: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyLatestRatesRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**base:** `Option<String>` — Base currency for rate calculations
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — Comma separated list of desired currency codes
    
</dd>
</dl>

<dl>
<dd>

**updates:** `Option<CurrencyLatestRatesRequestUpdates>` — Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_historical_rates</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyHistoricalRatesRequestFormat&gt;&gt;, base: Option&lt;Option&lt;String&gt;&gt;, date: Option&lt;String&gt;) -> Result&lt;CurrencyHistoricalRatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get historical exchange rates for any specific date
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_historical_rates(
            &CurrencyHistoricalRatesQueryRequest {
                api_key: "apiKey".to_string(),
                date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
                base: None,
                symbols: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyHistoricalRatesRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**base:** `Option<String>` — Base currency for rate calculations
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — Comma separated list of desired currency codes
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` — Specific date in YYYY-MM-DD format
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_convert_latest</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyConvertLatestRequestFormat&gt;&gt;, from: Option&lt;String&gt;, to: Option&lt;String&gt;, amount: Option&lt;Option&lt;f64&gt;&gt;, updates: Option&lt;Option&lt;CurrencyConvertLatestRequestUpdates&gt;&gt;) -> Result&lt;CurrencyConvertLatestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Convert amount between currencies using the latest exchange rates
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_convert_latest(
            &CurrencyConvertLatestQueryRequest {
                api_key: "apiKey".to_string(),
                from: "from".to_string(),
                to: "to".to_string(),
                format: None,
                amount: None,
                updates: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyConvertLatestRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**from:** `String` — Source currency code
    
</dd>
</dl>

<dl>
<dd>

**to:** `String` — Target currency code
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<f64>` — Amount to convert
    
</dd>
</dl>

<dl>
<dd>

**updates:** `Option<CurrencyConvertLatestRequestUpdates>` — Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_convert_historical</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyConvertHistoricalRequestFormat&gt;&gt;, from: Option&lt;String&gt;, to: Option&lt;String&gt;, amount: Option&lt;Option&lt;f64&gt;&gt;, date: Option&lt;String&gt;) -> Result&lt;CurrencyConvertHistoricalResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Convert amount between currencies using historical rates
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_convert_historical(
            &CurrencyConvertHistoricalQueryRequest {
                api_key: "apiKey".to_string(),
                from: "from".to_string(),
                to: "to".to_string(),
                date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
                amount: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyConvertHistoricalRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**from:** `String` — From currency symbol
    
</dd>
</dl>

<dl>
<dd>

**to:** `String` — To currency symbol
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<f64>` — The Amount to be converted
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` — specific date (format YYYY-MM-DD) of which exchange rates is used.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_time_series</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyTimeSeriesRequestFormat&gt;&gt;, start_date: Option&lt;String&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, base: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;CurrencyTimeSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get exchange rates for a time range
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_time_series(
            &CurrencyTimeSeriesQueryRequest {
                api_key: "apiKey".to_string(),
                start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
                end_date: None,
                base: None,
                symbols: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyTimeSeriesRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` — Start date (format YYYY-MM-DD) of the preferred time frame
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — End date (format YYYY-MM-DD) of the preferred time frame
    
</dd>
</dl>

<dl>
<dd>

**base:** `Option<String>` — Base currency
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — comma separated list of desired currencies/ commodities symbols
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_fluctuation</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyFluctuationRequestFormat&gt;&gt;, start_date: Option&lt;String&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, base: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;CurrencyFluctuationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get currency fluctuation data for a time period
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_fluctuation(
            &CurrencyFluctuationQueryRequest {
                api_key: "apiKey".to_string(),
                start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                base: Some("USD".to_string()),
                format: None,
                end_date: None,
                symbols: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyFluctuationRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` — Start date (format YYYY-MM-DD) of the preferred time frame
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — End date (format YYYY-MM-DD) of the preferred time frame
    
</dd>
</dl>

<dl>
<dd>

**base:** `Option<String>` — Base currency
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — comma separated list of desired currencies/ commodities symbols
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_convert_by_ip</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyConvertByIpRequestFormat&gt;&gt;, updates: Option&lt;Option&lt;CurrencyConvertByIpRequestUpdates&gt;&gt;, from: Option&lt;String&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, amount: Option&lt;Option&lt;f64&gt;&gt;) -> Result&lt;CurrencyConvertByIpResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Convert amount using user's location
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_convert_by_ip(
            &CurrencyConvertByIPQueryRequest {
                api_key: "apiKey".to_string(),
                from: "from".to_string(),
                format: None,
                updates: None,
                ip: None,
                amount: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyConvertByIpRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**updates:** `Option<CurrencyConvertByIpRequestUpdates>` — Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    
</dd>
</dl>

<dl>
<dd>

**from:** `String` — From currency symbol
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IPv4 or IPv6 geolocated currency
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<f64>` — Amount to convert
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_supported</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencySupportedRequestFormat&gt;&gt;) -> Result&lt;CurrencySupportedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get list of all supported currencies with their metadata
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_supported(
            &CurrencySupportedQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencySupportedRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_symbols</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencySymbolsRequestFormat&gt;&gt;) -> Result&lt;CurrencySymbolsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get currency symbols and codes
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_symbols(
            &CurrencySymbolsQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencySymbolsRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">currency_historical_limits</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrencyHistoricalLimitsRequestFormat&gt;&gt;) -> Result&lt;CurrencyHistoricalLimitsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about historical data availability and limits
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .currency_historical_limits(
            &CurrencyHistoricalLimitsQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrencyHistoricalLimitsRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">commodity_latest_rates</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CommodityLatestRatesRequestFormat&gt;&gt;, updates: Option&lt;CommodityLatestRatesRequestUpdates&gt;, quote: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;CommodityLatestRatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get live commodity rates with customizable update frequency
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .commodity_latest_rates(
            &CommodityLatestRatesQueryRequest {
                api_key: "apiKey".to_string(),
                symbols: vec![Some("symbols".to_string())],
                updates: CommodityLatestRatesRequestUpdates::TenM,
                format: None,
                quote: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CommodityLatestRatesRequestFormat>` — Format of the Response
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — Comma separated list of desired commodities symbols *(e.g. XAU,XAG,WTI,BRENT)* **Required**
    
</dd>
</dl>

<dl>
<dd>

**updates:** `CommodityLatestRatesRequestUpdates` — Exchange rates update period. Possible values are: (1) `10m` - 10 minute update (2) `1m` - 1 minute update **Required**
    
</dd>
</dl>

<dl>
<dd>

**quote:** `Option<String>` — Specifies the target currency for the exchange rate; default quote currency is the market currency of commodity *(e.g. USD, EUR, INR)*
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">commodity_historical_rates</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CommodityHistoricalRatesRequestFormat&gt;&gt;, date: Option&lt;String&gt;) -> Result&lt;CommodityHistoricalRatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get historical commodity rates for a specific date
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .commodity_historical_rates(
            &CommodityHistoricalRatesQueryRequest {
                api_key: "apiKey".to_string(),
                date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                symbols: vec![Some("symbols".to_string())],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CommodityHistoricalRatesRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` — Historical date (YYYY-MM-DD)
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — Comma-separated list of commodity symbols
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">commodity_fluctuation</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CommodityFluctuationRequestFormat&gt;&gt;, start_date: Option&lt;String&gt;, end_date: Option&lt;String&gt;) -> Result&lt;CommodityFluctuationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get commodity price fluctuation data for a time period
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .commodity_fluctuation(
            &CommodityFluctuationQueryRequest {
                api_key: "apiKey".to_string(),
                symbols: vec![Some("symbols".to_string())],
                start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CommodityFluctuationRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — Comma-separated list of commodity symbols
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` — Start date (YYYY-MM-DD)
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `String` — End date (YYYY-MM-DD)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">commodity_time_series</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CommodityTimeSeriesRequestFormat&gt;&gt;, start_date: Option&lt;String&gt;, end_date: Option&lt;String&gt;) -> Result&lt;CommodityTimeSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get commodity rates for a time range
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .commodity_time_series(
            &CommodityTimeSeriesQueryRequest {
                api_key: "apiKey".to_string(),
                symbols: vec![Some("symbols".to_string())],
                start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CommodityTimeSeriesRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**symbols:** `Option<String>` — Comma-separated list of commodity symbols
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` — Start date (YYYY-MM-DD)
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `String` — End date (YYYY-MM-DD)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">commodity_symbols</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CommoditySymbolsRequestFormat&gt;&gt;) -> Result&lt;CommoditySymbolsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get list of supported commodities
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .commodity_symbols(
            &CommoditySymbolsQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CommoditySymbolsRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">vat_supported_countries</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;VatSupportedCountriesRequestFormat&gt;&gt;, type_: Option&lt;Option&lt;VatSupportedCountriesRequestType&gt;&gt;) -> Result&lt;VatSupportedCountriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of supported countries.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .vat_supported_countries(
            &VatSupportedCountriesQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                r#type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<VatSupportedCountriesRequestFormat>` — Format of the response. Default is JSON.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<VatSupportedCountriesRequestType>` — Type of supported country. Supported values: IBAN, SWIFT, VAT. By default, it returns all supported countries for all types.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">vat_rate_by_ip</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;VatRateByIpRequestFormat&gt;&gt;, ip_address: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;VatRateByIpResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches VAT rate based on the specified or originating IP address.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .vat_rate_by_ip(
            &VatRateByIPQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                ip_address: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<VatRateByIpRequestFormat>` — Specify the desired response format. Options: 'json' (default) or 'xml'.
    
</dd>
</dl>

<dl>
<dd>

**ip_address:** `Option<String>` — IPv4 or IPv6 address to look up VAT rate for. If omitted, the originating IP address will be used.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">vat_rate_by_country</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;VatRateByCountryRequestFormat&gt;&gt;, country: Option&lt;String&gt;, state: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;VatRateByCountryResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches VAT rates for a single country or state provided via query parameters.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .vat_rate_by_country(
            &VatRateByCountryQueryRequest {
                api_key: "apiKey".to_string(),
                country: "country".to_string(),
                format: None,
                state: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<VatRateByCountryRequestFormat>` — Specify the desired response format. Options: 'json' (default) or 'xml'.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country identifier in Alpha-2 (PK), Alpha-3 (PAK), or full name (Pakistan). Combine with the optional "state" query for sub-national VAT; values are case-insensitive and may use underscores instead of spaces.
    
</dd>
</dl>

<dl>
<dd>

**state:** `Option<String>` — Optional state or region in Alpha-2 (NY) or full name (New_York). Use with "country" for state-level VAT; values are case-insensitive and may use underscores.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_vat_rate_by_country</a>(request: BulkVatRateByCountryRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkVatRateByCountryRequestFormat&gt;&gt;) -> Result&lt;BulkVatRateByCountryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves VAT details for multiple countries or country-state combinations in a single request. Maximum of `100` entries per request are allowed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_vat_rate_by_country(
            &BulkVatRateByCountryRequest {
                api_key: "apiKey".to_string(),
                countries: vec![
                    BulkVatRateByCountryRequestCountriesItem {
                        country: "PAK".to_string(),
                        ..Default::default()
                    },
                    BulkVatRateByCountryRequestCountriesItem {
                        country: "United_States".to_string(),
                        state: Some("New_York".to_string()),
                        ..Default::default()
                    },
                ],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**countries:** `Vec<BulkVatRateByCountryRequestCountriesItem>` 
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkVatRateByCountryRequestFormat>` — Specify the desired response format. Options: 'json' (default) or 'xml'.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">vat_validate</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;VatValidateRequestFormat&gt;&gt;, vat_number: Option&lt;String&gt;, requester_vat_number: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;VatValidateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates an EU or UK VAT number and returns registration status details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .vat_validate(
            &VatValidateQueryRequest {
                api_key: "apiKey".to_string(),
                vat_number: "vatNumber".to_string(),
                format: None,
                requester_vat_number: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<VatValidateRequestFormat>` — Specify the desired response format. Options: 'json' (default) or 'xml'.
    
</dd>
</dl>

<dl>
<dd>

**vat_number:** `String` — EU or UK VAT number to validate.
    
</dd>
</dl>

<dl>
<dd>

**requester_vat_number:** `Option<String>` — Requester EU or UK VAT number.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">iban_validate</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;IbanValidateRequestFormat&gt;&gt;, iban: Option&lt;String&gt;) -> Result&lt;IbanValidateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Checks an IBAN for structural validity, checksum accuracy, and bank metadata.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .iban_validate(
            &IbanValidateQueryRequest {
                api_key: "apiKey".to_string(),
                iban: "iban".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<IbanValidateRequestFormat>` — Specify the desired response format. Options: 'json' (default) or 'xml'.
    
</dd>
</dl>

<dl>
<dd>

**iban:** `String` — IBAN to validate.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">swift_code_find</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;SwiftCodeFindRequestFormat&gt;&gt;, country: Option&lt;Option&lt;String&gt;&gt;, bank: Option&lt;Option&lt;String&gt;&gt;, city: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;Vec&lt;String&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches SWIFT codes for a given country, bank, and city.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .swift_code_find(
            &SwiftCodeFindQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                country: None,
                bank: None,
                city: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<SwiftCodeFindRequestFormat>` — Specify the desired response format. Options: 'json' (default) or 'xml'.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Country name (accepts full name, e.g., Pakistan, United States). If only the country parameter is supplied, lists all banks in the country.
    
</dd>
</dl>

<dl>
<dd>

**bank:** `Option<String>` — Bank name (upper case) used to filter SWIFT codes. Should be used together with the country parameter. If only country and bank are provided (without city), returns the list of cities for that bank.
    
</dd>
</dl>

<dl>
<dd>

**city:** `Option<String>` — Gives SWIFT codes for a bank. Optionally specify the city (upper case) to narrow results to a specific city for that bank.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">swift_code_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;SwiftCodeLookupRequestFormat&gt;&gt;, swift_code: Option&lt;String&gt;) -> Result&lt;SwiftCodeLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches detailed information about a SWIFT code.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .swift_code_lookup(
            &SwiftCodeLookupQueryRequest {
                api_key: "apiKey".to_string(),
                swift_code: "swiftCode".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<SwiftCodeLookupRequestFormat>` — Specify the desired response format. Options: 'json' (default) or 'xml'.
    
</dd>
</dl>

<dl>
<dd>

**swift_code:** `String` — SWIFT/BIC code to lookup (must be 8 or 11 characters).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">zipcode_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;ZipcodeLookupRequestFormat&gt;&gt;, code: Option&lt;String&gt;, country: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ZipcodeLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .zipcode_lookup(
            &ZipcodeLookupQueryRequest {
                api_key: "apiKey".to_string(),
                code: "code".to_string(),
                format: None,
                country: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<ZipcodeLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**code:** `String` — Comma separated list of postal / zip codes. Max. 100 values.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Country code in ISO 3166-1 alpha-2 format. If not provided, search results will be returned from all countries.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_zipcode_lookup</a>(request: BulkZipcodeLookupRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkZipcodeLookupRequestFormat&gt;&gt;) -> Result&lt;BulkZipcodeLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates a bulk of ZIP/postal codes and returns result for each. Maximum `100` ZIP/postal codes per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_zipcode_lookup(
            &BulkZipcodeLookupRequest {
                api_key: "apiKey".to_string(),
                codes: vec!["codes".to_string()],
                format: None,
                country: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**codes:** `Vec<String>` — Comma separated list of postal / zip codes. Max. 100 values.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Country code in ISO 3166-1 alpha-2 format. If not provided, search results will be returned from all countries.
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkZipcodeLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">zipcode_search_by_city</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;ZipcodeSearchByCityRequestFormat&gt;&gt;, city: Option&lt;String&gt;, country: Option&lt;String&gt;, state_name: Option&lt;Option&lt;String&gt;&gt;, page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ZipcodeSearchByCityResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .zipcode_search_by_city(
            &ZipcodeSearchByCityQueryRequest {
                api_key: "apiKey".to_string(),
                city: "city".to_string(),
                country: "country".to_string(),
                format: None,
                state_name: None,
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<ZipcodeSearchByCityRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**city:** `String` — Name of the city in which we want to find zipcodes in.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**state_name:** `Option<String>` — Name of the state or province associated with the country.
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — Page number to retrieve paginated results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">zipcode_search_by_region</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;ZipcodeSearchByRegionRequestFormat&gt;&gt;, country: Option&lt;String&gt;, region: Option&lt;String&gt;, page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ZipcodeSearchByRegionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .zipcode_search_by_region(
            &ZipcodeSearchByRegionQueryRequest {
                api_key: "apiKey".to_string(),
                country: "country".to_string(),
                region: "region".to_string(),
                format: None,
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<ZipcodeSearchByRegionRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**region:** `String` — Name of the region, state or province associated with the country.
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — Page no. to retrieve paginated results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">zipcode_search_by_radius</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;ZipcodeSearchByRadiusRequestFormat&gt;&gt;, code: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;String&gt;&gt;, long: Option&lt;Option&lt;String&gt;&gt;, country: Option&lt;Option&lt;String&gt;&gt;, radius: Option&lt;String&gt;, unit: Option&lt;Option&lt;ZipcodeSearchByRadiusRequestUnit&gt;&gt;, page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ZipcodeSearchByRadiusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .zipcode_search_by_radius(
            &ZipcodeSearchByRadiusQueryRequest {
                api_key: "apiKey".to_string(),
                radius: 1.1,
                format: None,
                code: None,
                lat: None,
                long: None,
                country: None,
                unit: None,
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<ZipcodeSearchByRadiusRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` — Postal/Zip code to be used as the center point for the search.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<String>` — Latitude coordinate for the base location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<String>` — Longitude coordinate for the base location.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Country code in ISO 3166-1 alpha-2 format. Required only when using the code parameter.
    
</dd>
</dl>

<dl>
<dd>

**radius:** `String` — Search radius for the query. The maximum allowed values are: - 100 km - 100 mi - 109361 yd - 100000 m - 328084 ft - 3937007.75 in
    
</dd>
</dl>

<dl>
<dd>

**unit:** `Option<ZipcodeSearchByRadiusRequestUnit>` — Supported distance units are m, km, mi, ft, yd, in.
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — Page no. to retrieve paginated results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">zipcode_distance</a>(request: ZipcodeDistanceRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;ZipcodeDistanceRequestFormat&gt;&gt;) -> Result&lt;ZipcodeDistanceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get distance between postal codes. Maximum `100` postal codes per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .zipcode_distance(
            &ZipcodeDistanceRequest {
                api_key: "apiKey".to_string(),
                compare: vec!["compare".to_string()],
                country: "country".to_string(),
                format: None,
                code: None,
                lat: None,
                long: None,
                unit: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**compare:** `Vec<String>` — Comma separated list of postal / zip codes with which base point is compared w.r.t. Max 100 zip codes can be provided.
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` — Postal/Zip code to be used as the base point.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude coordinate for the base location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude coordinate for the base location.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**unit:** `Option<ZipcodeDistanceRequestUnit>` — Supported distance units are m, km, mi, ft, yd, in.
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<ZipcodeDistanceRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">zipcode_distance_match</a>(request: ZipcodeDistanceMatchRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;ZipcodeDistanceMatchRequestFormat&gt;&gt;) -> Result&lt;ZipcodeDistanceMatchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get matching ZIP/postal code pairs within a specified distance. Maximum `100` postal codes per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .zipcode_distance_match(
            &ZipcodeDistanceMatchRequest {
                api_key: "apiKey".to_string(),
                codes: vec!["codes".to_string()],
                country: "country".to_string(),
                format: None,
                distance: None,
                unit: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**codes:** `Vec<String>` — Comma-separated list of postal/zip codes. Maximum 100 values allowed.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**distance:** `Option<String>` — Maximum allowed distance between postal code pairs.
    
</dd>
</dl>

<dl>
<dd>

**unit:** `Option<ZipcodeDistanceMatchRequestUnit>` — Supported distance units are m, km, mi, ft, yd, in.
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<ZipcodeDistanceMatchRequestFormat>` — Format of the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">current_weather</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;CurrentWeatherRequestFormat&gt;&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;f64&gt;&gt;, long: Option&lt;Option&lt;f64&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;CurrentWeatherResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get current weather data including temperature, humidity, precipitation, wind conditions, atmospheric pressure, and air quality for any location. Accepts city names, coordinates, or IP addresses. Also includes astronomy data and timezone-aware timestamps.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .current_weather(
            &CurrentWeatherQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<CurrentWeatherRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — City name, place name, or full address.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP(v4 or v6) address for location inference.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_current_weather</a>(request: BulkCurrentWeatherRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkCurrentWeatherRequestFormat&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;BulkCurrentWeatherResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve current weather conditions for up to `50 locations` in a single request. A maximum of 50 locations (city names, IP addresses, or geographic coordinates) can be included in the request body.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_current_weather(
            &BulkCurrentWeatherRequest {
                api_key: "apiKey".to_string(),
                locations: vec![
                    BulkCurrentWeatherRequestLocationsItem {
                        location: Some("lahore".to_string()),
                        ..Default::default()
                    },
                    BulkCurrentWeatherRequestLocationsItem {
                        lat: Some(32.5),
                        long: Some(74.5),
                        ..Default::default()
                    },
                    BulkCurrentWeatherRequestLocationsItem {
                        ip: Some("8.8.8.8".to_string()),
                        ..Default::default()
                    },
                    BulkCurrentWeatherRequestLocationsItem {
                        location: Some("seoul".to_string()),
                        ..Default::default()
                    },
                ],
                format: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**locations:** `Vec<BulkCurrentWeatherRequestLocationsItem>` — Array of locations to fetch weather data for
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkCurrentWeatherRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">weather_forecast</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;WeatherForecastRequestFormat&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, forecast_days: Option&lt;Option&lt;i64&gt;&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;f64&gt;&gt;, long: Option&lt;Option&lt;f64&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, precision: Option&lt;Option&lt;WeatherForecastRequestPrecision&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;WeatherForecastResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Access comprehensive weather forecasts with customizable precision - choose from daily overviews, hourly breakdowns, or even minute-by-minute data. Configure your date ranges or use the default 7-day forecast for standard weather planning.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .weather_forecast(
            &WeatherForecastQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                start_date: None,
                end_date: None,
                forecast_days: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                precision: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<WeatherForecastRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Start date for the forecast in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between startDate and endDate must not exceed 16 days.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — End date for the forecast in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between startDate and endDate must not exceed 16 days.
    
</dd>
</dl>

<dl>
<dd>

**forecast_days:** `Option<i64>` — Number of days for the forecast, from 1 to 16. Default is 7. Maximum value is 16.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — City name, place name, or full address.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP(v4 or v6) address for location inference.
    
</dd>
</dl>

<dl>
<dd>

**precision:** `Option<WeatherForecastRequestPrecision>` — Precision of the forecast data.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">historical_weather</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;HistoricalWeatherRequestFormat&gt;&gt;, date: Option&lt;String&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;f64&gt;&gt;, long: Option&lt;Option&lt;f64&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, precision: Option&lt;Option&lt;HistoricalWeatherRequestPrecision&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;HistoricalWeatherResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Access past weather conditions for specific dates with records going back to 1940. Retrieve comprehensive historical data with both daily and hourly precision options.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .historical_weather(
            &HistoricalWeatherQueryRequest {
                api_key: "apiKey".to_string(),
                date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                precision: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<HistoricalWeatherRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` — Specific date for which to fetch weather data in YYYY-MM-DD format. Historical dates must be past dates only. Current or future dates are not allowed for historical data. Data available from 1940 onwards.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — City name, place name, or full address.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP(v4 or v6) address for location inference.
    
</dd>
</dl>

<dl>
<dd>

**precision:** `Option<HistoricalWeatherRequestPrecision>` — Precision of the historical data. **Note:** 'daily' returns daily aggregates, 'hourly' returns hourly data.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">weather_time_series</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;WeatherTimeSeriesRequestFormat&gt;&gt;, start_date: Option&lt;String&gt;, end_date: Option&lt;String&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;f64&gt;&gt;, long: Option&lt;Option&lt;f64&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, precision: Option&lt;Option&lt;WeatherTimeSeriesRequestPrecision&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;WeatherTimeSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pull historical weather information for date ranges up to 90 days (daily data) or 7 days (hourly data). Get consistent formatting across your specified date range with reliable historical weather patterns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .weather_time_series(
            &WeatherTimeSeriesQueryRequest {
                api_key: "apiKey".to_string(),
                start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                precision: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<WeatherTimeSeriesRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` — Starting date for the data in YYYY-MM-DD format. Historical dates must be past dates only. Current or future dates are not allowed for historical data. Data available from 1940 onwards. For precision=daily, the difference between endDate and startDate must not exceed 90 days. For precision=hourly, the difference must not exceed 7 days.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `String` — End date for the data in YYYY-MM-DD format. Historical dates must be past dates only. Current or future dates are not allowed for historical data. Data available from 1940 onwards. For precision=daily, the difference between endDate and startDate must not exceed 90 days. For precision=hourly, the difference must not exceed 7 days.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — City name, place name, or full address.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP(v4 or v6) address for location inference.
    
</dd>
</dl>

<dl>
<dd>

**precision:** `Option<WeatherTimeSeriesRequestPrecision>` — Precision of the data.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">marine_weather</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;MarineWeatherRequestFormat&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;f64&gt;&gt;, long: Option&lt;Option&lt;f64&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, precision: Option&lt;Option&lt;MarineWeatherRequestPrecision&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;MarineWeatherResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Provides hourly forecasts of marine conditions including wave heights, wave directions, wave periods, swell info, sea surface temperatures, and ocean currents. Supports multiple geographical points and returns daily max wave statistics for up to 7 days. Ideal for maritime planning, navigation, and coastal activities.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .marine_weather(
            &MarineWeatherQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                start_date: None,
                end_date: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                precision: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<MarineWeatherRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Starting date for marine forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — End date for marine forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — City name, place name, or full address.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP(v4 or v6) address for location inference.
    
</dd>
</dl>

<dl>
<dd>

**precision:** `Option<MarineWeatherRequestPrecision>` — Precision of the marine data.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">air_quality</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;AirQualityRequestFormat&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;f64&gt;&gt;, long: Option&lt;Option&lt;f64&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, precision: Option&lt;Option&lt;AirQualityRequestPrecision&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;AirQualityResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Monitor and predict air quality conditions using European and US AQI standards. Track pollutant concentrations including PM10, PM2.5, carbon monoxide, nitrogen dioxide, sulfur dioxide, ozone, and dust particles. Get current readings plus hourly forecasts up to 5 days ahead, complete with UV index and aerosol measurements for comprehensive air quality assessment.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .air_quality(
            &AirQualityQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                start_date: None,
                end_date: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                precision: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<AirQualityRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Starting date for AQI forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 5 days.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — End date for AQI forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 5 days.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — City name, place name, or full address.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP(v4 or v6) address for location inference.
    
</dd>
</dl>

<dl>
<dd>

**precision:** `Option<AirQualityRequestPrecision>` — Only hourly precision is supported; returns hourly AQI data for the selected date range.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">flood_forecast</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;FloodForecastRequestFormat&gt;&gt;, start_date: Option&lt;String&gt;, end_date: Option&lt;String&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;f64&gt;&gt;, long: Option&lt;Option&lt;f64&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, precision: Option&lt;Option&lt;FloodForecastRequestPrecision&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;FloodForecastResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Provides flood forecast data for a given location, including river discharge metrics such as mean, median, maximum, minimum, and percentile values (p25, p75). Requires a startDate and endDate, with the date range limited to 16 days. Location can be specified using city name, latitude/longitude, or IP address.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .flood_forecast(
            &FloodForecastQueryRequest {
                api_key: "apiKey".to_string(),
                start_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                end_date: NaiveDate::parse_from_str("2023-01-15", "%Y-%m-%d").unwrap(),
                format: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                precision: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<FloodForecastRequestFormat>` — Response format returned by the API.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` — Starting date for flood forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `String` — End date for flood forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — City name, place name, or full address.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<f64>` — Latitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<f64>` — Longitude of the location.
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP(v4 or v6) address for location inference.
    
</dd>
</dl>

<dl>
<dd>

**precision:** `Option<FloodForecastRequestPrecision>` — Only daily precision is supported; returns flood forecast data for the selected date range.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Timezone for the results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_countries</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetCountriesRequestFormat&gt;&gt;, region: Option&lt;Option&lt;String&gt;&gt;, subregion: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetCountriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve countries, optionally filtered by region or subregion.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_countries(
            &GetCountriesQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                region: None,
                subregion: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetCountriesRequestFormat>` — Format of the response
    
</dd>
</dl>

<dl>
<dd>

**region:** `Option<String>` — Optional filter to return countries within a specific region from the region endpoint.
    
</dd>
</dl>

<dl>
<dd>

**subregion:** `Option<String>` — Optional filter to return countries within a specific subregion from the subregion endpoint.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_country_details</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetCountryDetailsRequestFormat&gt;&gt;, country: Option&lt;String&gt;) -> Result&lt;GetCountryDetailsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_country_details(
            &GetCountryDetailsQueryRequest {
                api_key: "apiKey".to_string(),
                country: "country".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetCountryDetailsRequestFormat>` — Format of the response
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_regions</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetRegionsRequestFormat&gt;&gt;) -> Result&lt;GetRegionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_regions(
            &GetRegionsQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetRegionsRequestFormat>` — Format of the response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_subregions</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetSubregionsRequestFormat&gt;&gt;, region: Option&lt;String&gt;) -> Result&lt;GetSubregionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_subregions(
            &GetSubregionsQueryRequest {
                api_key: "apiKey".to_string(),
                region: "region".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetSubregionsRequestFormat>` — Format of the response
    
</dd>
</dl>

<dl>
<dd>

**region:** `String` — Name of the region.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_admin_levels</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetAdminLevelsRequestFormat&gt;&gt;, country: Option&lt;String&gt;) -> Result&lt;GetAdminLevelsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve administrative units based on ISO 3166-1 alpha-2 country code.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_admin_levels(
            &GetAdminLevelsQueryRequest {
                api_key: "apiKey".to_string(),
                country: "country".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetAdminLevelsRequestFormat>` — Format of the response
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_admin_units</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetAdminUnitsRequestFormat&gt;&gt;, country: Option&lt;String&gt;) -> Result&lt;GetAdminUnitsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve administrative divisions for a given country using ISO 3166-1 alpha-2 country codes. You can optionally filter by administrative levels.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_admin_units(
            &GetAdminUnitsQueryRequest {
                api_key: "apiKey".to_string(),
                country: "country".to_string(),
                format: None,
                admin_levels: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetAdminUnitsRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**admin_levels:** `Option<String>` — Comma-separated list to filter results by one or more administrative levels.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_admin_unit_details</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetAdminUnitDetailsRequestFormat&gt;&gt;, country: Option&lt;String&gt;, admin_unit: Option&lt;String&gt;) -> Result&lt;GetAdminUnitDetailsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed administrative unit information by country and optionally filtered by admin code.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_admin_unit_details(
            &GetAdminUnitDetailsQueryRequest {
                api_key: "apiKey".to_string(),
                country: "country".to_string(),
                admin_unit: "admin_unit".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetAdminUnitDetailsRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**admin_unit:** `String` — Optional admin code to fetch details for a specific administrative unit.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_cities</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;GetCitiesRequestFormat&gt;&gt;, country: Option&lt;String&gt;, admin_unit: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetCitiesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of cities within a country, optionally filtered by an administrative unit code.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_cities(
            &GetCitiesQueryRequest {
                api_key: "apiKey".to_string(),
                country: "country".to_string(),
                format: None,
                admin_unit: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetCitiesRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**country:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**admin_unit:** `Option<String>` — Administrative unit code used to filter cities within a specific region.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_supported_flags</a>(api_key: Option&lt;String&gt;) -> Result&lt;Vec&lt;GetSupportedFlagsResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get list of all supported flags with their metadata
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_supported_flags(
            &GetSupportedFlagsQueryRequest {
                api_key: "apiKey".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_flags</a>(api_key: Option&lt;String&gt;, name: Option&lt;String&gt;, shape: Option&lt;GetFlagsRequestShape&gt;, format: Option&lt;Option&lt;GetFlagsRequestFormat&gt;&gt;, size: Option&lt;Option&lt;GetFlagsRequestSize&gt;&gt;, type_: Option&lt;GetFlagsRequestType&gt;) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the flag for a specific country
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .get_flags(
            &GetFlagsQueryRequest {
                api_key: "apiKey".to_string(),
                name: "name".to_string(),
                shape: GetFlagsRequestShape::Flat,
                r#type: GetFlagsRequestType::Country,
                format: None,
                size: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Country code in ISO 3166-1 alpha-2 format.
    
</dd>
</dl>

<dl>
<dd>

**shape:** `GetFlagsRequestShape` — Flag shape. One of: `'flat'` or `'round'`.
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<GetFlagsRequestFormat>` — Flag format. Applicable only for PNG or WEBP formats. Default is png.
    
</dd>
</dl>

<dl>
<dd>

**size:** `Option<GetFlagsRequestSize>` — Flag size in pixels. Valid options: `16px`, `24px`, `32px`, `48px`, `64px`. Applicable only for PNG or WEBP formats.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `GetFlagsRequestType` — Type of flag. One of: `country` or `organization`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">timezone_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;TimezoneLookupRequestFormat&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, tz: Option&lt;Option&lt;String&gt;&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;String&gt;&gt;, long: Option&lt;Option&lt;String&gt;&gt;, lang: Option&lt;Option&lt;TimezoneLookupRequestLang&gt;&gt;, iata_code: Option&lt;Option&lt;String&gt;&gt;, icao_code: Option&lt;Option&lt;String&gt;&gt;, lo_code: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;TimezoneLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve current time, date, and timezone-related information by specifying a timezone name, location address, location coordinates, IP address, or use the client IP address if no parameter is passed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .timezone_lookup(
            &TimezoneLookupQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                ip: None,
                tz: None,
                location: None,
                lat: None,
                long: None,
                lang: None,
                iata_code: None,
                icao_code: None,
                lo_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<TimezoneLookupRequestFormat>` — Format of the response
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IPv4 or IPv6 address to extract timezone information.
    
</dd>
</dl>

<dl>
<dd>

**tz:** `Option<String>` — Timezone name (e.g., "Asia/Kolkata") to retrieve information directly.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — Location string (preferably city and country) to extract timezone.
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<String>` — Latitude for geolocation lookup.
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<String>` — Longitude for geolocation lookup.
    
</dd>
</dl>

<dl>
<dd>

**lang:** `Option<TimezoneLookupRequestLang>` — Language code for response localization (default is "en").
    
</dd>
</dl>

<dl>
<dd>

**iata_code:** `Option<String>` — 3-letter IATA airport code (e.g., JFK).
    
</dd>
</dl>

<dl>
<dd>

**icao_code:** `Option<String>` — 4-letter ICAO airport code (e.g., KJFK).
    
</dd>
</dl>

<dl>
<dd>

**lo_code:** `Option<String>` — 5-letter UN/LO city code.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">timezone_convert</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;TimezoneConvertRequestFormat&gt;&gt;, time: Option&lt;Option&lt;String&gt;&gt;, tz_from: Option&lt;Option&lt;String&gt;&gt;, tz_to: Option&lt;Option&lt;String&gt;&gt;, lat_from: Option&lt;Option&lt;f64&gt;&gt;, long_from: Option&lt;Option&lt;f64&gt;&gt;, lat_to: Option&lt;Option&lt;f64&gt;&gt;, long_to: Option&lt;Option&lt;f64&gt;&gt;, location_from: Option&lt;Option&lt;String&gt;&gt;, location_to: Option&lt;Option&lt;String&gt;&gt;, iata_from: Option&lt;Option&lt;String&gt;&gt;, iata_to: Option&lt;Option&lt;String&gt;&gt;, icao_from: Option&lt;Option&lt;String&gt;&gt;, icao_to: Option&lt;Option&lt;String&gt;&gt;, locode_from: Option&lt;Option&lt;String&gt;&gt;, locode_to: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;TimezoneConvertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Converts a given time from one timezone to another using various input types like timezone name, coordinates, location, or codes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .timezone_convert(
            &TimezoneConvertQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                time: None,
                tz_from: None,
                tz_to: None,
                lat_from: None,
                long_from: None,
                lat_to: None,
                long_to: None,
                location_from: None,
                location_to: None,
                iata_from: None,
                iata_to: None,
                icao_from: None,
                icao_to: None,
                locode_from: None,
                locode_to: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<TimezoneConvertRequestFormat>` — Format of the response .
    
</dd>
</dl>

<dl>
<dd>

**time:** `Option<String>` — Time to convert in `yyyy-MM-dd HH:mm` or `yyyy-MM-dd HH:mm:ss` format.
    
</dd>
</dl>

<dl>
<dd>

**tz_from:** `Option<String>` — Source timezone name (e.g., `Asia/Kolkata`).
    
</dd>
</dl>

<dl>
<dd>

**tz_to:** `Option<String>` — Target timezone name (e.g., `America/New_York`).
    
</dd>
</dl>

<dl>
<dd>

**lat_from:** `Option<f64>` — Latitude of source location.
    
</dd>
</dl>

<dl>
<dd>

**long_from:** `Option<f64>` — Longitude of source location.
    
</dd>
</dl>

<dl>
<dd>

**lat_to:** `Option<f64>` — Latitude of target location.
    
</dd>
</dl>

<dl>
<dd>

**long_to:** `Option<f64>` — Longitude of target location.
    
</dd>
</dl>

<dl>
<dd>

**location_from:** `Option<String>` — From location (city/country).
    
</dd>
</dl>

<dl>
<dd>

**location_to:** `Option<String>` — To location (city/country).
    
</dd>
</dl>

<dl>
<dd>

**iata_from:** `Option<String>` — From IATA airport code (e.g., JFK).
    
</dd>
</dl>

<dl>
<dd>

**iata_to:** `Option<String>` — To IATA airport code.
    
</dd>
</dl>

<dl>
<dd>

**icao_from:** `Option<String>` — From ICAO airport code (e.g., KJFK).
    
</dd>
</dl>

<dl>
<dd>

**icao_to:** `Option<String>` — To ICAO airport code.
    
</dd>
</dl>

<dl>
<dd>

**locode_from:** `Option<String>` — From UN/LO CODE.
    
</dd>
</dl>

<dl>
<dd>

**locode_to:** `Option<String>` — To UN/LO CODE.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">user_agent_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;UserAgentLookupRequestFormat&gt;&gt;) -> Result&lt;UserAgentLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Parse User Agent string to get detailed browser, device, and operating system information
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .user_agent_lookup(
            &UserAgentLookupQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<UserAgentLookupRequestFormat>` — Format of the response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">bulk_user_agent_lookup</a>(request: BulkUserAgentLookupRequest, api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;BulkUserAgentLookupRequestFormat&gt;&gt;) -> Result&lt;Vec&lt;BulkUserAgentLookupResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Parse up to `50,000 User-Agent strings` at once in a single request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .bulk_user_agent_lookup(
            &BulkUserAgentLookupRequest {
                api_key: "apiKey".to_string(),
                ua_strings: vec!["uaStrings".to_string()],
                format: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ua_strings:** `Vec<String>` — List of user agent strings to parse
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<BulkUserAgentLookupRequestFormat>` — Format of the response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">ocr_predict</a>(request: OcrPredictRequest, api_key: Option&lt;String&gt;, url: Option&lt;Option&lt;String&gt;&gt;, model: Option&lt;OcrPredictRequestModel&gt;, page_range: Option&lt;Option&lt;String&gt;&gt;, zone: Option&lt;Option&lt;String&gt;&gt;, new_line: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;OcrPredictResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Perform Optical Character Recognition (OCR) on images, PDFs, or ZIP archives. Supports two models: `mini-ocr-v1` for CAPTCHA-optimized OCR and `ocr-v1` for general-purpose document text extraction. Supports zonal OCR to extract text from specific regions of an image.

**Notes:**
- The `zone` query parameter cannot be given with .pdf and .zip types as it can only be applied to single image query.
- The `page_range` query parameter cannot be given in any other type except .pdf types.
- PDFs containing images in them are allowed only for processing.
- The `mini-ocr-v1` model doesn’t support the following query parameters:
    - `page_range` (.pdf types)
    - `zone`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .ocr_predict(
            &OcrPredictRequest {
                api_key: "apiKey".to_string(),
                model: OcrPredictRequestModel::MiniOcrV1,
                ocr_predict_request_model: OcrPredictRequestModel::MiniOcrV1,
                url: None,
                page_range: None,
                zone: None,
                new_line: None,
                ocr_predict_request_url: None,
                ocr_predict_request_page_range: None,
                ocr_predict_request_zone: None,
                ocr_predict_request_new_line: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ocr_predict_request_url:** `Option<String>` — URL of the image or PDF (required if `file` not provided)
    
</dd>
</dl>

<dl>
<dd>

**ocr_predict_request_model:** `OcrPredictRequestModel` — OCR model to use. `mini-ocr-v1` for CAPTCHA OCR, `ocr-v1` for general OCR
    
</dd>
</dl>

<dl>
<dd>

**ocr_predict_request_page_range:** `Option<String>` — Specify page range for multi-page PDFs (e.g., '1,3,5-10' or 'allpages'). **Note:** This parameter can only be used with .pdf file types.
    
</dd>
</dl>

<dl>
<dd>

**ocr_predict_request_zone:** `Option<String>` — Define OCR zones using coordinates (top:left:height:width). Multiple zones can be defined using commas. Only available for model 'ocr-v1'. **Note:** This parameter cannot be used with .pdf and .zip file types as it can only be applied to single image queries.
    
</dd>
</dl>

<dl>
<dd>

**ocr_predict_request_new_line:** `Option<i64>` — Set to 1 to split output text into individual lines (default: 0)
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — URL of the image or PDF (required if `file` not provided)
    
</dd>
</dl>

<dl>
<dd>

**model:** `OcrPredictRequestModel` — OCR model to use.
    
</dd>
</dl>

<dl>
<dd>

**page_range:** `Option<String>` — Specify page range for multi-page PDFs (e.g., '1,3,5-10' or 'allpages'). **Note:** This parameter can only be used with .pdf file types.
    
</dd>
</dl>

<dl>
<dd>

**zone:** `Option<String>` — Define OCR zones using coordinates (top:left:height:width). Multiple zones can be defined using commas. Only available for model 'ocr-v1'. **Note:** This parameter cannot be used with .pdf and .zip file types as it can only be applied to single image queries.
    
</dd>
</dl>

<dl>
<dd>

**new_line:** `Option<i64>` — Set to 1 to split output text into individual lines (default: 0)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">grammar_detect</a>(request: GrammarDetectRequest, api_key: Option&lt;String&gt;) -> Result&lt;GrammarDetectResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Analyze text for grammar errors and return the exact words flagged as grammatically incorrect with zero-based word positions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client.grammar_detect(&GrammarDetectRequest {
        api_key: "apiKey".to_string(),
        text: "The global mental is health crisis is now a serious and compelex problem. It need quick and ongoing action from policymakers, healthcare workers, and the whole society.".to_string()
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**text:** `String` — Text to analyze for grammar errors
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">grammar_correct</a>(request: GrammarCorrectRequest, api_key: Option&lt;String&gt;) -> Result&lt;GrammarCorrectResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Submit text with grammatical issues and receive a clean grammar-corrected result for proofreading and content workflows.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client.grammar_correct(&GrammarCorrectRequest {
        api_key: "apiKey".to_string(),
        text: "The global mental is health crisis is now a serious and compelex problem. It need quick and ongoing action from policymakers, healthcare workers, and the whole society.".to_string()
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**text:** `String` — Text to correct
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">weak_words_detect</a>(request: WeakWordsDetectRequest, api_key: Option&lt;String&gt;) -> Result&lt;WeakWordsDetectResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Analyze text and return weak, vague, or filler words with zero-based word positions to help writers produce clearer and more concise content.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .weak_words_detect(
            &WeakWordsDetectRequest {
                api_key: "apiKey".to_string(),
                text:
                    "Many people cannot get the support they need to handle their conditions well."
                        .to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**text:** `String` — Text to analyze for weak words
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">readability_score</a>(request: ReadabilityScoreRequest, api_key: Option&lt;String&gt;, target: Option&lt;Option&lt;ReadabilityScoreRequestTarget&gt;&gt;, exclude: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ReadabilityScoreResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Analyze text readability using industry-standard formulas including Flesch Reading Ease, Flesch-Kincaid Grade Level, Gunning Fog Index, SMOG Index, Coleman-Liau Index, and Automated Readability Index.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client.readability_score(&ReadabilityScoreRequest {
        api_key: "apiKey".to_string(),
        text: "The global mental is health crisis is now a serious and compelex problem. It needs quick and ongoing action from policymakers, healthcare workers, and the whole society.".to_string(),
        target: None,
        exclude: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**text:** `String` — Text to analyze for readability
    
</dd>
</dl>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**target:** `Option<ReadabilityScoreRequestTarget>` — Target audience used to tune sentence difficulty levels
    
</dd>
</dl>

<dl>
<dd>

**exclude:** `Option<String>` — Comma-separated response sections to omit. Possible values are readability_scores, sentence_readability, readability_grade
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">astronomy_lookup</a>(api_key: Option&lt;String&gt;, format: Option&lt;Option&lt;AstronomyLookupRequestFormat&gt;&gt;, location: Option&lt;Option&lt;String&gt;&gt;, lat: Option&lt;Option&lt;String&gt;&gt;, long: Option&lt;Option&lt;String&gt;&gt;, ip: Option&lt;Option&lt;String&gt;&gt;, lang: Option&lt;Option&lt;String&gt;&gt;, date: Option&lt;Option&lt;String&gt;&gt;, elevation: Option&lt;Option&lt;f64&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;AstronomyLookupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve sunrise and sunset times, current position of the moon, and other related information by specifying a location address, location coordinates, IP address, or using the client IP address if no parameter is passed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apifreaks::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiFreaks::new(config).expect("Failed to build client");
    client
        .astronomy_lookup(
            &AstronomyLookupQueryRequest {
                api_key: "apiKey".to_string(),
                format: None,
                location: None,
                lat: None,
                long: None,
                ip: None,
                lang: None,
                date: None,
                elevation: None,
                time_zone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key:** `String` — Your API key
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<AstronomyLookupRequestFormat>` — Format of the response.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — Location name or address
    
</dd>
</dl>

<dl>
<dd>

**lat:** `Option<String>` — Latitude for location coordinates
    
</dd>
</dl>

<dl>
<dd>

**long:** `Option<String>` — Longitude for location coordinates
    
</dd>
</dl>

<dl>
<dd>

**ip:** `Option<String>` — IP address for location detection
    
</dd>
</dl>

<dl>
<dd>

**lang:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` — Date for astronomy data (YYYY-MM-DD)
    
</dd>
</dl>

<dl>
<dd>

**elevation:** `Option<f64>` — Timezone of the location for which astronomy data is required
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

