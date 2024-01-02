use diesel::{insert_into, prelude::*};
use uuid::Uuid;

use super::models::{NewQuestion, Question};
use crate::database::{establish_connection, schema};
use schema::questions::dsl::questions;

pub struct QuestionService;

impl QuestionService {
    pub fn list() -> Vec<Question> {
        let connection = &mut establish_connection();

        questions
            .load::<Question>(connection)
            .expect("Error loading questions")
    }

    pub fn get_by_id(id: Uuid) -> Option<Question> {
        let connection = &mut establish_connection();

        questions
            .find(id)
            .first::<Question>(connection)
            .optional()
            .expect("Error loading question")
    }

    pub fn create(new_question: NewQuestion) -> Question {
        let connection = &mut establish_connection();

        insert_into(questions)
            .values(&new_question)
            .get_result::<Question>(connection)
            .expect("Error saving new questions")
    }

    pub fn delete(id: Uuid) {
        let connection = &mut establish_connection();

        diesel::delete(questions.find(id))
            .execute(connection)
            .expect("Error deleting questions");
    }
}
