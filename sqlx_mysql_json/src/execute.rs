use sqlx::{
    mysql::{MySqlQueryResult, MySqlRow},
    Executor, MySql, MySqlPool, Transaction,
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

/// unprepared querys dont have their query plan cached and dont have parameters
/// https://github.com/launchbadge/sqlx#querying
/// also a note about statements here https://docs.rs/sqlx/latest/sqlx/trait.Statement.html
pub async fn execute_in_transaction_unprepared(
    tx: &mut Transaction<'_, MySql>,
    query: &str,
) -> Result<MySqlQueryResult, sqlx::error::Error> {
    let result = tx.execute(query).await?;
    Ok(result)
}
