#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

use clokwerk::ScheduleHandle;
use clokwerk::{Scheduler, TimeUnits};
use std::time::Duration;

mod lights;

fn main() {
    println!("Launching !");
    println!("Setting up scheduler");

    let mut scheduler = Scheduler::new();
    println!("Updating light state ...");
    lights::get_all();
    scheduler.every(20.minutes()).run(|| {
        println!("Updating light state ...");
        lights::get_all();
    });
    let _thread_handle: ScheduleHandle = scheduler.watch_thread(Duration::from_millis(100));

    println!("Setting up rocket");

    rocket::ignite()
        .mount(
            "/lights",
            routes![lights::get_all, lights::get_one, lights::toggle],
        )
        .launch();
    println!("Shutting down");
}
