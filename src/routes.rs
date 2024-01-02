use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Acquire, MySqlPool};

#[derive(Deserialize)]
struct Q {
    q: String,
}

#[get("/")]
async fn root(pool: web::Data<MySqlPool>, query: web::Query<Q>) -> impl Responder {
    //println!("query.q: {:?}", query.q);
    //general purpose "query via http"
    let result = sqlx_mysql_json::query(&pool, &query.q).await;
    match result {
        Ok(value) => {
            //println!("responding with value: {:?}", value);
            HttpResponse::Ok().json(value)
        }
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[post("/transaction")]
pub async fn transaction(
    pool: web::Data<MySqlPool>,
    queries: web::Json<Vec<String>>,
) -> impl Responder {
    // multiple queries in sequence, with rollback if one fails.
    // cant really use results of earlier queries in later queries here like in a real transaction
    //
    // also "data definition" stuff like CREATE and ALTER etc all do implicit
    // commits and can not be rolled back, see: https://dev.mysql.com/doc/refman/8.0/en/implicit-commit.html
    // so they are commited even on error / rollback
    //
    // btw, apparently, this auto commit behaviour also applies to regular queries unless explicitly doing "SET autocommit=0"
    // tldr from mysql8 reference manual (pseudo quotes):
    // "autocommit=1 is not recommended for transactions"
    // "all other databases has autocommit=0 as default"
    // "mysql has autocommit=1 as default because... no reason"

    //println!("transaction, queries: {:?}", queries);
    let mut results: Vec<serde_json::Value> = vec![];

    match pool.acquire().await {
        Err(_) => HttpResponse::InternalServerError()
            .json("couldnt acquire connection from pool".to_string()),
        Ok(mut conn) => match conn.begin().await {
            Err(_) => {
                HttpResponse::InternalServerError().json("couldnt begin transaction".to_string())
            }
            Ok(mut tx) => {
                match sqlx::query("SET autocommit=0").execute(&mut *tx).await {
                    Ok(_) => {}
                    Err(_) => {
                        return HttpResponse::InternalServerError()
                            .json("could not set autocommit=0 at start of transaction".to_string())
                    }
                }

                for q in queries.into_inner() {
                    //see example of transaction here: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/transaction/src/main.rs
                    match sqlx_mysql_json::execute_in_transaction(&mut tx, &q).await {
                        Ok(result) => {
                            results.push(result);
                        }
                        Err(err) => match tx.rollback().await {
                            Ok(_) => {
                                return HttpResponse::BadRequest()
                                    .json(format!("Rolled back, err: {:?}", err.to_string()))
                            }
                            Err(rollbackerr) => {
                                return HttpResponse::InternalServerError().json(format!(
                                    "Explicit rollback failed (never comitted, implicitly rolled back). err: {:?}",
                                    rollbackerr.to_string()
                                ))
                            }
                        },
                    }
                }
                match tx.commit().await {
                    Ok(_) => HttpResponse::Ok().json(results),
                    Err(err) => {
                        return HttpResponse::InternalServerError().json(format!(
                            "Commit failed (never comitted, implicitly rolled back). err: {:?}",
                            err.to_string()
                        ))
                    }
                }
            }
        },
    }
}

/*
#[get("/examples")]
pub async fn examples(pool: web::Data<MySqlPool>) -> impl Responder {
    //example of a normal endpoint,
    //note: if database is running we can use macro to verify sql with query!()
    //but it returns anonymous record so use regular query() when running
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
