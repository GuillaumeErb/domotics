#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

mod lights;

use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let no_lights: Vec<lights::WifiBulb> = vec![];
    rocket::ignite()
        .mount(
            "/lights",
            routes![lights::get_all, lights::get_one, lights::toggle],
        )
        .manage(Arc::new(Mutex::new(Arc::new(no_lights))))
        .launch();
}
