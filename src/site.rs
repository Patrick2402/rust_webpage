use askama::{Template};
use axum::{response::{Html, IntoResponse}, extract::Path};
use hyper::{StatusCode, header, HeaderMap};


static THEME_CSS: &str = include_str!("../templates/style.css");

pub async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "style.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, THEME_CSS)
    }  else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}

pub async fn test_page() -> impl IntoResponse{
    let page = Testo::new();
    let rendered = page.render().unwrap();

    return 
    (StatusCode::OK,Html(rendered));
    (StatusCode::INTERNAL_SERVER_ERROR,"testowe");
}

#[derive(Template)]
#[template(path="test.html")]
pub struct Testo {

} 

impl Testo {
    pub fn new() -> Self {
        Self {  } 
    }
}