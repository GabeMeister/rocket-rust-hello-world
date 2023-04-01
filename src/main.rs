#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use std::env;

#[derive(Debug, FromRow, Serialize)]
struct User {
    id: i32,
    name: String,
}
struct GabeTest {
    pool: PgPool,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
async fn test(gabe: &State<GabeTest>) -> Json<Option<User>> {
    let user = sqlx::query_as::<_, User>("select id, name from users where id=$1")
        .bind(10)
        .fetch_one(&gabe.pool)
        .await;

    Json(Some(user.unwrap()))
}

#[get("/test2/<id>")]
async fn test2(id: i32, gabe: &State<GabeTest>) -> Json<Option<User>> {
    let user = sqlx::query_as::<_, User>("select id, name from users where id=$1")
        .bind(id)
        .fetch_one(&gabe.pool)
        .await;

    match user {
        Ok(u) => Json(Some(u)),
        Err(e) => {
            dbg!(e);
            Json(None)
        }
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    rocket::build()
        .manage(GabeTest { pool })
        .mount("/", routes![index, test, test2])
}
