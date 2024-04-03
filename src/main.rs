use actix_web::{dev::ServiceRequest, web, App, HttpServer};
mod error;
mod routes;

use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, sync::OnceLock};

//in javascript:
//str = Buffer.from(`${userID}:${password}`).toString("base64")
//headers: {Authorization: `Basic ${str}`},

async fn validate_credentials(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    match credentials.password() {
        None => Err((error::Error::AuthError.into(), req)),
        Some(pw) => {
            if pw == auth_password() {
                Ok(req)
            } else {
                Err((error::Error::AuthError.into(), req))
            }
        }
    }
}

fn auth_password() -> &'static str {
    static PASSWORD: OnceLock<String> = OnceLock::new();
    PASSWORD.get_or_init(|| env::var("DB_HTTP_AUTH_PASSWORD").unwrap())
}

pub struct Pools {
    db: MySqlPool,
    musker: MySqlPool,
    svgbattle: MySqlPool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("starting http api (tag 0.3)");
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("expected DATABASE_URL in env");
    let database_url_musker =
        env::var("DATABASE_URL_MUSKER").expect("expected DATABASE_URL_MUSKER in env");
    let database_url_svgbattle =
        env::var("DATABASE_URL_MUSKER").expect("expected DATABASE_URL_MUSKER in env");
    let addrs = env::var("DB_HTTP_LISTEN_ADRESS").expect("expected DB_HTTP_LISTEN_ADRESS in env");
    let _x = env::var("DB_HTTP_AUTH_PASSWORD").expect("expected DB_HTTP_AUTH_PASSWORD in env");

    println!("connecting to db and creating pool...");
    //let pool = web::Data::new(MySqlPoolOptions::new().max_connections(10).connect(&database_url).await.unwrap());

    let pools = web::Data::new(Pools {
        db: MySqlPoolOptions::new()
            .max_connections(2) //default is 10
            .connect(&database_url)
            .await
            .unwrap(),
        musker: MySqlPoolOptions::new()
            .max_connections(2)
            .connect(&database_url_musker)
            .await
            .unwrap(),
        svgbattle: MySqlPoolOptions::new()
            .max_connections(2)
            .connect(&database_url_svgbattle)
            .await
            .unwrap(),
    });
    println!("...pool created");
    println!("http api listening on '{:?}'", addrs);
    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(validate_credentials);
        App::new()
            .app_data(pools.clone())
            .wrap(auth)
            .service(routes::root)
            .service(routes::transaction)
    })
    .bind(addrs)?
    .run()
    .await
}
