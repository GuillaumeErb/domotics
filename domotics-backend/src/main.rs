#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

mod lights;

fn main() {
    rocket::ignite()
        .mount("/lights", routes![lights::get_all])
        .launch();
}
