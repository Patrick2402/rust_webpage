// this code is useless :)

use crate::page_no_params;
use askama::Template;
use std::path::PathBuf;
use tower_http::services::ServeDir;

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
