#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Set DB url !");
    let pool = db::init_pool(db_url);
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![static_files::all, static_files::index])
}

fn main() {
    rocket().launch();
}
