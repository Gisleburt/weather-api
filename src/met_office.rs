mod direction;
mod forecast;
mod forecast_response;
mod forecasts;
mod location;
mod uv;
mod visibility;
mod weather_type;

use isahc::prelude::*;
use thiserror::Error;

pub use forecast::{Forecast, ForecastConversionError};
use forecast_response::ForecastResponse;
pub use forecasts::Forecasts;
use location::LocationsResponse;
pub use location::{Location, LocationId};
use std::convert::TryInto;

const MET_BASE: &str = "http://datapoint.metoffice.gov.uk/public/data";

#[derive(Debug, Error)]
pub enum MetApiError {
    #[error("http error: {0:?}")]
    HttpError(#[from] isahc::Error),
    #[error("parse error: {0:?}")]
    ParseError(#[from] serde_json::Error),
    #[error("parse error: {0:?}")]
    IoError(#[from] std::io::Error),
    #[error("conversion error: {0:?}")]
    ForecastConversionError(#[from] ForecastConversionError),
    #[error("Could not parse number: {0:?}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, MetApiError>;

pub struct MetApi {
    pub api_key: String,
}

impl MetApi {
    pub fn new(api_key: String) -> MetApi {
        MetApi { api_key }
    }

    fn make_request(&self, uri: String) -> Result<Response<Body>> {
        Ok(isahc::get(uri)?)
    }

    pub fn forecast_site_list(&self) -> Result<Vec<Location>> {
        let response: LocationsResponse = self
            .make_request(format!(
                "{}/val/wxfcs/all/json/sitelist?key={}",
                MET_BASE, self.api_key
            ))?
            .json()?;
        Ok(response.locations.location)
    }

    pub fn forecast(&self, location_id: LocationId) -> Result<Forecasts> {
        let mut response = self.make_request(format!(
            "{}/val/wxfcs/all/json/{}?res=3hourly&key={}",
            MET_BASE, location_id, self.api_key
        ))?;

        let forecast_response: ForecastResponse = response.json()?;

        Ok(forecast_response.try_into()?)
    }
}
