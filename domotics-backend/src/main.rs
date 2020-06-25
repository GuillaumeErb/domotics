#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

use clokwerk::ScheduleHandle;
use clokwerk::{Scheduler, TimeUnits};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::time::Duration;

mod frontend;
mod lights;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn launch_scheduler() -> ScheduleHandle {
    println!("Setting up scheduler");

    let mut scheduler = Scheduler::new();
    println!("Updating light state ...");
    let _ = lights::get_all(Some(true));
    scheduler.every(20.minutes()).run(|| {
        println!("Updating light state ...");
        let _ = lights::get_all(Some(true));
    });
    scheduler.watch_thread(Duration::from_millis(100))
}

fn main() {
    println!("Launching !");
    let _thread_handle: ScheduleHandle = launch_scheduler();

    println!("Setting up rocket");

    rocket::ignite()
        .mount(
            "/lights",
            routes![lights::get_all, lights::get_one, lights::toggle],
        )
        .mount("/", routes![frontend::index, frontend::file])
        .attach(make_cors())
        .launch();
    println!("Shutting down");
}
