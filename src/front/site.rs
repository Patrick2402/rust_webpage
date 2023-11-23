// this code is useless :)

use askama::Template;
use axum::response::{Html, IntoResponse};
use std::path::PathBuf;
use tower_http::services::ServeDir;

use crate::page_no_params;

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

page_no_params!(Map, map_page, "map.html");
page_no_params!(Admin, admin_page, "admin.html");

pub fn create_asset_dir_service() -> ServeDir {
    ServeDir::new(
        [std::env::current_dir().unwrap().to_str().unwrap(), "assets"]
            .iter()
            .collect::<PathBuf>(),
    )
}
