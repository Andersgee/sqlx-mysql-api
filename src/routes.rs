use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Executor, MySqlPool};

#[derive(Deserialize)]
struct Q {
    q: String,
}

#[get("/")]
async fn root(pool: web::Data<MySqlPool>, query: web::Query<Q>) -> impl Responder {
    //general purpose "query via http"
    let result = sqlx_mysql_json::query(&pool, &query.q).await;
    match result {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[post("/transaction")]
pub async fn transaction(
    pool: web::Data<MySqlPool>,
    queries: web::Json<Vec<String>>,
) -> impl Responder {
    let mut results: Vec<serde_json::Value> = vec![];

    match pool.acquire().await {
        Err(_) => HttpResponse::InternalServerError()
            .json("couldnt acquire connection from pool".to_string()),
        Ok(mut conn) => {
            //let _ = conn.execute("SET autocommit=1").await; //default.. except when inside a START TRANSACTION?...
            //let _ = conn.execute("SET autocommit=0").await;
            //
            match conn.execute("START TRANSACTION").await {
                Err(_) => HttpResponse::InternalServerError().json("failed to START TRANSACTION"),
                Ok(_) => {
                    let mut should_rollback = false;
                    for q in queries.into_inner() {
                        match sqlx_mysql_json::execute_in_connection(&mut conn, &q).await {
                            Ok(result) => {
                                results.push(result);
                            }
                            Err(_) => {
                                should_rollback = true;
                                break;
                            }
                        }
                    }

                    match should_rollback {
                        true => match conn.execute("ROLLBACK").await {
                            Ok(_) => HttpResponse::BadRequest().json("ROLLBACK"),
                            Err(_) => {
                                HttpResponse::InternalServerError().json("failed to ROLLBACK")
                            }
                        },

                        false => match conn.execute("COMMIT").await {
                            Ok(_) => HttpResponse::Ok().json(results),
                            Err(_) => HttpResponse::InternalServerError().json("failed to COMMIT"),
                        },
                    }
                }
            }
        }
    }
}

/*
#[get("/examples")]
pub async fn examples(pool: web::Data<MySqlPool>) -> impl Responder {
    //example of a normal endpoint,
    //note: if database is running we can use macro eg query!() to verify sql with query!()
    //but it returns anonymous record so use regular query() when running

    let r = sqlx::query!(
        "UPDATE Event SET location = ? WHERE id = ?",
        None::<String>,
        1
    )
    .fetch_all(pool.get_ref())
    .await;

    let result = sqlx::query("SELECT * FROM `FullPrismaTypes`")
        .fetch_all(pool.get_ref())
        .await;
    match result {
        Ok(rows) => match sqlx_mysql_json::row::rows_to_value(&rows) {
            Ok(value) => HttpResponse::Ok().json(value),
            Err(err) => HttpResponse::BadRequest().json(err.to_string()),
        },
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}
*/
