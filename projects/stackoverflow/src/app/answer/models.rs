use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::schema::answers;
use crate::question::models::Question;

#[derive(
    Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, Debug, PartialEq,
)]
#[diesel(belongs_to(Question))]
#[diesel(table_name = answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Answer {
    pub id: Uuid,
    pub question_id: Uuid,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = answers)]
pub struct NewAnswer {
    pub id: uuid::Uuid,
    pub question_id: Uuid,
    pub content: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl NewAnswer {
    pub fn new(question_id: Uuid, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            question_id,
            content,
            created_at: None,
        }
    }
}
