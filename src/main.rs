#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate smart_default;

mod auth;
mod core;
mod database_operations;
mod database_oracle;
mod error;
mod read_insert_update_delete;
mod schema;

use crate::auth::{User, UserFullData};
use crate::database_operations::{get_user, update_data};
use crate::database_oracle::OracleConnection;
use crate::error::Error;
use crate::read_insert_update_delete::CRUD_ROUTES;
use crate::schema::UserInfo;
use chrono::{self, Utc};
use log::info;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::{Request, Response};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Value;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::path::PathBuf;

fn create_context(templ_name: &str, is_admin: bool) -> HashMap<&str, Value> {
    let mut map = HashMap::new();
    map.insert("current_tname", Value::from(templ_name));
    map.insert("is_admin", Value::from(is_admin));
    map
}

#[get("/")]
fn index(user: User) -> Template {
    Template::render("home", create_context("home", user.is_admin))
}

#[get("/continents.tera")]
fn continents(user: User) -> Template {
    Template::render("continents", create_context("continents", user.is_admin))
}

#[get("/cities.tera")]
fn cities(user: User) -> Template {
    Template::render("cities", create_context("cities", user.is_admin))
}

#[get("/countries.tera")]
fn countries(user: User) -> Template {
    Template::render("countries", create_context("countries", user.is_admin))
}

#[get("/districts.tera")]
fn districts(user: User) -> Template {
    Template::render("districts", create_context("districts", user.is_admin))
}

#[get("/regions.tera")]
fn regions(user: User) -> Template {
    Template::render("regions", create_context("regions", user.is_admin))
}

#[get("/login.tera")]
fn login() -> Template {
    Template::render("login", create_context("login", false))
}

#[post("/login.tera", data = "<user>")]
fn auth_user(
    connection: OracleConnection,
    mut cookies: Cookies,
    user: Form<UserFullData>,
) -> Result<Redirect, Error> {
    dbg!(&user);
    info!(
        "{}",
        format!("Login attempt with username {:?}", &user.0.username)
    );
    let pass = user.password.clone();
    let user = get_user(
        &*connection,
        &UserInfo {
            username: user.username.clone(),
            ..UserInfo::default()
        },
    )?;
    if let Some(u) = user {
        if pass == u.password {
            cookies.add_private(Cookie::new("user_name", u.username.clone()));
            update_data(
                &*connection,
                &u,
                &UserInfo {
                    last_appearance: Utc::now(),
                    ..u.clone()
                },
            )?;
            info!(
                "{}",
                format!("Login attempt with username {:?} successful", &u.username)
            );
        }
    };
    Ok(Redirect::to("/"))
}

#[get("/signout.tera")]
fn signout_user(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_name"));
    Redirect::to("/")
}

#[catch(401)]
fn unauthorized(req: &Request) -> Redirect {
    Redirect::to("/login.tera")
}

fn main() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()
        .unwrap();

    dbg!(std::env::var("LD_LIBRARY_PATH"));

    let mut root_routes = routes![
        index,
        continents,
        cities,
        countries,
        districts,
        regions,
        login,
        auth_user,
        signout_user
    ];
    root_routes.extend(CRUD_ROUTES.clone());
    rocket::ignite()
        .attach(OracleConnection::fairing())
        .attach(Template::fairing())
        .mount("/", root_routes)
        .mount("/images", StaticFiles::from("./images"))
        .mount("/adminlte", StaticFiles::from("./adminlte"))
        .mount("/js", StaticFiles::from("./js"))
        .register(catchers![unauthorized])
        .launch();
    Ok(())
}
