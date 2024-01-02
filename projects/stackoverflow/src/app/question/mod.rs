pub mod controllers;
pub mod models;
pub mod requests;
pub mod service;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        controllers::index,
        controllers::show,
        controllers::store,
        controllers::destroy,
        controllers::answers,
        controllers::store_answer
    ]
}
