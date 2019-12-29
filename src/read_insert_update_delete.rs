use crate::auth::{Admin, User};
use crate::core::OldNew;
use crate::database_operations::{
    count_rows, delete_data, insert_data, load_data, update_data, SchemaTable,
};
use crate::database_oracle::OracleConnection;
use crate::error::Error;
use crate::schema::{City, Continent, Country, District, Region};
use r2d2_oracle::oracle::RowValue;
use rocket::Route;
use rocket_contrib::json::{Json, JsonValue};
use serde::Serialize;
use std::fmt::Debug;

lazy_static! {
    pub static ref CRUD_ROUTES: Vec<Route> = routes![
        read_data,
        continents_update,
        continents_insert,
        continents_delete,
        cities_update,
        cities_insert,
        cities_delete,
        countries_update,
        countries_insert,
        countries_delete,
        districts_update,
        districts_insert,
        districts_delete,
        regions_update,
        regions_insert,
        regions_delete,
    ];
}

enum Table {
    Continents,
    Cities,
    Countries,
    Districts,
    Regions,
}

impl Table {
    fn parse(name: String) -> Result<Self, Error> {
        Ok(match name.split(".").next().take() {
            Some("continents") => Self::Continents,
            Some("cities") => Self::Cities,
            Some("countries") => Self::Countries,
            Some("districts") => Self::Districts,
            Some("regions") => Self::Regions,
            _ => return Err(Error::TableDoesntExistError { table: name }),
        })
    }
}

#[get("/<table_name>/items?<page_index>&<page_size>")]
fn read_data(
    conn: OracleConnection,
    table_name: String,
    mut page_index: usize,
    page_size: usize,
    user: User,
) -> Result<JsonValue, Error> {
    fn load_data_and_count_to_json<T>(
        connection: &OracleConnection,
        lower: usize,
        higher: usize,
    ) -> Result<JsonValue, Error>
    where
        T: SchemaTable + RowValue + Debug + Serialize,
    {
        let conn = &**connection;
        let rows = count_rows::<T>(conn)?;
        let data = load_data::<T>(conn, lower, higher)?;
        Ok(json!({ "itemsCount" : rows, "data" : data}))
    }
    page_index -= 1;
    let record_lower = page_index * page_size + 1;
    let record_higher = page_size * (page_index + 1);
    match Table::parse(table_name)? {
        Table::Continents => {
            load_data_and_count_to_json::<Continent>(&conn, record_lower, record_higher)
        }
        Table::Cities => load_data_and_count_to_json::<City>(&conn, record_lower, record_higher),
        Table::Countries => {
            load_data_and_count_to_json::<Country>(&conn, record_lower, record_higher)
        }
        Table::Districts => {
            load_data_and_count_to_json::<District>(&conn, record_lower, record_higher)
        }
        Table::Regions => load_data_and_count_to_json::<Region>(&conn, record_lower, record_higher),
    }
}

#[post("/continents.tera/items", format = "json", data = "<item>")]
fn continents_insert(
    conn: OracleConnection,
    item: Json<Continent>,
    user: User,
) -> Result<Json<Continent>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/cities.tera/items", format = "json", data = "<item>")]
fn cities_insert(
    conn: OracleConnection,
    item: Json<City>,
    user: User,
) -> Result<Json<City>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/countries.tera/items", format = "json", data = "<item>")]
fn countries_insert(
    conn: OracleConnection,
    item: Json<Country>,
    user: User,
) -> Result<Json<Country>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/districts.tera/items", format = "json", data = "<item>")]
fn districts_insert(
    conn: OracleConnection,
    item: Json<District>,
    user: User,
) -> Result<Json<District>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/regions.tera/items", format = "json", data = "<item>")]
fn regions_insert(
    conn: OracleConnection,
    item: Json<Region>,
    user: User,
) -> Result<Json<Region>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[put("/continents.tera/items", format = "json", data = "<item>")]
fn continents_update(
    conn: OracleConnection,
    item: Json<OldNew<Continent>>,
    user: Admin,
) -> Result<Json<Continent>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/cities.tera/items", format = "json", data = "<item>")]
fn cities_update(
    conn: OracleConnection,
    item: Json<OldNew<City>>,
    user: Admin,
) -> Result<Json<City>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/countries.tera/items", format = "json", data = "<item>")]
fn countries_update(
    conn: OracleConnection,
    item: Json<OldNew<Country>>,
    user: Admin,
) -> Result<Json<Country>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/districts.tera/items", format = "json", data = "<item>")]
fn districts_update(
    conn: OracleConnection,
    item: Json<OldNew<District>>,
    user: Admin,
) -> Result<Json<District>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/regions.tera/items", format = "json", data = "<item>")]
fn regions_update(
    conn: OracleConnection,
    item: Json<OldNew<Region>>,
    user: Admin,
) -> Result<Json<Region>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[delete("/continents.tera/items", format = "json", data = "<item>")]
fn continents_delete(
    conn: OracleConnection,
    item: Json<Continent>,
    user: Admin,
) -> Result<Json<Continent>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/cities.tera/items", format = "json", data = "<item>")]
fn cities_delete(
    conn: OracleConnection,
    item: Json<City>,
    user: Admin,
) -> Result<Json<City>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/countries.tera/items", format = "json", data = "<item>")]
fn countries_delete(
    conn: OracleConnection,
    item: Json<Country>,
    user: Admin,
) -> Result<Json<Country>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/districts.tera/items", format = "json", data = "<item>")]
fn districts_delete(
    conn: OracleConnection,
    item: Json<District>,
    user: Admin,
) -> Result<Json<District>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/regions.tera/items", format = "json", data = "<item>")]
fn regions_delete(
    conn: OracleConnection,
    item: Json<Region>,
    user: Admin,
) -> Result<Json<Region>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}
