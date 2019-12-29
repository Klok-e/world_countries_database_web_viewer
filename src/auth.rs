use rocket::http::Status;
use rocket::request::Form;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

#[derive(FromForm, Debug)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

pub struct User {
    pub is_admin: bool,
    pub data: UserData,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request.cookies().get_private("user_id");

        Outcome::Failure((Status::Unauthorized, ()))
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
