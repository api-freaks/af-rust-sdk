pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeocoderReverseResponse {
    /// WGS84 latitude value for the location.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lat: f64,
    /// WGS84 longitude value for the location.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lon: f64,
    /// Name for the primary place searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// High-level category of the place, e.g. amenity, place, natural, building, highway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Specific kind within the category, e.g. fast_food, city, park, residential, house_number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Array describing the POI (point of interest) and closely related elements at this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poi: Option<Vec<GeocoderReverseResponsePoiItem>>,
    /// Street or road name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// Intra-city area below city level, e.g. suburb, borough, quarter, ward, district, sector, zone, tehsil, taluka, neighbourhood.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// Postal code of the area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    /// Settlement label appropriate to the country, e.g. city, town, municipality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Second-tier administrative area, e.g. county, district, shire, prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// Subdivision code when available, typically ISO-3166-2 like US-TX or PK-PB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// First-tier administrative area; varies by country, e.g. state, region, province, division, autonomous community.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Two-letter country code in upper case, ISO-3166-1 alpha-2.
    #[serde(default)]
    pub country_code: String,
    /// Country name of the searched place.
    #[serde(default)]
    pub country: String,
    /// Single-line, human-readable address of the location.
    #[serde(default)]
    pub full_address: String,
    /// Extent of the feature as `[lat_min, lat_max, lon_min, lon_max]` (south, north, west, east) in WGS84 decimal degrees.
    #[serde(default)]
    pub bounding_box: Vec<String>,
}

impl GeocoderReverseResponse {
    pub fn builder() -> GeocoderReverseResponseBuilder {
        <GeocoderReverseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeocoderReverseResponseBuilder {
    lat: Option<f64>,
    lon: Option<f64>,
    name: Option<String>,
    category: Option<String>,
    r#type: Option<String>,
    poi: Option<Vec<GeocoderReverseResponsePoiItem>>,
    street: Option<String>,
    area: Option<String>,
    postcode: Option<String>,
    city: Option<String>,
    county: Option<String>,
    state_code: Option<String>,
    state: Option<String>,
    country_code: Option<String>,
    country: Option<String>,
    full_address: Option<String>,
    bounding_box: Option<Vec<String>>,
}

impl GeocoderReverseResponseBuilder {
    pub fn lat(mut self, value: f64) -> Self {
        self.lat = Some(value);
        self
    }

    pub fn lon(mut self, value: f64) -> Self {
        self.lon = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn poi(mut self, value: Vec<GeocoderReverseResponsePoiItem>) -> Self {
        self.poi = Some(value);
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn area(mut self, value: impl Into<String>) -> Self {
        self.area = Some(value.into());
        self
    }

    pub fn postcode(mut self, value: impl Into<String>) -> Self {
        self.postcode = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn county(mut self, value: impl Into<String>) -> Self {
        self.county = Some(value.into());
        self
    }

    pub fn state_code(mut self, value: impl Into<String>) -> Self {
        self.state_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn full_address(mut self, value: impl Into<String>) -> Self {
        self.full_address = Some(value.into());
        self
    }

    pub fn bounding_box(mut self, value: Vec<String>) -> Self {
        self.bounding_box = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeocoderReverseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lat`](GeocoderReverseResponseBuilder::lat)
    /// - [`lon`](GeocoderReverseResponseBuilder::lon)
    /// - [`country_code`](GeocoderReverseResponseBuilder::country_code)
    /// - [`country`](GeocoderReverseResponseBuilder::country)
    /// - [`full_address`](GeocoderReverseResponseBuilder::full_address)
    /// - [`bounding_box`](GeocoderReverseResponseBuilder::bounding_box)
    pub fn build(self) -> Result<GeocoderReverseResponse, BuildError> {
        Ok(GeocoderReverseResponse {
            lat: self.lat.ok_or_else(|| BuildError::missing_field("lat"))?,
            lon: self.lon.ok_or_else(|| BuildError::missing_field("lon"))?,
            name: self.name,
            category: self.category,
            r#type: self.r#type,
            poi: self.poi,
            street: self.street,
            area: self.area,
            postcode: self.postcode,
            city: self.city,
            county: self.county,
            state_code: self.state_code,
            state: self.state,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            full_address: self
                .full_address
                .ok_or_else(|| BuildError::missing_field("full_address"))?,
            bounding_box: self
                .bounding_box
                .ok_or_else(|| BuildError::missing_field("bounding_box"))?,
        })
    }
}
