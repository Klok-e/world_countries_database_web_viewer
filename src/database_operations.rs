use crate::database_oracle::DbConnection;
use crate::error::Error;
use crate::schema::Continent;
use r2d2_oracle::oracle::ResultSet;

pub fn load_data(
    connection: &DbConnection,
    record_start: usize,
    record_end: usize,
) -> Result<ResultSet<Continent>, Error> {
    let conn = connection.oracle_connection();
    let query = "select name, area_m2 from continents where rownum >= :1 and rownum <= :2";

    conn.query_as::<Continent>(query, &[&record_start, &record_end])
        .map_err(|e| e.into())
}

pub fn count_rows(connection: &DbConnection) -> Result<usize, Error> {
    let conn = connection.oracle_connection();
    let query = "select count(*) from continents";

    conn.query_as::<usize>(query, &[])?
        .collect::<Result<Vec<_>, _>>()?
        .pop()
        .ok_or_else(|| Error::TableEmptyError {
            table_name: "continents".to_owned(),
        })
}
