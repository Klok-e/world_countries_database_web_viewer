use crate::database_operations::SchemaTable;
use chrono;
use r2d2_oracle::oracle::{
    sql_type::{OracleType, ToSql},
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
    pub fg_capital_city_id: Option<usize>,
}
impl RowValue for Country {
    fn get(row: &Row) -> Result<Self, Error> {
        Ok(Country {
            name: row.get("name")?,
            fg_continent_name: row.get("fg_continent_name")?,
            fg_capital_city_id: row.get("fg_capital_city_id")?,
        })
    }
}
impl SchemaTable for Country {
    fn column_names() -> Vec<&'static str> {
        vec!["name", "fg_continent_name", "fg_capital_city_id"]
    }

    fn table_name() -> &'static str {
        "countries"
    }

    fn values(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.name.clone()),
            Box::new(self.fg_continent_name.clone()),
            Box::new(self.fg_capital_city_id.clone()),
        ]
    }

    fn key_attrs() -> Vec<&'static str> {
        vec!["name"]
    }

    fn key_attr_values(&self) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(self.name.clone())]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct District {
    pub district_id: usize,
    pub district_name: String,
    pub fg_city_id: Option<usize>,
}
impl RowValue for District {
    fn get(row: &Row) -> Result<Self, Error> {
        Ok(District {
            district_id: row.get("district_id")?,
            district_name: row.get("district_name")?,
            fg_city_id: row.get("fg_city_id")?,
        })
    }
}
impl SchemaTable for District {
    fn column_names() -> Vec<&'static str> {
        vec!["district_id", "district_name", "fg_city_id"]
    }

    fn table_name() -> &'static str {
        "districts"
    }

    fn values(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.district_id.clone()),
            Box::new(self.district_name.clone()),
            Box::new(self.fg_city_id.clone()),
        ]
    }

    fn key_attrs() -> Vec<&'static str> {
        vec!["district_id"]
    }

    fn key_attr_values(&self) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(self.district_id.clone())]
    }
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
impl RowValue for Region {
    fn get(row: &Row) -> Result<Self, Error> {
        Ok(Region {
            region_id: row.get("region_id")?,
            region_name: row.get("region_name")?,
            fg_country_name: row.get("fg_country_name")?,
            population: row.get("population")?,
            area_m2: row.get("area_m2")?,
            climate: row.get("climate")?,
            fg_centre_city_id: row.get("fg_centre_city_id")?,
        })
    }
}
impl SchemaTable for Region {
    fn column_names() -> Vec<&'static str> {
        vec![
            "region_id",
            "region_name",
            "fg_country_name",
            "population",
            "area_m2",
            "climate",
            "fg_centre_city_id",
        ]
    }

    fn table_name() -> &'static str {
        "regions"
    }

    fn values(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.region_id.clone()),
            Box::new(self.region_name.clone()),
            Box::new(self.fg_country_name.clone()),
            Box::new(self.population.clone()),
            Box::new(self.area_m2.clone()),
            Box::new(self.climate.clone()),
            Box::new(self.fg_centre_city_id.clone()),
        ]
    }

    fn key_attrs() -> Vec<&'static str> {
        vec!["region_id"]
    }

    fn key_attr_values(&self) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(self.region_id.clone())]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, SmartDefault)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
    pub is_admin: String,
    #[default(_code = "chrono::Utc::now()")]
    pub last_appearance: chrono::DateTime<chrono::Utc>,
}
impl RowValue for UserInfo {
    fn get(row: &Row) -> Result<Self, Error> {
        Ok(UserInfo {
            username: row.get("username")?,
            password: row.get("password")?,
            is_admin: row.get("is_admin")?,
            last_appearance: row.get("last_appearance")?,
        })
    }
}
impl SchemaTable for UserInfo {
    fn column_names() -> Vec<&'static str> {
        vec!["username", "password", "is_admin", "last_appearance"]
    }

    fn table_name() -> &'static str {
        "users_info"
    }

    fn values(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.username.clone()),
            Box::new(self.password.clone()),
            Box::new(self.is_admin.clone()),
            Box::new(self.last_appearance.clone()),
        ]
    }

    fn key_attrs() -> Vec<&'static str> {
        vec!["username"]
    }

    fn key_attr_values(&self) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(self.username.clone())]
    }
}
