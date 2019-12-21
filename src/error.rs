#[derive(Debug)]
pub enum Error {
    OracleError(r2d2_oracle::oracle::Error),
    TableEmptyError { table_name: String },
    KeyAlreadyExistsError { table_name: String },
}

impl From<r2d2_oracle::oracle::Error> for Error {
    fn from(error: r2d2_oracle::oracle::Error) -> Self {
        Error::OracleError(error)
    }
}
