use juniper::GraphQLEnum;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Serialize, GraphQLEnum)]
pub enum WeatherType {
    NotAvailable,
    ClearNight,
    SunnyDay,
    PartlyCloudyNight,
    PartlyCloudyDay,
    // NotUsed, // This code is documented as not used therefore we will treat it as an error
    Mist,
    Fog,
    Cloudy,
    Overcast,
    LightRainShowerNight,
    LightRainShowerDay,
    Drizzle,
    LightRain,
    HeavyRainShowerNight,
    HeavyRainShowerDay,
    HeavyRain,
    SleetShowerNight,
    SleetShowerDay,
    Sleet,
    HailShowerNight,
    HailShowerDay,
    Hail,
    LightSnowShowerNight,
    LightSnowShowerDay,
    LightSnow,
    HeavySnowShowerNight,
    HeavySnowShowerDay,
    HeavySnow,
    ThunderShowerNight,
    ThunderShowerDay,
    Thunder,
}

#[derive(Error, Debug, PartialEq)]
pub enum WeatherTypeCodeConversionError {
    #[error("invalid weather type code, found {0}")]
    InvalidCode(String),
    #[error("weather code documented unused, found {0}")]
    UnusedCode(String),
}

impl FromStr for WeatherType {
    type Err = WeatherTypeCodeConversionError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code {
            "NA" => Ok(WeatherType::NotAvailable),
            "0" => Ok(WeatherType::ClearNight),
            "1" => Ok(WeatherType::SunnyDay),
            "2" => Ok(WeatherType::PartlyCloudyNight),
            "3" => Ok(WeatherType::PartlyCloudyDay),
            "4" => Err(WeatherTypeCodeConversionError::UnusedCode(code.to_string())),
            "5" => Ok(WeatherType::Mist),
            "6" => Ok(WeatherType::Fog),
            "7" => Ok(WeatherType::Cloudy),
            "8" => Ok(WeatherType::Overcast),
            "9" => Ok(WeatherType::LightRainShowerNight),
            "10" => Ok(WeatherType::LightRainShowerDay),
            "11" => Ok(WeatherType::Drizzle),
            "12" => Ok(WeatherType::LightRain),
            "13" => Ok(WeatherType::HeavyRainShowerNight),
            "14" => Ok(WeatherType::HeavyRainShowerDay),
            "15" => Ok(WeatherType::HeavyRain),
            "16" => Ok(WeatherType::SleetShowerNight),
            "17" => Ok(WeatherType::SleetShowerDay),
            "18" => Ok(WeatherType::Sleet),
            "19" => Ok(WeatherType::HailShowerNight),
            "20" => Ok(WeatherType::HailShowerDay),
            "21" => Ok(WeatherType::Hail),
            "22" => Ok(WeatherType::LightSnowShowerNight),
            "23" => Ok(WeatherType::LightSnowShowerDay),
            "24" => Ok(WeatherType::LightSnow),
            "25" => Ok(WeatherType::HeavySnowShowerNight),
            "26" => Ok(WeatherType::HeavySnowShowerDay),
            "27" => Ok(WeatherType::HeavySnow),
            "28" => Ok(WeatherType::ThunderShowerNight),
            "29" => Ok(WeatherType::ThunderShowerDay),
            "30" => Ok(WeatherType::Thunder),
            _ => Err(WeatherTypeCodeConversionError::InvalidCode(
                code.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid_weather_codes() {
        assert_eq!(
            WeatherType::from_str("NA").unwrap(),
            WeatherType::NotAvailable
        );
        assert_eq!(WeatherType::from_str("0").unwrap(), WeatherType::ClearNight);
        assert_eq!(WeatherType::from_str("1").unwrap(), WeatherType::SunnyDay);
        assert_eq!(
            WeatherType::from_str("2").unwrap(),
            WeatherType::PartlyCloudyNight
        );
        assert_eq!(
            WeatherType::from_str("3").unwrap(),
            WeatherType::PartlyCloudyDay
        );
        assert_eq!(WeatherType::from_str("5").unwrap(), WeatherType::Mist);
        assert_eq!(WeatherType::from_str("6").unwrap(), WeatherType::Fog);
        assert_eq!(WeatherType::from_str("7").unwrap(), WeatherType::Cloudy);
        assert_eq!(WeatherType::from_str("8").unwrap(), WeatherType::Overcast);
        assert_eq!(
            WeatherType::from_str("9").unwrap(),
            WeatherType::LightRainShowerNight
        );
        assert_eq!(
            WeatherType::from_str("10").unwrap(),
            WeatherType::LightRainShowerDay
        );
        assert_eq!(WeatherType::from_str("11").unwrap(), WeatherType::Drizzle);
        assert_eq!(WeatherType::from_str("12").unwrap(), WeatherType::LightRain);
        assert_eq!(
            WeatherType::from_str("13").unwrap(),
            WeatherType::HeavyRainShowerNight
        );
        assert_eq!(
            WeatherType::from_str("14").unwrap(),
            WeatherType::HeavyRainShowerDay
        );
        assert_eq!(WeatherType::from_str("15").unwrap(), WeatherType::HeavyRain);
        assert_eq!(
            WeatherType::from_str("16").unwrap(),
            WeatherType::SleetShowerNight
        );
        assert_eq!(
            WeatherType::from_str("17").unwrap(),
            WeatherType::SleetShowerDay
        );
        assert_eq!(WeatherType::from_str("18").unwrap(), WeatherType::Sleet);
        assert_eq!(
            WeatherType::from_str("19").unwrap(),
            WeatherType::HailShowerNight
        );
        assert_eq!(
            WeatherType::from_str("20").unwrap(),
            WeatherType::HailShowerDay
        );
        assert_eq!(WeatherType::from_str("21").unwrap(), WeatherType::Hail);
        assert_eq!(
            WeatherType::from_str("22").unwrap(),
            WeatherType::LightSnowShowerNight
        );
        assert_eq!(
            WeatherType::from_str("23").unwrap(),
            WeatherType::LightSnowShowerDay
        );
        assert_eq!(WeatherType::from_str("24").unwrap(), WeatherType::LightSnow);
        assert_eq!(
            WeatherType::from_str("25").unwrap(),
            WeatherType::HeavySnowShowerNight
        );
        assert_eq!(
            WeatherType::from_str("26").unwrap(),
            WeatherType::HeavySnowShowerDay
        );
        assert_eq!(WeatherType::from_str("27").unwrap(), WeatherType::HeavySnow);
        assert_eq!(
            WeatherType::from_str("28").unwrap(),
            WeatherType::ThunderShowerNight
        );
        assert_eq!(
            WeatherType::from_str("29").unwrap(),
            WeatherType::ThunderShowerDay
        );
        assert_eq!(WeatherType::from_str("30").unwrap(), WeatherType::Thunder);
    }

    #[test]
    fn test_unused_code() {
        let result = WeatherType::from_str("4");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            WeatherTypeCodeConversionError::UnusedCode("4".to_string())
        );
    }

    #[test]
    fn test_unknown_code() {
        let result = WeatherType::from_str("An invalid code");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            WeatherTypeCodeConversionError::InvalidCode("An invalid code".to_string())
        );
    }
}
