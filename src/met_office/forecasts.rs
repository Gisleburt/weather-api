use crate::met_office::forecast_response::ForecastResponse;
use crate::met_office::{Forecast, ForecastConversionError};
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

pub struct Forecasts(pub Vec<Forecast>);

impl Deref for Forecasts {
    type Target = Vec<Forecast>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Forecasts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<ForecastResponse> for Forecasts {
    type Error = ForecastConversionError;

    fn try_from(value: ForecastResponse) -> Result<Self, Self::Error> {
        let location = value.site_rep.dv.location;
        let location_id = location.i.parse()?;
        let forecasts: std::result::Result<Vec<_>, ForecastConversionError> = location
            .period
            .iter()
            .flat_map(|period| {
                let date_time = period.value.as_str();
                period
                    .rep
                    .iter()
                    .map(move |rep| Forecast::try_from((location_id, date_time, rep)))
            })
            .collect();
        Ok(Self(forecasts?))
    }
}
