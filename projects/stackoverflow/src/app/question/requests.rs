use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct StoreQuestionRequest<'r> {
    pub title: &'r str,
    pub description: &'r str,
}
