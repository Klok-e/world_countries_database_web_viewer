#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod database_operations;
mod database_oracle;
mod error;
mod schema;

use crate::database_operations::{count_rows, delete_data, insert_data, load_data, update_data};
use crate::database_oracle::OracleConnection;
use crate::error::Error;
use crate::schema::Continent;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Value;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

fn create_context(templ_name: &str) -> HashMap<&str, Value> {
    let mut map = HashMap::new();
    map.insert("current_tname", Value::from(templ_name));
    map
}

#[get("/")]
fn index(conn: OracleConnection) -> Template {
    Template::render("home", create_context("home"))
}

#[get("/continents.tera")]
fn continents(conn: OracleConnection) -> Template {
    Template::render("continents", create_context("continents"))
}

#[get("/continents.tera/items?<page_index>&<page_size>")]
fn continents_read(
    conn: OracleConnection,
    mut page_index: usize,
    page_size: usize,
) -> Result<JsonValue, Error> {
    page_index -= 1;
    let data =
        load_data::<Continent>(&*conn, page_index * page_size, page_size * (page_index + 1))?
            .collect::<Result<Vec<_>, _>>()?;
    let item_count = count_rows::<Continent>(&*conn)?;
    Ok(json!({ "itemsCount" : item_count, "data" : data}))
}

#[post("/continents.tera/items", format = "json", data = "<continent>")]
fn continents_insert(
    conn: OracleConnection,
    continent: Json<schema::Continent>,
) -> Result<Json<schema::Continent>, Error> {
    insert_data(&*conn, continent.0.clone())?;
    Ok(continent)
}

#[put("/continents.tera/items", format = "json", data = "<continent>")]
fn continents_update(
    conn: OracleConnection,
    continent: Json<schema::Continent>,
) -> Result<Json<schema::Continent>, Error> {
    update_data(&*conn, continent.0.clone())?;
    Ok(continent)
}

#[delete("/continents.tera/items", format = "json", data = "<continent>")]
fn continents_delete(
    conn: OracleConnection,
    continent: Json<schema::Continent>,
) -> Result<Json<schema::Continent>, Error> {
    delete_data(&*conn, continent.0.clone())?;
    Ok(continent)
}

#[get("/cities.tera")]
fn cities(conn: OracleConnection) -> Template {
    Template::render("cities", create_context("cities"))
}

#[get("/countries.tera")]
fn countries(conn: OracleConnection) -> Template {
    Template::render("countries", create_context("countries"))
}

#[get("/districts.tera")]
fn districts(conn: OracleConnection) -> Template {
    Template::render("districts", create_context("districts"))
}

#[get("/regions.tera")]
fn regions(conn: OracleConnection) -> Template {
    Template::render("regions", create_context("regions"))
}

fn main() {
    rocket::ignite()
        .attach(OracleConnection::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                continents,
                continents_read,
                continents_update,
                continents_insert,
                continents_delete,
                cities,
                countries,
                districts,
                regions
            ],
        )
        .mount("/images", StaticFiles::from("./images"))
        .mount("/adminlte", StaticFiles::from("./adminlte"))
        .mount("/js", StaticFiles::from("./js"))
        .launch();
}
