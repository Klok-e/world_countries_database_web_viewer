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
