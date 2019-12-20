use r2d2_oracle::oracle::{sql_type::FromSql, Error, Row, RowValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Continent {
    name: String,
    area_m2: f64,
}
impl RowValue for Continent {
    fn get(row: &Row) -> Result<Self, Error> {
        Ok(Continent {
            name: row.get("name")?,
            area_m2: row.get("area_m2")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    city_id: usize,
    city_name: String,
    fg_region_id: Option<usize>,
}
impl RowValue for City {
    fn get(row: &Row) -> Result<Self, Error> {
        todo!();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    name: String,
    fg_continent_name: Option<String>,
    fg_capital_city_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct District {
    district_id: usize,
    district_name: String,
    fg_city_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
    region_id: usize,
    region_name: String,
    fg_country_name: Option<String>,
    population: usize,
    area_m2: f64,
    climate: String,
    fg_centre_city_id: Option<usize>,
}
