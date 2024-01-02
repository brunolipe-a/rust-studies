use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::schema::questions;

use super::requests::StoreQuestionRequest;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Question {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl NewQuestion {
    pub fn from_json_request(request: Json<StoreQuestionRequest>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: request.title.to_owned(),
            description: request.description.to_owned(),
            created_at: None,
        }
    }
}
