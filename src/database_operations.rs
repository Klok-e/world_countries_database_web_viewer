use crate::database_oracle::DbConnection;
use r2d2_oracle::oracle::Connection;

pub fn load_data(connection: &DbConnection, record_start: usize, record_end: usize) {
    let conn = connection.oracle_connection();
    let query = "select name, area_m2 from continents";

    //conn.query(query);
}
