use serde::Deserialize;
use serde_json::Value;

use crate::{base64::base64string_to_vecu8, error::Error, execute::Parameter};
use crate::{wkb::geom_to_wkb};

#[derive(Deserialize, Debug)]
pub struct JsonQuery {
    sql: String,
    parameters: Vec<Value>,
}

pub struct Query {
    pub sql: String,
    pub parameters: Vec<Parameter>,
}

pub fn string_to_query(string: &String) -> Result<Query, Error> {
    match serde_json::from_str::<JsonQuery>(string) {
        Err(err) => Err(Error::SerdeJson(err.to_string())),
        Ok(query) => {
            match query
                .parameters
                .into_iter()
                .map(|a| value_to_parameter(a))
                .collect::<Result<Vec<Parameter>, Error>>()
            {
                Err(err) => Err(err),
                Ok(parameters) => Ok(Query {
                    sql: query.sql,
                    parameters,
                }),
            }
        }
    }
}

pub fn value_to_parameter(value: Value) -> Result<Parameter, Error> {
    
    match value {
        Value::Null => {
            Err(Error::Parameter("parameter value should not be null. put 'IS NULL or 'IS NOT NULL' in sql rather than parameter.".to_string()))
        }
        Value::Object(obj) => {
            //Err(Error::Parameter("parameter value should not be object".to_string()))
            
            //Ok(Parameter::Str(geojsonvalue.to_string()))

            //so actually, lets just write the WKB instead
            //but with [230, 16, 0, 0] as first 4 bytes aka SRS_ID 4326 which it seems to write by default when using from geojson stuff
            //
            match <geojson::Value>::try_from(obj) {
                Err(_) => Err(Error::Parameter("jsonvalue is not geojson".to_string())),
                Ok(geojsonvalue) => {

                    match <geo_types::Geometry>::try_from(geojsonvalue) {
                        Err(_) => Err(Error::Parameter("geojsonvalue is not geometry".to_string())),
                        Ok(geom) => {
                            match geom_to_wkb(&geom) {
                                Err(_) => Err(Error::Parameter("geometry is not wkb".to_string())),
                                Ok(bytes) => {
                                    
                                    //mysql just uses the wkb as internal format but with the first 4 bytes being SRS_ID
                                    //[230, 16, 0, 0] aka SRS_ID=4326 representing SRS_NAME="WGS 84" is what it wrote by default when using ST_GeomFromGeoJSON() sql function
                                    //so lets do the same

                                    let mut mysql_spatial_type_bytes:Vec<u8> = Vec::with_capacity(bytes.len() + 4);
                                    mysql_spatial_type_bytes.push(230);
                                    mysql_spatial_type_bytes.push(16);
                                    mysql_spatial_type_bytes.push(0);
                                    mysql_spatial_type_bytes.push(0);
                                    for b in bytes {
                                        mysql_spatial_type_bytes.push(b);
                                    }

                                    //let mut mysql_spatial_type_bytes:Vec<u8> = vec![230, 16, 0, 0];
                                    //mysql_spatial_type_bytes.append(&mut bytes);

                                    Ok(Parameter::Bytes(mysql_spatial_type_bytes))
                                }
                            }
                        },
                    }
                                        
                },
            }
        }
        Value::Bool(x) => {
            //better to use  "IS TRUE" or "IS FALSE" in sql rather than parameter.
            //but this is fine as long as you dont store a 2,3 or 4 etc in BOOLEAN column which is technically possible since db representation is TINYINT
            match x {
                true => Ok(Parameter::Bool(true)),
                false => Ok(Parameter::Bool(false)),
            }
        }
        Value::Number(x) => {
            if x.is_f64() {
                Ok(Parameter::Float(x.as_f64().unwrap()))
            } else if x.is_i64() {
                Ok(Parameter::Int(x.as_i64().unwrap()))
            } else if x.is_u64() {
                Ok(Parameter::Uint(x.as_u64().unwrap()))
            } else {
                Err(Error::Parameter("parameter value number is not f64, i64 or u64".to_string()))
            }
        }
        Value::String(x) => Ok(Parameter::Str(x)),
        Value::Array(v) => {
            match tuple_type(v) {
                Err(err) => Err(err),
                Ok(variant) => match variant {
                    TupleType::Date(x) => Ok(Parameter::Str(x)),
                    TupleType::BigInt(x) => Ok(Parameter::Int(x)),
                    TupleType::Bytes(x) => Ok(Parameter::Bytes(x)),
                    //TupleType::Decimal(x) => Some(Parameter::Str(x)),
                },
            }
        }
    }
}

enum TupleType {
    Date(String),
    BigInt(i64),
    Bytes(Vec<u8>),
    //Decimal(String),
}

fn tuple_type(v: Vec<serde_json::Value>) -> Result<TupleType, Error> {
    if v.len() != 2 {
        return Err(default_tuple_type_error());
    }

    let a = v[0].as_str();
    let b = v[1].as_str();
    if a.is_none() || b.is_none() {
        return Err(default_tuple_type_error());
    }

    let a = a.unwrap();
    let b = b.unwrap();

    if a == "Date" {
        let s = mysql_date_string(b);
        match s {
            None => Err(Error::TupleType("invalid Date string parsing".to_string())),
            Some(datestring) => Ok(TupleType::Date(datestring)),
        }
    } else if a == "BigInt" {
        let x = b.parse::<i64>();
        match x {
            Err(_) => Err(Error::TupleType("invalid i64 parsing".to_string())),
            Ok(val) => Ok(TupleType::BigInt(val)),
        }
    } else if a == "Base64" {
        let x = base64string_to_vecu8(b.to_string());
        match x {
            Err(_) => Err(Error::TupleType("invalid Base64 parsing".to_string())),
            Ok(val) => Ok(TupleType::Bytes(val)),
        }
    } else {
        Err(default_tuple_type_error())
    }
}

fn default_tuple_type_error() -> Error {
    Error::TupleType("parameter value should not be array unless one of [\"Date\",\"str\"], [\"BigInt\",\"str\"], [\"Base64\",\"str\"]".to_string())
}

/// "2023-12-12T19:49:38.415Z" => "2023-12-12 19:49:38.415"
pub fn mysql_date_string(str: &str) -> Option<String> {
    let s = str.replace("T", " ").replace("Z", "");
    match s.len() {
        23 => Some(s.to_string()),
        _ => None,
    }
}
