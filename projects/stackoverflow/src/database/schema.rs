// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Uuid,
        question_id -> Uuid,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    questions (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(answers -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
);
