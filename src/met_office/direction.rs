use juniper::GraphQLEnum;
use serde::Serialize;
use std::convert::TryFrom;
use thiserror::Error;

type CompassInitials = str;

#[derive(Serialize, GraphQLEnum)]
pub enum Direction {
    North,
    NorthNorthEast,
    NorthEast,
    EastNorthEast,
    East,
    EastSouthEast,
    SouthEast,
    SouthSouthEast,
    South,
    SouthSouthWest,
    SouthWest,
    WestSouthWest,
    West,
    WestNorthWest,
    NorthWest,
    NorthNorthWest,
}

#[derive(Error, Debug, PartialEq)]
pub enum DirectionConversionError {
    #[error("invalid direction, expected compass initials, found {0}")]
    InvalidDirection(String),
}

impl TryFrom<&CompassInitials> for Direction {
    type Error = DirectionConversionError;

    fn try_from(value: &CompassInitials) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Self::North),
            "NNE" => Ok(Self::NorthNorthEast),
            "NE" => Ok(Self::NorthEast),
            "ENE" => Ok(Self::EastNorthEast),
            "E" => Ok(Self::East),
            "ESE" => Ok(Self::EastSouthEast),
            "SE" => Ok(Self::SouthEast),
            "SSE" => Ok(Self::SouthSouthEast),
            "S" => Ok(Self::South),
            "SSW" => Ok(Self::SouthSouthWest),
            "SW" => Ok(Self::SouthWest),
            "WSW" => Ok(Self::WestSouthWest),
            "W" => Ok(Self::West),
            "WNW" => Ok(Self::WestNorthWest),
            "NW" => Ok(Self::NorthWest),
            "NNW" => Ok(Self::NorthNorthWest),
            _ => Err(DirectionConversionError::InvalidDirection(
                value.to_string(),
            )),
        }
    }
}
