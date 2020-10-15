use core::fmt;
use serde::Deserialize;
use std::str::FromStr;
use thiserror::Error;

pub enum LocationId {
    All,
    Location(u32),
}

impl fmt::Display for LocationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            LocationId::Location(loc) => write!(f, "{}", loc),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum LocationConversionError {
    #[error("invalid location, expected \"all\" or positive integer, found {0}")]
    InvalidLocation(String),
}

impl FromStr for LocationId {
    type Err = LocationConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "all" {
            Ok(LocationId::All)
        } else {
            let location_id = s
                .parse()
                .map_err(|_| LocationConversionError::InvalidLocation(s.to_string()))?;
            Ok(LocationId::Location(location_id))
        }
    }
}

#[derive(Debug, Deserialize, juniper::GraphQLObject)]
#[serde(rename_all = "snake_case")]
pub struct Location {
    pub id: String,
    pub name: String,
    pub latitude: String,
    pub longitude: String,
    pub elevation: Option<String>,
    pub region: Option<String>,
    pub unitary_auth_area: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationsResponse {
    pub locations: Locations,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Locations {
    pub location: Vec<Location>,
}
