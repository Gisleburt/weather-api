use juniper::GraphQLEnum;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq, Serialize, GraphQLEnum)]
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

impl FromStr for Direction {
    type Err = DirectionConversionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directions() {
        assert_eq!(Direction::from_str("N").unwrap(), Direction::North);
        assert_eq!(
            Direction::from_str("NNE").unwrap(),
            Direction::NorthNorthEast
        );
        assert_eq!(Direction::from_str("NE").unwrap(), Direction::NorthEast);
        assert_eq!(
            Direction::from_str("ENE").unwrap(),
            Direction::EastNorthEast
        );
        assert_eq!(Direction::from_str("E").unwrap(), Direction::East);
        assert_eq!(
            Direction::from_str("ESE").unwrap(),
            Direction::EastSouthEast
        );
        assert_eq!(Direction::from_str("SE").unwrap(), Direction::SouthEast);
        assert_eq!(
            Direction::from_str("SSE").unwrap(),
            Direction::SouthSouthEast
        );
        assert_eq!(Direction::from_str("S").unwrap(), Direction::South);
        assert_eq!(
            Direction::from_str("SSW").unwrap(),
            Direction::SouthSouthWest
        );
        assert_eq!(Direction::from_str("SW").unwrap(), Direction::SouthWest);
        assert_eq!(
            Direction::from_str("WSW").unwrap(),
            Direction::WestSouthWest
        );
        assert_eq!(Direction::from_str("W").unwrap(), Direction::West);
        assert_eq!(
            Direction::from_str("WNW").unwrap(),
            Direction::WestNorthWest
        );
        assert_eq!(Direction::from_str("NW").unwrap(), Direction::NorthWest);
        assert_eq!(
            Direction::from_str("NNW").unwrap(),
            Direction::NorthNorthWest
        );
    }

    #[test]
    fn test_direction_error() {
        assert_eq!(
            Direction::from_str("not a direction").unwrap_err(),
            DirectionConversionError::InvalidDirection("not a direction".to_string())
        );
    }
}
