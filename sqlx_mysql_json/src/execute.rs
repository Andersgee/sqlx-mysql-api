use sqlx::{
    mysql::{MySqlQueryResult, MySqlRow},
    pool::PoolConnection,
    MySql, MySqlPool,
};

use crate::parse::Query;

#[derive(Debug)]
pub enum Parameter {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Bool(bool),
    Bytes(Vec<u8>),
    NULL,
}

pub async fn fetch_all(
    pool: &MySqlPool,
    query: &Query,
) -> Result<Vec<MySqlRow>, sqlx::error::Error> {
    let mut q = sqlx::query(&query.sql);
    for p in query.parameters.iter() {
        match p {
            Parameter::Int(x) => q = q.bind(x),
            Parameter::Uint(x) => q = q.bind(x),
            Parameter::Float(x) => q = q.bind(x),
            Parameter::Str(x) => q = q.bind(x),
            Parameter::Bool(x) => q = q.bind(x),
            Parameter::Bytes(x) => q = q.bind(x),
            Parameter::NULL => q = q.bind(None::<String>),
        }
    }
    let rows = q.fetch_all(pool).await?;
    Ok(rows)
}

pub async fn execute(
    pool: &MySqlPool,
    query: &Query,
) -> Result<MySqlQueryResult, sqlx::error::Error> {
    let mut q = sqlx::query(&query.sql);
    for p in query.parameters.iter() {
        match p {
            Parameter::Int(x) => q = q.bind(x),
            Parameter::Uint(x) => q = q.bind(x),
            Parameter::Float(x) => q = q.bind(x),
            Parameter::Str(x) => q = q.bind(x),
            Parameter::Bool(x) => q = q.bind(x),
            Parameter::Bytes(x) => q = q.bind(x),
            Parameter::NULL => q = q.bind(None::<String>),
        }
    }
    let result = q.execute(pool).await?;
    Ok(result)
}

pub async fn execute_in_connection(
    conn: &mut PoolConnection<MySql>,
    query: &Query,
) -> Result<MySqlQueryResult, sqlx::error::Error> {
    let mut q = sqlx::query(&query.sql);
    for p in query.parameters.iter() {
        match p {
            Parameter::Int(x) => q = q.bind(x),
            Parameter::Uint(x) => q = q.bind(x),
            Parameter::Float(x) => q = q.bind(x),
            Parameter::Str(x) => q = q.bind(x),
            Parameter::Bool(x) => q = q.bind(x),
            Parameter::Bytes(x) => q = q.bind(x),
            Parameter::NULL => q = q.bind(None::<String>),
        }
    }
    let result = q.execute(&mut **conn).await?;
    Ok(result)
}
