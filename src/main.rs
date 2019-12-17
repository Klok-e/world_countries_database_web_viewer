#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod database;

use database::OracleConnection;

#[get("/")]
fn index(conn: OracleConnection) -> &'static str {
    "hello world"
}

fn main() {
    rocket::ignite()
        .attach(OracleConnection::fairing())
        .mount("/", routes![index])
        .launch();
}
