pub use crate::prelude::*;

/// Astronomy data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoricalWeatherResponseHistoricalAstronomy {
    /// Calendar date in YYYY-MM-DD format for the astronomical data.
    #[serde(default)]
    pub date: NaiveDate,
    /// Time of true solar midnight in HH:MM format.
    #[serde(default)]
    pub mid_night: String,
    /// Time when astronomical twilight ends and dawn begins in HH:MM format.
    #[serde(default)]
    pub night_end: String,
    /// Time when the Sun becomes visible above the horizon in HH:MM format.
    #[serde(default)]
    pub sunrise: String,
    /// Time when the Sun reaches its highest point in the sky in HH:MM format.
    #[serde(default)]
    pub solar_noon: String,
    /// Time when the Sun disappears below the horizon in HH:MM format.
    #[serde(default)]
    pub sunset: String,
    /// Time when astronomical twilight begins and night starts in HH:MM format.
    #[serde(default)]
    pub night_begin: String,
    /// Total duration of daylight in HH:MM format.
    #[serde(default)]
    pub day_length: String,
    /// Current position of the Sun relative to the horizon (above_horizon, below_horizon, or -).
    #[serde(default)]
    pub sun_status: String,
    /// Current lunar phase (NEW_MOON, WAXING_CRESCENT, FULL_MOON, WANING_GIBBOUS, etc.).
    #[serde(default)]
    pub moon_phase: String,
    /// Time when the Moon becomes visible above the horizon in HH:MM format.
    #[serde(default)]
    pub moonrise: String,
    /// Time when the Moon disappears below the horizon in HH:MM format.
    #[serde(default)]
    pub moonset: String,
    /// Current visibility status of the Moon (visible, not_visible, or -).
    #[serde(default)]
    pub moon_status: String,
}

impl HistoricalWeatherResponseHistoricalAstronomy {
    pub fn builder() -> HistoricalWeatherResponseHistoricalAstronomyBuilder {
        <HistoricalWeatherResponseHistoricalAstronomyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoricalWeatherResponseHistoricalAstronomyBuilder {
    date: Option<NaiveDate>,
    mid_night: Option<String>,
    night_end: Option<String>,
    sunrise: Option<String>,
    solar_noon: Option<String>,
    sunset: Option<String>,
    night_begin: Option<String>,
    day_length: Option<String>,
    sun_status: Option<String>,
    moon_phase: Option<String>,
    moonrise: Option<String>,
    moonset: Option<String>,
    moon_status: Option<String>,
}

impl HistoricalWeatherResponseHistoricalAstronomyBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn mid_night(mut self, value: impl Into<String>) -> Self {
        self.mid_night = Some(value.into());
        self
    }

    pub fn night_end(mut self, value: impl Into<String>) -> Self {
        self.night_end = Some(value.into());
        self
    }

    pub fn sunrise(mut self, value: impl Into<String>) -> Self {
        self.sunrise = Some(value.into());
        self
    }

    pub fn solar_noon(mut self, value: impl Into<String>) -> Self {
        self.solar_noon = Some(value.into());
        self
    }

    pub fn sunset(mut self, value: impl Into<String>) -> Self {
        self.sunset = Some(value.into());
        self
    }

    pub fn night_begin(mut self, value: impl Into<String>) -> Self {
        self.night_begin = Some(value.into());
        self
    }

    pub fn day_length(mut self, value: impl Into<String>) -> Self {
        self.day_length = Some(value.into());
        self
    }

    pub fn sun_status(mut self, value: impl Into<String>) -> Self {
        self.sun_status = Some(value.into());
        self
    }

    pub fn moon_phase(mut self, value: impl Into<String>) -> Self {
        self.moon_phase = Some(value.into());
        self
    }

    pub fn moonrise(mut self, value: impl Into<String>) -> Self {
        self.moonrise = Some(value.into());
        self
    }

    pub fn moonset(mut self, value: impl Into<String>) -> Self {
        self.moonset = Some(value.into());
        self
    }

    pub fn moon_status(mut self, value: impl Into<String>) -> Self {
        self.moon_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoricalWeatherResponseHistoricalAstronomy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](HistoricalWeatherResponseHistoricalAstronomyBuilder::date)
    /// - [`mid_night`](HistoricalWeatherResponseHistoricalAstronomyBuilder::mid_night)
    /// - [`night_end`](HistoricalWeatherResponseHistoricalAstronomyBuilder::night_end)
    /// - [`sunrise`](HistoricalWeatherResponseHistoricalAstronomyBuilder::sunrise)
    /// - [`solar_noon`](HistoricalWeatherResponseHistoricalAstronomyBuilder::solar_noon)
    /// - [`sunset`](HistoricalWeatherResponseHistoricalAstronomyBuilder::sunset)
    /// - [`night_begin`](HistoricalWeatherResponseHistoricalAstronomyBuilder::night_begin)
    /// - [`day_length`](HistoricalWeatherResponseHistoricalAstronomyBuilder::day_length)
    /// - [`sun_status`](HistoricalWeatherResponseHistoricalAstronomyBuilder::sun_status)
    /// - [`moon_phase`](HistoricalWeatherResponseHistoricalAstronomyBuilder::moon_phase)
    /// - [`moonrise`](HistoricalWeatherResponseHistoricalAstronomyBuilder::moonrise)
    /// - [`moonset`](HistoricalWeatherResponseHistoricalAstronomyBuilder::moonset)
    /// - [`moon_status`](HistoricalWeatherResponseHistoricalAstronomyBuilder::moon_status)
    pub fn build(self) -> Result<HistoricalWeatherResponseHistoricalAstronomy, BuildError> {
        Ok(HistoricalWeatherResponseHistoricalAstronomy {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            mid_night: self
                .mid_night
                .ok_or_else(|| BuildError::missing_field("mid_night"))?,
            night_end: self
                .night_end
                .ok_or_else(|| BuildError::missing_field("night_end"))?,
            sunrise: self
                .sunrise
                .ok_or_else(|| BuildError::missing_field("sunrise"))?,
            solar_noon: self
                .solar_noon
                .ok_or_else(|| BuildError::missing_field("solar_noon"))?,
            sunset: self
                .sunset
                .ok_or_else(|| BuildError::missing_field("sunset"))?,
            night_begin: self
                .night_begin
                .ok_or_else(|| BuildError::missing_field("night_begin"))?,
            day_length: self
                .day_length
                .ok_or_else(|| BuildError::missing_field("day_length"))?,
            sun_status: self
                .sun_status
                .ok_or_else(|| BuildError::missing_field("sun_status"))?,
            moon_phase: self
                .moon_phase
                .ok_or_else(|| BuildError::missing_field("moon_phase"))?,
            moonrise: self
                .moonrise
                .ok_or_else(|| BuildError::missing_field("moonrise"))?,
            moonset: self
                .moonset
                .ok_or_else(|| BuildError::missing_field("moonset"))?,
            moon_status: self
                .moon_status
                .ok_or_else(|| BuildError::missing_field("moon_status"))?,
        })
    }
}
