#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate lazy_static;

mod database_operations;
mod database_oracle;
mod error;
mod read_insert_update_delete;
mod schema;

use crate::database_oracle::OracleConnection;
use crate::read_insert_update_delete::CRUD_ROUTES;
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
fn index() -> Template {
    Template::render("home", create_context("home"))
}

#[get("/continents.tera")]
fn continents() -> Template {
    Template::render("continents", create_context("continents"))
}

#[get("/cities.tera")]
fn cities() -> Template {
    Template::render("cities", create_context("cities"))
}

#[get("/countries.tera")]
fn countries() -> Template {
    Template::render("countries", create_context("countries"))
}

#[get("/districts.tera")]
fn districts() -> Template {
    Template::render("districts", create_context("districts"))
}

#[get("/regions.tera")]
fn regions() -> Template {
    Template::render("regions", create_context("regions"))
}

fn main() {
    let mut root_routes = routes![index, continents, cities, countries, districts, regions];
    root_routes.extend(CRUD_ROUTES.clone());
    rocket::ignite()
        .attach(OracleConnection::fairing())
        .attach(Template::fairing())
        .mount("/", root_routes)
        .mount("/images", StaticFiles::from("./images"))
        .mount("/adminlte", StaticFiles::from("./adminlte"))
        .mount("/js", StaticFiles::from("./js"))
        .launch();
}
