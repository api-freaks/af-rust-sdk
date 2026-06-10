pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeocoderReverseResponsePoiItem {
    /// Name of the POI of the searched place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Category of the POI, e.g. amenity, place, natural, building, highway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Specific type of the category for the POI, e.g. fast_food, city, park, residential, house_number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GeocoderReverseResponsePoiItem {
    pub fn builder() -> GeocoderReverseResponsePoiItemBuilder {
        <GeocoderReverseResponsePoiItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeocoderReverseResponsePoiItemBuilder {
    name: Option<String>,
    category: Option<String>,
    r#type: Option<String>,
}

impl GeocoderReverseResponsePoiItemBuilder {
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

    /// Consumes the builder and constructs a [`GeocoderReverseResponsePoiItem`].
    pub fn build(self) -> Result<GeocoderReverseResponsePoiItem, BuildError> {
        Ok(GeocoderReverseResponsePoiItem {
            name: self.name,
            category: self.category,
            r#type: self.r#type,
        })
    }
}
