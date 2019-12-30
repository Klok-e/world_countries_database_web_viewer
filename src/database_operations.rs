use crate::database_oracle::DbConnection;
use crate::error::Error;
use crate::schema::UserInfo;
use itertools::Itertools;
use r2d2_oracle::oracle::{sql_type::ToSql, RowValue};
use std::fmt::Debug;

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
) -> Result<Vec<T>, Error>
where
    T: SchemaTable + RowValue + Debug,
{
    let conn = connection.oracle_connection();
    let sql = format!(
        "select {} from (select c.*,rownum r from (select * from {} order by {}) c) where r between :1 and :2",
        T::column_names().join(","),
        T::table_name(),
        T::key_attrs().join(",")
    );

    let conts = conn.query_as::<T>(&sql, &[&record_start, &record_end])?;
    Ok(conts.collect::<Result<Vec<_>, _>>()?)
}

pub fn insert_data<T>(connection: &DbConnection, table_entity: &T) -> Result<(), Error>
where
    T: SchemaTable,
{
    let conn = connection.oracle_connection();
    let sql = format!(
        "insert into {} values ({})",
        T::table_name(),
        T::column_names()
            .into_iter()
            .enumerate()
            .map(|(i, _)| format!(":{}", i + 1))
            .join(",")
    );

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

fn check_data_key_exists<T>(connection: &DbConnection, table_entity: &T) -> Result<bool, Error>
where
    T: SchemaTable + RowValue,
{
    let conn = connection.oracle_connection();
    let sql = format!(
        "select {} from {} where {}",
        T::key_attrs().join(","),
        T::table_name(),
        T::key_attrs()
            .into_iter()
            .enumerate()
            .map(|(i, key)| format!("{}=:{}", key, i + 1))
            .join(" and ")
    );
    let vals = table_entity.key_attr_values();
    let sql_params = vals
        .iter()
        .map(|i| i.as_ref())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let matches = conn.query_as::<T>(&sql, sql_params.as_ref())?;
    Ok(matches.count() > 0)
}

pub fn update_data<T>(
    connection: &DbConnection,
    table_entity_old: &T,
    table_entity_new: &T,
) -> Result<(), Error>
where
    T: SchemaTable + RowValue,
{
    // first check if an item with the old keys exists
    if !check_data_key_exists(connection, table_entity_old)? {
        // error if it doesn't
        return Err(Error::KeyDoesntExistError {
            table_name: T::table_name().to_owned(),
        });
    }

    let col_len = T::column_names().len();
    let conn = connection.oracle_connection();
    let sql = format!(
        "update {} set {} where {}",
        T::table_name(),
        T::column_names()
            .into_iter()
            .enumerate()
            .map(|(i, col_name)| format!("{}=:{}", col_name, i + 1))
            .join(","),
        T::key_attrs()
            .into_iter()
            .enumerate()
            .map(|(i, key_attr_name)| format!("{}=:{}", key_attr_name, col_len + i + 1))
            .join(" and ")
    );

    let new_vals = table_entity_new.values();
    let old_keys = table_entity_old.key_attr_values();
    let sql_params = new_vals
        .iter()
        .chain(old_keys.iter())
        .map(|i| i.as_ref())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    conn.execute(&sql, sql_params.as_ref())?;
    conn.commit()?;
    Ok(())
}

pub fn delete_data<T>(connection: &DbConnection, table_entity: &T) -> Result<(), Error>
where
    T: SchemaTable,
{
    let conn = connection.oracle_connection();
    let sql = format!(
        "delete from {} where {}",
        T::table_name(),
        T::key_attrs()
            .into_iter()
            .enumerate()
            .map(|(i, key_attr_name)| format!("{}=:{}", key_attr_name, i + 1))
            .join(" and ")
    );

    let vals = table_entity.key_attr_values();
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

pub fn get_user(connection: &DbConnection, user: &UserInfo) -> Result<Option<UserInfo>, Error> {
    let conn = connection.oracle_connection();
    let sql = dbg!(format!(
        "select {} from {} where {}",
        UserInfo::column_names().join(","),
        UserInfo::table_name(),
        UserInfo::key_attrs()
            .into_iter()
            .enumerate()
            .map(|(i, key_attr_name)| format!("{}=:{}", key_attr_name, i + 1))
            .join(" and ")
    ));
    let vals = user.key_attr_values();
    let sql_params = vals
        .iter()
        .map(|i| i.as_ref())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let matches = conn
        .query_as::<UserInfo>(&sql, sql_params.as_ref())?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(if matches.len() == 1 {
        Some(matches.into_iter().next().take().unwrap())
    } else {
        None
    })
}
