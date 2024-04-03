use actix_web::{
    http::header::{HeaderMap, HeaderName, HeaderValue},
    web,
};
use sqlx::MySqlPool;

pub struct Pools {
    pub db: MySqlPool,
    pub musker: MySqlPool,
    pub svgbattle: MySqlPool,
}

pub fn select_pool_by_header(headermap: &HeaderMap, pools: &web::Data<Pools>) -> Option<MySqlPool> {
    let custom_header: &'static str = "db";
    let b = HeaderName::from_static(custom_header);
    //default to "db" if no "db" header
    let d = HeaderValue::from_str("db").unwrap();

    let db = headermap.get(b).unwrap_or(&d).to_str().unwrap();
    match db {
        "db" => Some(pools.db.clone()),
        "musker" => Some(pools.musker.clone()),
        "svgbattle" => Some(pools.svgbattle.clone()),
        _ => None,
    }
}
