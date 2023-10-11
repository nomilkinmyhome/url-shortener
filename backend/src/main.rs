#![feature(proc_macro_hygiene, decl_macro)]

use std::io::ErrorKind;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::{OptionalExtension};
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::Redirect;

pub struct CORS;

use self::models::*;
use self::schema::urls::dsl::*;

mod models;
mod schema;
mod db;
mod services;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/?<url>")]
fn short_route(url: String) -> JsonValue {
    let connection = &mut db::establish_connection();
    let shorted: String = services::shorten_url();
    insert_into(urls).values((
        original_url.eq(url.as_str()),
        shorted_url.eq(shorted.as_str()),
    )).execute(connection).expect("Database error");
    json!({"shorted_url": shorted.as_str()})
}

#[get("/<url>")]
fn redirect_route(url: String) -> Result<Redirect, JsonValue> {
    let connection = &mut db::establish_connection();
    let original: Option<String> = urls.filter(shorted_url.eq(url.as_str()))
        .select(original_url)
        .first(connection)
        .ok();

    if !original.is_none() {
        Ok(Redirect::to(original.expect("none").to_string()))
    } else {
        Err(json!({"error": "not found"}))
    }
}


fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![short_route, redirect_route])
        .attach(CORS)
}

fn main() {
    rocket().launch();
}