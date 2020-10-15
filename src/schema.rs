use crate::met_office::{Forecast, Location, LocationId, MetApi};
use juniper::FieldResult;
use juniper::RootNode;
use std::str::FromStr;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn api_version() -> &str {
        "0.1.0"
    }

    fn locations(api_key: String) -> FieldResult<Vec<Location>> {
        Ok(MetApi::new(api_key).forecast_site_list()?) // ToDo: Don't leak the error
    }

    fn forecast(api_key: String, location: String) -> FieldResult<Vec<Forecast>> {
        let location_id = LocationId::from_str(&location)?;
        Ok(MetApi::new(api_key).forecast(location_id)?.0) // ToDo: Don't leak the error
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
