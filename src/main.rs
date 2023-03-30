#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[cfg(test)]
mod tests;

#[get("/")]
fn index() -> &'static str {
    "It works!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/environment")]
fn environment() -> String {
    env::var("RUST_ENV").unwrap_or("yikes".to_string())
}

fn main() {
    dotenv().ok();

    let app_env = env::var("RUST_ENV").unwrap_or("yikes".to_string());
    dbg!(app_env);

    rocket::ignite()
        .mount("/", routes![index, hello, environment])
        .launch();
}
