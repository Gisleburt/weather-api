mod location;
mod uv;
mod visibility;
mod weather_type;

use isahc::prelude::*;
pub use location::Location;
use location::LocationsResponse;

pub struct MetApi {
    pub api_key: String,
}

const MET_BASE: &str = "http://datapoint.metoffice.gov.uk/public/data";

impl MetApi {
    pub fn new(api_key: String) -> MetApi {
        MetApi { api_key }
    }

    fn make_request(&self, uri: String) -> Response<Body> {
        isahc::get(uri).expect("Could not contact met office")
    }

    pub fn forecast_site_list(&self) -> Vec<Location> {
        let response: LocationsResponse = self
            .make_request(format!(
                "{}/val/wxfcs/all/json/sitelist?key={}",
                MET_BASE, self.api_key
            ))
            .json()
            .expect("Could not read response");
        response.locations.location
    }
}
