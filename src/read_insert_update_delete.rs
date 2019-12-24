use crate::core::OldNew;
use crate::database_operations::{count_rows, delete_data, insert_data, load_data, update_data};
use crate::database_oracle::OracleConnection;
use crate::error::Error;
use crate::schema::{City, Continent, Country, District, Region};
use rocket::Route;
use rocket_contrib::json::{Json, JsonValue};

lazy_static! {
    pub static ref CRUD_ROUTES: Vec<Route> = routes![
        continents_read,
        continents_update,
        continents_insert,
        continents_delete,
        cities_read,
        cities_update,
        cities_insert,
        cities_delete,
        countries_read,
        countries_update,
        countries_insert,
        countries_delete,
        districts_read,
        districts_update,
        districts_insert,
        districts_delete,
        regions_read,
        regions_update,
        regions_insert,
        regions_delete,
    ];
}

#[get("/continents.tera/items?<page_index>&<page_size>")]
fn continents_read(
    conn: OracleConnection,
    mut page_index: usize,
    page_size: usize,
) -> Result<JsonValue, Error> {
    page_index -= 1;
    let data = load_data::<Continent>(
        &*conn,
        page_index * page_size + 1,
        page_size * (page_index + 1),
    )?;
    let item_count = count_rows::<Continent>(&*conn)?;
    Ok(json!({ "itemsCount" : item_count, "data" : data}))
}

#[get("/cities.tera/items?<page_index>&<page_size>")]
fn cities_read(
    conn: OracleConnection,
    mut page_index: usize,
    page_size: usize,
) -> Result<JsonValue, Error> {
    page_index -= 1;
    let data = load_data::<City>(
        &*conn,
        page_index * page_size + 1,
        page_size * (page_index + 1),
    )?;
    let item_count = count_rows::<City>(&*conn)?;
    Ok(json!({ "itemsCount" : item_count, "data" : data}))
}

#[get("/countries.tera/items?<page_index>&<page_size>")]
fn countries_read(
    conn: OracleConnection,
    mut page_index: usize,
    page_size: usize,
) -> Result<JsonValue, Error> {
    page_index -= 1;
    let data = load_data::<Country>(
        &*conn,
        page_index * page_size + 1,
        page_size * (page_index + 1),
    )?;
    let item_count = count_rows::<Country>(&*conn)?;
    Ok(json!({ "itemsCount" : item_count, "data" : data}))
}

#[get("/districts.tera/items?<page_index>&<page_size>")]
fn districts_read(
    conn: OracleConnection,
    mut page_index: usize,
    page_size: usize,
) -> Result<JsonValue, Error> {
    page_index -= 1;
    let data = load_data::<District>(
        &*conn,
        page_index * page_size + 1,
        page_size * (page_index + 1),
    )?;
    let item_count = count_rows::<District>(&*conn)?;
    Ok(json!({ "itemsCount" : item_count, "data" : data}))
}

#[get("/regions.tera/items?<page_index>&<page_size>")]
fn regions_read(
    conn: OracleConnection,
    mut page_index: usize,
    page_size: usize,
) -> Result<JsonValue, Error> {
    page_index -= 1;
    let data = load_data::<Region>(
        &*conn,
        page_index * page_size + 1,
        page_size * (page_index + 1),
    )?;
    let item_count = count_rows::<Region>(&*conn)?;
    Ok(json!({ "itemsCount" : item_count, "data" : data}))
}

#[post("/continents.tera/items", format = "json", data = "<item>")]
fn continents_insert(
    conn: OracleConnection,
    item: Json<Continent>,
) -> Result<Json<Continent>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/cities.tera/items", format = "json", data = "<item>")]
fn cities_insert(conn: OracleConnection, item: Json<City>) -> Result<Json<City>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/countries.tera/items", format = "json", data = "<item>")]
fn countries_insert(conn: OracleConnection, item: Json<Country>) -> Result<Json<Country>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/districts.tera/items", format = "json", data = "<item>")]
fn districts_insert(conn: OracleConnection, item: Json<District>) -> Result<Json<District>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[post("/regions.tera/items", format = "json", data = "<item>")]
fn regions_insert(conn: OracleConnection, item: Json<Region>) -> Result<Json<Region>, Error> {
    insert_data(&*conn, &item.0)?;
    Ok(item)
}

#[put("/continents.tera/items", format = "json", data = "<item>")]
fn continents_update(
    conn: OracleConnection,
    item: Json<OldNew<Continent>>,
) -> Result<Json<Continent>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/cities.tera/items", format = "json", data = "<item>")]
fn cities_update(conn: OracleConnection, item: Json<OldNew<City>>) -> Result<Json<City>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/countries.tera/items", format = "json", data = "<item>")]
fn countries_update(
    conn: OracleConnection,
    item: Json<OldNew<Country>>,
) -> Result<Json<Country>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/districts.tera/items", format = "json", data = "<item>")]
fn districts_update(
    conn: OracleConnection,
    item: Json<OldNew<District>>,
) -> Result<Json<District>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[put("/regions.tera/items", format = "json", data = "<item>")]
fn regions_update(
    conn: OracleConnection,
    item: Json<OldNew<Region>>,
) -> Result<Json<Region>, Error> {
    update_data(&*conn, &(item.0).old, &(item.0).new)?;
    Ok(Json(item.into_inner().new))
}

#[delete("/continents.tera/items", format = "json", data = "<item>")]
fn continents_delete(
    conn: OracleConnection,
    item: Json<Continent>,
) -> Result<Json<Continent>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/cities.tera/items", format = "json", data = "<item>")]
fn cities_delete(conn: OracleConnection, item: Json<City>) -> Result<Json<City>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/countries.tera/items", format = "json", data = "<item>")]
fn countries_delete(conn: OracleConnection, item: Json<Country>) -> Result<Json<Country>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/districts.tera/items", format = "json", data = "<item>")]
fn districts_delete(conn: OracleConnection, item: Json<District>) -> Result<Json<District>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}

#[delete("/regions.tera/items", format = "json", data = "<item>")]
fn regions_delete(conn: OracleConnection, item: Json<Region>) -> Result<Json<Region>, Error> {
    delete_data(&*conn, &item.0)?;
    Ok(item)
}
