pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeocoderSearchResponseItemPoiItem {
    /// Name of the poi (point of interest) of the searched place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Category of the poi, e.g., amenity, place, natural, building, highway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Specific type of the category for the poi, e.g., fast_food, city, park, residential, house_number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GeocoderSearchResponseItemPoiItem {
    pub fn builder() -> GeocoderSearchResponseItemPoiItemBuilder {
        <GeocoderSearchResponseItemPoiItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeocoderSearchResponseItemPoiItemBuilder {
    name: Option<String>,
    category: Option<String>,
    r#type: Option<String>,
}

impl GeocoderSearchResponseItemPoiItemBuilder {
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

    /// Consumes the builder and constructs a [`GeocoderSearchResponseItemPoiItem`].
    pub fn build(self) -> Result<GeocoderSearchResponseItemPoiItem, BuildError> {
        Ok(GeocoderSearchResponseItemPoiItem {
            name: self.name,
            category: self.category,
            r#type: self.r#type,
        })
    }
}
