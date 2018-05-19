#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/version")]
fn version() -> String {
    format!("v0.0.1")
}

#[post("/document/<doctype>")]
fn document(doctype: String) -> String {
    format!("You want document {}!", doctype)
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![version, document])
        //.mount("/api/document", routes![document])
        .launch();
}
