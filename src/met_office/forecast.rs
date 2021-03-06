use crate::met_office::forecast_response::Rep;
use crate::met_office::{
    direction::{Direction, DirectionConversionError},
    uv::{UvCodeConversionError, UvIndex},
    visibility::{Visibility, VisibilityCodeConversionError},
    weather_type::{WeatherType, WeatherTypeCodeConversionError},
};
use chrono::{Duration, NaiveDateTime};
use juniper::GraphQLObject;
use serde::Serialize;
use std::convert::{TryFrom};
use std::ops::Add;
use std::str::FromStr;
use thiserror::Error;

#[derive(GraphQLObject, Serialize)]
pub struct Forecast {
    location_id: i32,
    timestamp: NaiveDateTime,
    feels_like_temperature: i32,
    wind_gust: i32,
    screen_relative_humidity: i32,
    temperature: i32,
    visibility: Visibility,
    wind_direction: Direction,
    wind_speed: i32,
    max_uv_index: UvIndex,
    weather_type: WeatherType,
    precipitation_probability: i32,
}

#[derive(Debug, Error)]
pub enum ForecastConversionError {
    #[error("Direction Conversion Error: {0:?}")]
    DirectionConversionError(#[from] DirectionConversionError),
    #[error("Uv Code Conversion Error: {0:?}")]
    UvCodeConversionError(#[from] UvCodeConversionError),
    #[error("Visibility Code Conversion Error: {0:?}")]
    VisibilityCodeConversionError(#[from] VisibilityCodeConversionError),
    #[error("Weather Type Code Conversion Error: {0:?}")]
    WeatherTypeCodeConversionError(#[from] WeatherTypeCodeConversionError),
    #[error("Could not parse the date: {0}")]
    DateTimeParseError(String),
    #[error("Could not parse number: {0:?}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

impl TryFrom<(i32, &str, &Rep)> for Forecast {
    type Error = ForecastConversionError;

    fn try_from(
        (location_id, date_time_str, weather): (i32, &str, &Rep),
    ) -> Result<Self, Self::Error> {
        let date_time = format!("{} {}", &date_time_str[..10], "00:00:00");
        let minutes = Duration::minutes(weather.dollar.parse()?);
        Ok(Self {
            location_id,
            timestamp: NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| ForecastConversionError::DateTimeParseError(date_time.to_string()))?
                .add(minutes),
            feels_like_temperature: weather.f.parse()?,
            wind_gust: weather.g.parse()?,
            screen_relative_humidity: weather.h.parse()?,
            temperature: weather.t.parse()?,
            visibility: Visibility::from_str(&weather.v)?,
            wind_direction: Direction::from_str(&weather.d)?,
            wind_speed: weather.s.parse()?,
            max_uv_index: UvIndex::from_str(&weather.u)?,
            weather_type: WeatherType::from_str(&weather.w)?,
            precipitation_probability: weather.pp.parse()?,
        })
    }
}
