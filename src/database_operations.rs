use crate::database_oracle::DbConnection;
use crate::error::Error;
use crate::schema::Continent;
use itertools::Itertools;
use r2d2_oracle::oracle::ResultSet;
use r2d2_oracle::oracle::{
    sql_type::{FromSql, ToSql},
    Row, RowValue,
};

pub trait SchemaTable {
    fn column_names() -> Vec<&'static str>;
    fn table_name() -> &'static str;
    fn values(&self) -> Vec<Box<dyn ToSql>>;
    fn key_attrs() -> Vec<&'static str>;
    fn key_attr_values(&self) -> Vec<Box<dyn ToSql>>;
}

pub fn load_data<T>(
    connection: &DbConnection,
    record_start: usize,
    record_end: usize,
) -> Result<ResultSet<T>, Error>
where
    T: SchemaTable + RowValue,
{
    let conn = connection.oracle_connection();
    let sql = format!(
        "select {} from {} where rownum >= :1 and rownum <= :2",
        T::column_names().join(","),
        T::table_name()
    );

    let conts = conn.query_as::<T>(&sql, &[&record_start, &record_end])?;
    Ok(conts)
}

pub fn insert_data<T>(connection: &DbConnection, table_entity: T) -> Result<(), Error>
where
    T: SchemaTable,
{
    let conn = connection.oracle_connection();
    let sql = dbg!(format!(
        "insert into {} values ({})",
        T::table_name(),
        T::column_names()
            .into_iter()
            .enumerate()
            .map(|(i, _)| format!(":{}", i + 1))
            .join(",")
    ));

    let vals = table_entity.values();
    let sql_params = vals
        .iter()
        .map(|i| i.as_ref())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    conn.execute(&sql, sql_params.as_ref())?;
    conn.commit()?;
    Ok(())
}

pub fn update_data<T>(connection: &DbConnection, table_entity: T) -> Result<(), Error>
where
    T: SchemaTable,
{
    let conn = connection.oracle_connection();
    let sql = dbg!(format!(
        "update {} set {} where {}",
        T::table_name(),
        T::column_names()
            .into_iter()
            .enumerate()
            .map(|(i, col_name)| format!("{}=:{}", col_name, i))
            .join(","),
        T::key_attrs()
            .into_iter()
            .enumerate()
            .map(|(i, key_attr_name)| format!("{}=:{}", key_attr_name, i))
            .join(" and ")
    ));

    let vals = table_entity.values();
    let sql_params = vals
        .iter()
        .map(|i| i.as_ref())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    conn.execute(&sql, sql_params.as_ref())?;
    conn.commit()?;
    Ok(())
}

pub fn delete_data<T>(connection: &DbConnection, table_entity: T) -> Result<(), Error>
where
    T: SchemaTable,
{
    let conn = connection.oracle_connection();
    let sql = dbg!(format!(
        "delete from {} where {}",
        T::table_name(),
        T::key_attrs()
            .into_iter()
            .enumerate()
            .map(|(i, key_attr_name)| format!("{}=:{}", key_attr_name, i))
            .join(" and ")
    ));

    let vals = table_entity.values();
    let sql_params = vals
        .iter()
        .map(|i| i.as_ref())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    conn.execute(&sql, sql_params.as_ref())?;
    conn.commit()?;
    Ok(())
}

pub fn count_rows<T>(connection: &DbConnection) -> Result<usize, Error>
where
    T: SchemaTable,
{
    let conn = connection.oracle_connection();
    let sql = format!("select count(*) from {}", T::table_name());

    conn.query_as::<usize>(&sql, &[])?
        .collect::<Result<Vec<_>, _>>()?
        .pop()
        .ok_or_else(|| Error::TableEmptyError {
            table_name: "continents".to_owned(),
        })
}
