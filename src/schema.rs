use crate::database_operations::SchemaTable;
use r2d2_oracle::oracle::{
    sql_type::{FromSql, ToSql},
    Error, Row, RowValue,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Continent {
    pub name: String,
    pub area_m2: f64,
}
impl RowValue for Continent {
    fn get(row: &Row) -> Result<Self, Error> {
        Ok(Continent {
            name: row.get("name")?,
            area_m2: row.get("area_m2")?,
        })
    }
}
impl SchemaTable for Continent {
    fn column_names() -> Vec<&'static str> {
        vec!["name", "area_m2"]
    }

    fn table_name() -> &'static str {
        "continents"
    }

    fn values(&self) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(self.name.clone()), Box::new(self.area_m2.clone())]
    }

    fn key_attrs() -> Vec<&'static str> {
        vec!["name"]
    }

    fn key_attr_values(&self) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(self.name.clone())]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct City {
    pub city_id: usize,
    pub city_name: String,
    pub fg_region_id: Option<usize>,
}
impl RowValue for City {
    fn get(row: &Row) -> Result<Self, Error> {
        Ok(City {
            city_id: row.get("city_id")?,
            city_name: row.get("city_name")?,
            fg_region_id: row.get("fg_region_id")?,
        })
    }
}
impl SchemaTable for City {
    fn column_names() -> Vec<&'static str> {
        vec!["city_id", "city_name", "fg_region_id"]
    }

    fn table_name() -> &'static str {
        "cities"
    }

    fn values(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.city_id.clone()),
            Box::new(self.city_name.clone()),
            Box::new(self.fg_region_id.clone()),
        ]
    }

    fn key_attrs() -> Vec<&'static str> {
        vec!["city_id"]
    }

    fn key_attr_values(&self) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(self.city_id.clone())]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub name: String,
    pub fg_continent_name: Option<String>,
    pub fg_capital_city_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct District {
    pub district_id: usize,
    pub district_name: String,
    pub fg_city_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Region {
    pub region_id: usize,
    pub region_name: String,
    pub fg_country_name: Option<String>,
    pub population: usize,
    pub area_m2: f64,
    pub climate: String,
    pub fg_centre_city_id: Option<usize>,
}
