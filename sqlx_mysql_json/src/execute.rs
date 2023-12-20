use sqlx::{
    mysql::{MySqlQueryResult, MySqlRow},
    MySql, MySqlPool, Transaction,
};

use crate::parse::Query;

pub enum Parameter {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Bool(bool),
    Bytes(Vec<u8>),
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
        }
    }
    let result = q.execute(pool).await?;
    Ok(result)
}

pub async fn execute_in_transaction(
    tx: &mut Transaction<'_, MySql>,
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
        }
    }

    let result = q.execute(&mut **tx).await?;
    Ok(result)
}
