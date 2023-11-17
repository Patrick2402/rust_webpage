// this code is useless :)
use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "test.html")]
pub struct Testo;

impl Testo {
    pub fn new() -> Self {
        Self
    }
}

pub async fn test_page() -> impl IntoResponse {
    let page = Testo::new();
    let rendered = page.render().unwrap();

    Html(rendered)
}

pub async fn user_page() -> &'static str {
    "User"
}
