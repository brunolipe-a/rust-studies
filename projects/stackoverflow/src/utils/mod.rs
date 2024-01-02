use std::env;

pub fn location(uri: String) -> String {
    let app_url = env::var("APP_URL").expect("APP_URL must be set");

    format!("{}{}", app_url, uri)
}
