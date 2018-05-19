#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod document;
use document::{Document};

#[get("/version")]
fn api_version() -> String {
    format!("v0.0.1")
}

#[post("/", format = "application/json",data = "<document>")]
fn api_document(document: Json<Document>) -> Json<Document> {
    document
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![api_version])
        .mount("/api/document", routes![api_document])
        .launch();
}
