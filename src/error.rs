use r2d2_oracle::oracle;
use rocket::{
    http::{ContentType, Status},
    response,
    response::Responder,
    Request, Response,
};

#[derive(Debug)]
pub enum Error {
    OracleError(r2d2_oracle::oracle::Error),
    TableEmptyError { table_name: String },
    KeyDoesntExistError { table_name: String },
    TableDoesntExistError { table: String },
}

impl From<r2d2_oracle::oracle::Error> for Error {
    fn from(error: r2d2_oracle::oracle::Error) -> Self {
        Error::OracleError(error)
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            Error::OracleError(or_error) => {
                if let oracle::Error::OciError(_) = or_error {
                    Response::build_from(
                        json!({ "error_msg": format!("{}", or_error) }).respond_to(req)?,
                    )
                    .status(Status::new(278, "A constraint was violated"))
                    .header(ContentType::JSON)
                    .ok()
                } else {
                    Err(Status::new(500, "oracle"))
                }
            }
            Error::TableEmptyError { .. } => Err(Status::new(500, "oracle")),
            Error::KeyDoesntExistError { .. } => Err(Status::new(
                400,
                "Entity with the specified key doesn't exist",
            )),
            Error::TableDoesntExistError { .. } => {
                Err(Status::new(400, "Specified table doesn't exist"))
            }
        }
    }
}
