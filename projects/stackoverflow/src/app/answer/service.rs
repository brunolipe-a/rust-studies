use diesel::{insert_into, prelude::*};
use uuid::Uuid;

use super::models::{Answer, NewAnswer};
use crate::database::{establish_connection, schema};
use schema::answers::dsl::{self, answers};

pub struct AnswerService;

impl AnswerService {
    pub fn list(question_id: Uuid) -> Vec<Answer> {
        let connection = &mut establish_connection();

        answers
            .filter(dsl::question_id.eq(question_id))
            .load::<Answer>(connection)
            .expect("Error loading answers")
    }

    pub fn get_by_id(id: Uuid) -> Option<Answer> {
        let connection = &mut establish_connection();

        answers
            .find(id)
            .first::<Answer>(connection)
            .optional()
            .expect("Error loading question")
    }

    pub fn create(new_answer: NewAnswer) -> Answer {
        let connection = &mut establish_connection();

        insert_into(answers)
            .values(&new_answer)
            .get_result::<Answer>(connection)
            .expect("Error saving new answers")
    }

    pub fn delete(id: Uuid) {
        let connection = &mut establish_connection();

        diesel::delete(answers.find(id))
            .execute(connection)
            .expect("Error deleting answers");
    }
}
