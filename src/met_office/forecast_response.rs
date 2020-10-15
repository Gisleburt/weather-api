use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForecastResponse {
    pub site_rep: SiteRep,
}

#[derive(Debug, Deserialize)]
pub struct SiteRep {
    #[serde(rename = "Wx")]
    pub wx: Params,
    #[serde(rename = "DV")]
    pub dv: Dv,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(rename = "Param")]
    pub param: Vec<Param>,
}

#[derive(Debug, Deserialize)]
pub struct Param {
    pub name: String,
    pub units: String,
    #[serde(rename = "$")]
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dv {
    #[serde(rename = "dataDate")]
    pub data_date: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "Location")]
    pub location: ForecastLocation,
}

#[derive(Debug, Deserialize)]
pub struct ForecastLocation {
    pub i: String,
    pub lat: String,
    pub lon: String,
    pub name: String,
    pub country: String,
    pub continent: String,
    #[serde(rename = "Period")]
    pub period: Vec<ForecastPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastPeriod {
    #[serde(rename = "type")]
    pub period_type: String,
    pub value: String,
    #[serde(rename = "Rep")]
    pub rep: Vec<Rep>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rep {
    pub pp: String,
    pub s: String,
    pub d: String,
    pub w: String,
    pub h: String,
    pub t: String,
    #[serde(rename = "$")]
    pub dollar: String,
    pub g: String,
    pub u: String,
    pub v: String,
    pub f: String,
}
