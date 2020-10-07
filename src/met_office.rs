mod location;
mod uv;
mod visibility;
mod weather_type;

use isahc::prelude::*;
use thiserror::Error;

pub use location::Location;
use location::LocationsResponse;

const MET_BASE: &str = "http://datapoint.metoffice.gov.uk/public/data";

#[derive(Debug, Error)]
pub enum MetApiError {
    #[error("http error: {0:?}")]
    HttpError(#[from] isahc::Error),
    #[error("parse error: {0:?}")]
    ParseError(#[from] serde_json::Error),
}

pub struct MetApi {
    pub api_key: String,
}

impl MetApi {
    pub fn new(api_key: String) -> MetApi {
        MetApi { api_key }
    }

    fn make_request(&self, uri: String) -> Result<Response<Body>, MetApiError> {
        Ok(isahc::get(uri)?)
    }

    pub fn forecast_site_list(&self) -> Result<Vec<Location>, MetApiError> {
        let response: LocationsResponse = self
            .make_request(format!(
                "{}/val/wxfcs/all/json/sitelist?key={}",
                MET_BASE, self.api_key
            ))?
            .json()?;
        Ok(response.locations.location)
    }
}
