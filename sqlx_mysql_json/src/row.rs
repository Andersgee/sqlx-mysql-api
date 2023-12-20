use crate::{base64, error::Error};
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use sqlx::{
    mysql::{MySqlColumn, MySqlRow},
    Column, Decode, MySql, Row, TypeInfo, ValueRef,
};

pub fn row_to_value(row: &MySqlRow) -> Result<Value, Error> {
    let columns = row.columns();
    let mut map = Map::new();
    for col in columns {
        let key = col.name().to_string();
        let value = col_to_value(row, col)?;
        map = add_value_to_map(map, (key, value));
    }
    Ok(Value::Object(map))
}

pub fn rows_to_value(rows: &Vec<MySqlRow>) -> Result<Vec<Value>, Error> {
    rows.iter().map(|row| row_to_value(row)).collect()
}

fn add_value_to_map(
    mut map: Map<String, Value>,
    (key, value): (String, Value),
) -> Map<String, Value> {
    match map.entry(key) {
        serde_json::map::Entry::Vacant(vacant) => {
            vacant.insert(value);
        }
        serde_json::map::Entry::Occupied(mut old_entry) => {
            let mut new_array = if let Value::Array(v) = value {
                v
            } else {
                vec![value]
            };
            match old_entry.get_mut() {
                Value::Array(old_array) => old_array.append(&mut new_array),
                old_scalar => {
                    new_array.insert(0, old_scalar.take());
                    *old_scalar = Value::Array(new_array);
                }
            }
        }
    }
    map
}

/// convert MySqlColumn to serde_json::Value.
///
/// supports all possible types definable in a `schema.prisma` file for mysql
///
/// some types require special care to send over json and properly recieve in javascript:
/// - Binary and Blob types are base64 encoded and returned as `["Base64", "somestring"]`
/// - BIGINT type is returned as `["Bigint", "somestring"]`
/// - DATETIME and TIMESTAMP are returned as `["Date", "somestring"]`
///
/// ### note to self:
/// I deliberately avoided other types and aliases such as "NUMERIC" and "SERIAL" etc
/// to not fool myself into thinking this is feature complete with mysql itself.
/// its just feature complete with the subset of mysql types that a schema.prisma allows
///
fn col_to_value(row: &MySqlRow, col: &MySqlColumn) -> Result<Value, Error> {
    // inspired by https://github.com/lovasoa/SQLpage/blob/main/src/webserver/database/sql_to_json.rs
    let valueref_result = row.try_get_raw(col.ordinal());
    match valueref_result {
        Err(_) => Err(Error::Decode("could not get column value".to_string())),
        Ok(valueref) => {
            if valueref.is_null() {
                Ok(Value::Null)
            } else {
                let type_info = valueref.type_info();
                let type_name = type_info.name();

                match type_name {
                    "BOOLEAN" => {
                        //stored as tinyint
                        //let x = <u8 as Decode<MySql>>::decode(valueref).unwrap_or_default() == 1;
                        //serde_json::json!(x)
                        match <u8 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "TINYINT" => {
                        //let x = <i8 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <i8 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "TINYINT UNSIGNED" => {
                        //let x = <u8 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <u8 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "SMALLINT" => {
                        //let x = <i16 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <i16 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "SMALLINT UNSIGNED" => {
                        //let x = <u16 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <u16 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "INT" | "INTEGER" => {
                        //let x = <i32 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <i32 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "INT UNSIGNED" | "INTEGER UNSIGNED" => {
                        //let x = <u32 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <u32 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "MEDIUMINT" => {
                        //just decode this as i32.
                        //at database level its i24
                        //let x = <i32 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <i32 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "MEDIUMINT UNSIGNED" => {
                        //just decode this as u32.
                        //at database level its u24
                        //let x = <u32 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <u32 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "BIGINT" => {
                        //let x = <i64 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(["BigInt", x.to_string()])
                        match <i64 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "BIGINT UNSIGNED" => {
                        //let x = <u64 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(["BigInt", x.to_string()])
                        match <u64 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(["BigInt", x.to_string()])),
                        }
                    }
                    "FLOAT" => {
                        //let x = <f32 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <f32 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "DOUBLE" => {
                        //let x = <f64 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <f64 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "YEAR" => {
                        //just decode this as u16.
                        //at database level its some special single byte representation (not simply u8)
                        //let x = <u16 as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <u16 as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "DECIMAL" => {
                        //just decode this as string.
                        //at database level its some special byte represenation with 9 digits per 4 bytes
                        //we wouldnt want to cast this to f64 anyway, the whole point of decimal is that f64 etc is not enough
                        //let x = <String as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <String as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "TIME" => {
                        //let x = <chrono::NaiveTime>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x.to_string())
                        match <chrono::NaiveTime>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x.to_string())),
                        }
                    }
                    "DATE" => {
                        //let x = <chrono::NaiveDate>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x.to_string())
                        match <chrono::NaiveDate>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x.to_string())),
                        }
                    }
                    "DATETIME" | "TIMESTAMP" => {
                        //let x = <DateTime<Utc>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(["Date", x.to_string()])
                        match <DateTime<Utc>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(["Date", x.to_string()])),
                        }
                    }
                    "CHAR" | "VARCHAR" => {
                        //let x = <String as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <String as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "TINYTEXT" | "TEXT" | "MEDIUMTEXT" | "LONGTEXT" => {
                        //let x = <String as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        match <String as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(x)),
                        }
                    }
                    "BINARY" | "VARBINARY" => {
                        //let x = <Vec<u8> as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(["Base64", base64::vecu8_to_base64string(x)])
                        match <Vec<u8> as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!([
                                "Base64",
                                base64::vecu8_to_base64string(x)
                            ])),
                        }
                    }
                    "TINYBLOB" | "BLOB" | "MEDIUMBLOB" | "LONGBLOB" => {
                        //let x = <Vec<u8> as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(["Base64", base64::vecu8_to_base64string(x)])
                        match <Vec<u8> as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!([
                                "Base64",
                                base64::vecu8_to_base64string(x)
                            ])),
                        }
                    }
                    "JSON" => {
                        //let x = <String as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(["Json", x])
                        match <String as Decode<MySql>>::decode(valueref) {
                            Err(err) => Err(Error::Decode(err.to_string())),
                            Ok(x) => Ok(serde_json::json!(["Json", x])),
                        }
                    }
                    _ => {
                        //println!("default parsing database type '{:?}' as string", type_name);
                        //let x = <String as Decode<MySql>>::decode(valueref).unwrap_or_default();
                        //serde_json::json!(x)
                        Err(Error::Decode(format!("unsupported type {:?}", type_name)))
                    }
                }
            }
        }
    }
}
