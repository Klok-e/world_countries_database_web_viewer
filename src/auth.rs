use crate::database_operations::{get_user, update_data};
use crate::database_oracle::{DbConnection, OracleConnection};
use crate::error::Error;
use crate::schema::UserInfo;
use chrono::Utc;
use rocket::http::{Cookie, Status};
use rocket::request::Form;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

#[derive(FromForm, Debug, Default)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

#[derive(Default)]
pub struct User {
    pub is_admin: bool,
    pub data: UserData,
}
impl User {
    fn new(username: String, password: String, is_admin: bool) -> Self {
        User {
            is_admin,
            data: UserData { username, password },
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cook = request.cookies().get_private("user_name");
        let cook = match cook {
            Some(c) => c,
            None => return Outcome::Failure((Status::Unauthorized, ())),
        };

        let connection = request.guard::<OracleConnection>();
        let connection = match connection {
            Outcome::Success(s) => s,
            Outcome::Failure(f) => return Outcome::Failure(f),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };
        Outcome::Success(
            match get_user(
                &*connection,
                &UserInfo {
                    username: cook.value().to_owned(),
                    ..UserInfo::default()
                },
            ) {
                Ok(Some(UserInfo {
                    username,
                    password,
                    is_admin,
                    last_appearance,
                })) => {
                    let u = UserInfo {
                        username: username.clone(),
                        password: password.clone(),
                        is_admin: is_admin.clone(),
                        last_appearance,
                    };
                    // check if the user needs to relogin
                    if last_appearance < Utc::now() - chrono::Duration::minutes(5) {
                        println!("relogin! {}", username);
                        request.cookies().remove_private(Cookie::named("user_name"));
                        return Outcome::Failure((Status::Unauthorized, ()));
                    }
                    // update last seen timer
                    request
                        .cookies()
                        .add_private(Cookie::new("user_name", username.clone()));
                    match update_data(
                        &*connection,
                        &u,
                        &UserInfo {
                            last_appearance: Utc::now(),
                            ..u.clone()
                        },
                    ) {
                        Ok(_) => (),
                        Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
                    };
                    User::new(username, password, is_admin == "y")
                }
                Ok(None) => return Outcome::Failure((Status::Unauthorized, ())),
                Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
            },
        )
    }
}

pub struct Admin {
    pub data: UserData,
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = request.guard::<User>()?;

        if user.is_admin {
            Outcome::Success(Admin {
                data: UserData {
                    username: user.data.username,
                    password: user.data.password,
                },
            })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
