pub mod controllers;
pub mod models;
pub mod requests;
pub mod service;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![controllers::show, controllers::destroy]
}
