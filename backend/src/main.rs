#![feature(proc_macro_hygiene, decl_macro)]

use std::io::ErrorKind;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::{OptionalExtension};
use rocket_contrib::json::{Json, JsonValue};

use self::models::*;
use self::schema::urls::dsl::*;

mod models;
mod schema;
mod db;
mod services;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

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
fn redirect_route(url: String) -> JsonValue {
    let connection = &mut db::establish_connection();
    let original: Option<String> = urls.filter(shorted_url.eq(url.as_str()))
        .select(original_url)
        .first(connection)
        .ok();

    if !original.is_none() {
        json!({"original_url": original})
    } else {
        json!({"error": "not found"})
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![short_route, redirect_route])
}

fn main() {
    rocket().launch();
}