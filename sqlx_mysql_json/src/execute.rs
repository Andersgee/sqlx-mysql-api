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
    let a = pool.execute("hello").await;
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

/// unprepared querys dont have their query plan cached... also dont have parameters
///
/// see https://github.com/launchbadge/sqlx#querying
///
/// is "prepared query" the reason Im getting stale info in information_schema tables?
/// like if I drop a column, then the information_schema.COLUMNS would still have the column... sometimes.
/// I read somewhere in sqlx crate docs (cant find it right now) that the query plan is cached once
/// and the query plan is build AFTER (for next time) instead of BEFORE or some such
///
/// this kind of makes sense, it felt like it was always "one behind" fresh info
///
/// found it: https://docs.rs/sqlx/latest/sqlx/trait.Statement.html
/// anyway, try this...
pub async fn execute_in_transaction_unprepared(
    tx: &mut Transaction<'_, MySql>,
    query: &str,
) -> Result<MySqlQueryResult, sqlx::error::Error> {
    //let mut q = sqlx::query(&query.sql);
    //for p in query.parameters.iter() {
    //    match p {
    //        Parameter::Int(x) => q = q.bind(x),
    //        Parameter::Uint(x) => q = q.bind(x),
    //        Parameter::Float(x) => q = q.bind(x),
    //        Parameter::Str(x) => q = q.bind(x),
    //        Parameter::Bool(x) => q = q.bind(x),
    //        Parameter::Bytes(x) => q = q.bind(x),
    //    }
    //}
    let result = tx.execute(query).await?;

    //let result = q.execute(&mut **tx).await?;
    Ok(result)
}
