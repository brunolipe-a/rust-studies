use rocket::{http::Status, response::status::Created, serde::json::Json};

use uuid::Uuid;

use crate::{
    app::answer::{
        models::{Answer, NewAnswer},
        requests::StoreAnswerRequest,
        service::AnswerService,
    },
    utils::location,
};

use super::{
    models::{NewQuestion, Question},
    requests::StoreQuestionRequest,
    service::QuestionService,
};

#[get("/")]
pub fn index() -> Json<Vec<Question>> {
    let questions = QuestionService::list();

    Json(questions)
}

#[get("/<id>")]
pub fn show(id: &str) -> (Status, Option<Json<Question>>) {
    let uuid = Uuid::parse_str(&id).unwrap_or_default();

    let question = QuestionService::get_by_id(uuid);

    match question {
        Some(question) => (Status::Ok, Some(Json(question))),
        None => (Status::NotFound, None),
    }
}

#[post("/", format = "json", data = "<request>")]
pub fn store(request: Json<StoreQuestionRequest>) -> Created<Json<Question>> {
    let question = QuestionService::create(NewQuestion::from_json_request(request));

    Created::new(location(format!("/questions/{}", question.id))).body(Json(question))
}

#[delete("/<id>")]
pub fn destroy(id: &str) -> Status {
    let uuid = Uuid::parse_str(&id).unwrap_or_default();

    QuestionService::delete(uuid);

    Status::NoContent
}

#[get("/<id>/answers")]
pub fn answers(id: &str) -> Json<Vec<Answer>> {
    let uuid = Uuid::parse_str(id).unwrap_or_default();

    let answers = AnswerService::list(uuid);

    Json(answers)
}

#[post("/<id>/answers", format = "json", data = "<request>")]
pub fn store_answer(id: &str, request: Json<StoreAnswerRequest>) -> Created<Json<Answer>> {
    let uuid = Uuid::parse_str(id).unwrap_or_default();

    let new_answer = NewAnswer::new(uuid, request.content.to_owned());

    let answer = AnswerService::create(new_answer);

    Created::new(location(format!("/answers/{}", answer.id))).body(Json(answer))
}
