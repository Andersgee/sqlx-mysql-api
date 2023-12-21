use error::Error;
use sqlx::{MySql, MySqlPool, Transaction};

mod base64;
pub mod error;
mod execute;
mod parse;
pub mod row;

/// calls `fetch_all()` for SELECT queries, otherwise calls `execute()`
///
/// returns a json object like this:
///```ts
///{
/// /** This is defined for insert, update and delete queries and contains the number of rows the query inserted/updated/deleted. */
/// numAffectedRows?: ["BigInt","number"],
/// /** This is defined for update queries and contains the number of rows the query changed. */
/// numChangedRows?: ["BigInt", "number"],
/// /** This is defined for insert queries */
/// insertId?: ["BigInt","number",
/// /** The rows returned by the query. This is always defined and is empty if the query returned no rows. */
/// rows: [];
///}
/// ```
pub async fn query(pool: &MySqlPool, s: &String) -> Result<serde_json::Value, Error> {
    let query = parse::string_to_query(s)?;
    match is_select_query(&query.sql) {
        true => fetch_all(pool, s).await,
        false => execute(pool, s).await,
    }
}

pub async fn fetch_all(pool: &MySqlPool, s: &String) -> Result<serde_json::Value, Error> {
    let query = parse::string_to_query(s)?;
    match execute::fetch_all(pool, &query).await {
        Err(err) => Err(Error::Sqlx(err.to_string())),
        Ok(rows) => {
            let value = serde_json::json!({
                "rows": row::rows_to_value(&rows)?
            });
            Ok(value)
        }
    }
}

pub async fn execute(pool: &MySqlPool, s: &String) -> Result<serde_json::Value, Error> {
    let query = parse::string_to_query(s)?;
    match execute::execute(pool, &query).await {
        Err(err) => Err(Error::Sqlx(err.to_string())),
        Ok(result) => {
            let num_affected_rows = result.rows_affected().to_string();
            let insert_id = result.last_insert_id().to_string();

            let value = serde_json::json!({
                "numAffectedRows": ["BigInt", num_affected_rows],
                "numChangedRows": ["BigInt",num_affected_rows],
                "insertId": ["BigInt", insert_id],
                "rows": []
            });

            Ok(value)
        }
    }
}

pub async fn execute_in_transaction(
    tx: &mut Transaction<'_, MySql>,
    s: &String,
) -> Result<serde_json::Value, Error> {
    let query = parse::string_to_query(s)?;
    match execute::execute_in_transaction(tx, &query).await {
        Err(err) => Err(Error::Sqlx(err.to_string())),
        Ok(result) => {
            let num_affected_rows = result.rows_affected().to_string();
            let insert_id = result.last_insert_id().to_string();

            let value = serde_json::json!({
                "numAffectedRows": ["BigInt", num_affected_rows],
                "numChangedRows": ["BigInt",num_affected_rows],
                "insertId": ["BigInt", insert_id],
                "rows": []
            });

            Ok(value)
        }
    }
}

fn is_select_query(sql: &String) -> bool {
    let first_word = sql.split_whitespace().next();
    match first_word {
        None => false,
        Some(s) => s.to_lowercase().starts_with("select"),
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
