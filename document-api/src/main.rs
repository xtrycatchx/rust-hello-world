#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;
extern crate serde_json;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[derive(Debug)]
enum CustomError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}


use serde_json::Error;

use rocket_contrib::{Json, Value};

mod document;
use document::{Document};



mod document_data;
use document_data::{DocumentData};
use document_data::{Page};
use document_data::{Image};



use std::collections::HashMap;
use std::vec::Vec;

use reqwest::header::{Authorization, Basic};
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::Method::Post;


struct CCall {
    image: String
}

impl CCall {
    fn call(&self) -> Result<String, CustomError> {

        let credentials = Basic {
            username: "username".to_string(),
            password: Some("password".to_string()),
        };

        let mut map = HashMap::new();

        map.insert("title", "foo");
        map.insert("body", "bar");
        map.insert("userId", "1");

        let client = Client::new();

        let mut resp = client.request(Post, "https://jsonplaceholder.typicode.com/posts")
        .json(&map)
        .send()?;
        
        match resp.status() {
            StatusCode::Ok => println!("success!"),
            StatusCode::PayloadTooLarge => {
                println!("Request payload is too large!");
            }
            s => println!("Received response status: {:?}", s),
        };

        //println!("{}", resp.status());

        Ok("HEY".to_string())

    }
}




#[get("/version")]
fn api_version() -> String {
    format!("v0.0.1")
}

#[post("/test", format = "application/json",data = "<document>")]
fn ap_test(document: Json<Document>) -> String {
    let temp = &document.image;
    let c = CCall { image: temp.to_string() };
    c.call();
    
    format!("YEY")
    
}

#[post("/", format = "application/json",data = "<document>")]
fn api_document(document: Json<Document>) -> Json<Document> {
    document
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![api_version, ap_test])
        .mount("/api/document", routes![api_document])
        .launch();
}
