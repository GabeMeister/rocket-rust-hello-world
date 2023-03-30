#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

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

// #[get("/data")]
// fn select_something() -> String {

// }

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
    println!("HELLO!");
}
