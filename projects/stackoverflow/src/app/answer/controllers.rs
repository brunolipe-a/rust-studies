use rocket::{http::Status, serde::json::Json};

use uuid::Uuid;

use super::{models::Answer, service::AnswerService};

#[get("/<id>")]
pub fn show(id: &str) -> (Status, Option<Json<Answer>>) {
    let uuid = Uuid::parse_str(id).unwrap_or_default();

    let answer = AnswerService::get_by_id(uuid);

    match answer {
        Some(answer) => (Status::Ok, Some(Json(answer))),
        None => (Status::NotFound, None),
    }
}

#[delete("/<id>")]
pub fn destroy(id: &str) -> Status {
    let uuid = Uuid::parse_str(&id).unwrap_or_default();

    AnswerService::delete(uuid);

    Status::NoContent
}
