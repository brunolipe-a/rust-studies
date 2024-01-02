use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct StoreAnswerRequest<'r> {
    pub content: &'r str,
}
