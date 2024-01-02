#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

mod app;
mod config;
mod database;
mod utils;

use app::{answer, question};
use config::cors;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/questions", question::routes())
        .mount("/answers", answer::routes())
        .attach(cors::CORS)
}
