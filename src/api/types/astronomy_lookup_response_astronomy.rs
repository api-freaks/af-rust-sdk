pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupResponseAstronomy {
    /// Time zone to receive all time-based data in your preferred local time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Date for astronomy data (YYYY-MM-DD)
    #[serde(default)]
    pub date: String,
    /// Current time (HH:mm:ss.SSS)
    #[serde(default)]
    pub current_time: String,
    /// Midnight time (HH:mm)
    #[serde(default)]
    pub mid_night: String,
    /// Time when night ends (HH:mm)
    #[serde(default)]
    pub night_end: String,
    #[serde(default)]
    pub morning: AstronomyLookupResponseAstronomyMorning,
    /// Sunrise time (HH:mm)
    #[serde(default)]
    pub sunrise: String,
    /// Sunset time (HH:mm)
    #[serde(default)]
    pub sunset: String,
    #[serde(default)]
    pub evening: AstronomyLookupResponseAstronomyEvening,
    /// Time when night begins (HH:mm)
    #[serde(default)]
    pub night_begin: String,
    /// Current status of the sun
    #[serde(default)]
    pub sun_status: String,
    /// Solar noon time (HH:mm)
    #[serde(default)]
    pub solar_noon: String,
    /// Length of the day (HH:mm)
    #[serde(default)]
    pub day_length: String,
    /// Sun altitude angle
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub sun_altitude: f64,
    /// Distance from Earth to Sun
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub sun_distance: f64,
    /// Sun azimuth angle
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub sun_azimuth: f64,
    /// Moon phase
    #[serde(default)]
    pub moon_phase: String,
    /// Moonrise time (HH:mm)
    #[serde(default)]
    pub moonrise: String,
    /// Moonset time (HH:mm)
    #[serde(default)]
    pub moonset: String,
    /// Current status of the moon
    #[serde(default)]
    pub moon_status: String,
    /// Moon altitude angle
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub moon_altitude: f64,
    /// Distance from Earth to Moon
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub moon_distance: f64,
    /// Moon azimuth angle
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub moon_azimuth: f64,
    /// Moon parallactic angle
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub moon_parallactic_angle: f64,
    /// Moon illumination percentage
    #[serde(default)]
    pub moon_illumination_percentage: String,
    /// Moon angle
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub moon_angle: f64,
}

impl AstronomyLookupResponseAstronomy {
    pub fn builder() -> AstronomyLookupResponseAstronomyBuilder {
        <AstronomyLookupResponseAstronomyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupResponseAstronomyBuilder {
    time_zone: Option<String>,
    date: Option<String>,
    current_time: Option<String>,
    mid_night: Option<String>,
    night_end: Option<String>,
    morning: Option<AstronomyLookupResponseAstronomyMorning>,
    sunrise: Option<String>,
    sunset: Option<String>,
    evening: Option<AstronomyLookupResponseAstronomyEvening>,
    night_begin: Option<String>,
    sun_status: Option<String>,
    solar_noon: Option<String>,
    day_length: Option<String>,
    sun_altitude: Option<f64>,
    sun_distance: Option<f64>,
    sun_azimuth: Option<f64>,
    moon_phase: Option<String>,
    moonrise: Option<String>,
    moonset: Option<String>,
    moon_status: Option<String>,
    moon_altitude: Option<f64>,
    moon_distance: Option<f64>,
    moon_azimuth: Option<f64>,
    moon_parallactic_angle: Option<f64>,
    moon_illumination_percentage: Option<String>,
    moon_angle: Option<f64>,
}

impl AstronomyLookupResponseAstronomyBuilder {
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn current_time(mut self, value: impl Into<String>) -> Self {
        self.current_time = Some(value.into());
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

    pub fn morning(mut self, value: AstronomyLookupResponseAstronomyMorning) -> Self {
        self.morning = Some(value);
        self
    }

    pub fn sunrise(mut self, value: impl Into<String>) -> Self {
        self.sunrise = Some(value.into());
        self
    }

    pub fn sunset(mut self, value: impl Into<String>) -> Self {
        self.sunset = Some(value.into());
        self
    }

    pub fn evening(mut self, value: AstronomyLookupResponseAstronomyEvening) -> Self {
        self.evening = Some(value);
        self
    }

    pub fn night_begin(mut self, value: impl Into<String>) -> Self {
        self.night_begin = Some(value.into());
        self
    }

    pub fn sun_status(mut self, value: impl Into<String>) -> Self {
        self.sun_status = Some(value.into());
        self
    }

    pub fn solar_noon(mut self, value: impl Into<String>) -> Self {
        self.solar_noon = Some(value.into());
        self
    }

    pub fn day_length(mut self, value: impl Into<String>) -> Self {
        self.day_length = Some(value.into());
        self
    }

    pub fn sun_altitude(mut self, value: f64) -> Self {
        self.sun_altitude = Some(value);
        self
    }

    pub fn sun_distance(mut self, value: f64) -> Self {
        self.sun_distance = Some(value);
        self
    }

    pub fn sun_azimuth(mut self, value: f64) -> Self {
        self.sun_azimuth = Some(value);
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

    pub fn moon_altitude(mut self, value: f64) -> Self {
        self.moon_altitude = Some(value);
        self
    }

    pub fn moon_distance(mut self, value: f64) -> Self {
        self.moon_distance = Some(value);
        self
    }

    pub fn moon_azimuth(mut self, value: f64) -> Self {
        self.moon_azimuth = Some(value);
        self
    }

    pub fn moon_parallactic_angle(mut self, value: f64) -> Self {
        self.moon_parallactic_angle = Some(value);
        self
    }

    pub fn moon_illumination_percentage(mut self, value: impl Into<String>) -> Self {
        self.moon_illumination_percentage = Some(value.into());
        self
    }

    pub fn moon_angle(mut self, value: f64) -> Self {
        self.moon_angle = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupResponseAstronomy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](AstronomyLookupResponseAstronomyBuilder::date)
    /// - [`current_time`](AstronomyLookupResponseAstronomyBuilder::current_time)
    /// - [`mid_night`](AstronomyLookupResponseAstronomyBuilder::mid_night)
    /// - [`night_end`](AstronomyLookupResponseAstronomyBuilder::night_end)
    /// - [`morning`](AstronomyLookupResponseAstronomyBuilder::morning)
    /// - [`sunrise`](AstronomyLookupResponseAstronomyBuilder::sunrise)
    /// - [`sunset`](AstronomyLookupResponseAstronomyBuilder::sunset)
    /// - [`evening`](AstronomyLookupResponseAstronomyBuilder::evening)
    /// - [`night_begin`](AstronomyLookupResponseAstronomyBuilder::night_begin)
    /// - [`sun_status`](AstronomyLookupResponseAstronomyBuilder::sun_status)
    /// - [`solar_noon`](AstronomyLookupResponseAstronomyBuilder::solar_noon)
    /// - [`day_length`](AstronomyLookupResponseAstronomyBuilder::day_length)
    /// - [`sun_altitude`](AstronomyLookupResponseAstronomyBuilder::sun_altitude)
    /// - [`sun_distance`](AstronomyLookupResponseAstronomyBuilder::sun_distance)
    /// - [`sun_azimuth`](AstronomyLookupResponseAstronomyBuilder::sun_azimuth)
    /// - [`moon_phase`](AstronomyLookupResponseAstronomyBuilder::moon_phase)
    /// - [`moonrise`](AstronomyLookupResponseAstronomyBuilder::moonrise)
    /// - [`moonset`](AstronomyLookupResponseAstronomyBuilder::moonset)
    /// - [`moon_status`](AstronomyLookupResponseAstronomyBuilder::moon_status)
    /// - [`moon_altitude`](AstronomyLookupResponseAstronomyBuilder::moon_altitude)
    /// - [`moon_distance`](AstronomyLookupResponseAstronomyBuilder::moon_distance)
    /// - [`moon_azimuth`](AstronomyLookupResponseAstronomyBuilder::moon_azimuth)
    /// - [`moon_parallactic_angle`](AstronomyLookupResponseAstronomyBuilder::moon_parallactic_angle)
    /// - [`moon_illumination_percentage`](AstronomyLookupResponseAstronomyBuilder::moon_illumination_percentage)
    /// - [`moon_angle`](AstronomyLookupResponseAstronomyBuilder::moon_angle)
    pub fn build(self) -> Result<AstronomyLookupResponseAstronomy, BuildError> {
        Ok(AstronomyLookupResponseAstronomy {
            time_zone: self.time_zone,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            current_time: self
                .current_time
                .ok_or_else(|| BuildError::missing_field("current_time"))?,
            mid_night: self
                .mid_night
                .ok_or_else(|| BuildError::missing_field("mid_night"))?,
            night_end: self
                .night_end
                .ok_or_else(|| BuildError::missing_field("night_end"))?,
            morning: self
                .morning
                .ok_or_else(|| BuildError::missing_field("morning"))?,
            sunrise: self
                .sunrise
                .ok_or_else(|| BuildError::missing_field("sunrise"))?,
            sunset: self
                .sunset
                .ok_or_else(|| BuildError::missing_field("sunset"))?,
            evening: self
                .evening
                .ok_or_else(|| BuildError::missing_field("evening"))?,
            night_begin: self
                .night_begin
                .ok_or_else(|| BuildError::missing_field("night_begin"))?,
            sun_status: self
                .sun_status
                .ok_or_else(|| BuildError::missing_field("sun_status"))?,
            solar_noon: self
                .solar_noon
                .ok_or_else(|| BuildError::missing_field("solar_noon"))?,
            day_length: self
                .day_length
                .ok_or_else(|| BuildError::missing_field("day_length"))?,
            sun_altitude: self
                .sun_altitude
                .ok_or_else(|| BuildError::missing_field("sun_altitude"))?,
            sun_distance: self
                .sun_distance
                .ok_or_else(|| BuildError::missing_field("sun_distance"))?,
            sun_azimuth: self
                .sun_azimuth
                .ok_or_else(|| BuildError::missing_field("sun_azimuth"))?,
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
            moon_altitude: self
                .moon_altitude
                .ok_or_else(|| BuildError::missing_field("moon_altitude"))?,
            moon_distance: self
                .moon_distance
                .ok_or_else(|| BuildError::missing_field("moon_distance"))?,
            moon_azimuth: self
                .moon_azimuth
                .ok_or_else(|| BuildError::missing_field("moon_azimuth"))?,
            moon_parallactic_angle: self
                .moon_parallactic_angle
                .ok_or_else(|| BuildError::missing_field("moon_parallactic_angle"))?,
            moon_illumination_percentage: self
                .moon_illumination_percentage
                .ok_or_else(|| BuildError::missing_field("moon_illumination_percentage"))?,
            moon_angle: self
                .moon_angle
                .ok_or_else(|| BuildError::missing_field("moon_angle"))?,
        })
    }
}
