#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod database_operations;
mod database_oracle;
mod schema;

use database_operations::load_data;
use database_oracle::OracleConnection;
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
fn continents_read(conn: OracleConnection, page_index: usize, page_size: usize) -> JsonValue {
    dbg!(page_index);
    dbg!(page_size);
    dbg!(load_data(&*conn, 0, 5));
    json!({ "status" : "ok"})
}

#[post("/continents.tera/items", data = "<continent>")]
fn continents_insert(conn: OracleConnection, continent: Json<schema::Continent>) -> JsonValue {
    println!("insert");
    json!({ "status" : "ok"})
}

#[put("/continents.tera/items", data = "<continent>")]
fn continents_update(conn: OracleConnection, continent: Json<schema::Continent>) -> JsonValue {
    println!("update");
    json!({ "status" : "ok"})
}

#[delete("/continents.tera", data = "<continent>")]
fn continents_delete(conn: OracleConnection, continent: Json<schema::Continent>) -> JsonValue {
    println!("delete");
    json!({ "status" : "ok"})
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
        .launch();
}
