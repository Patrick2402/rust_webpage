use askama::Template;
use askama_axum::IntoResponse;
use axum_csrf::CsrfToken;

#[macro_export]
macro_rules! page_no_params {
    ($name:ident, $fname:ident, $path:expr) => {
        #[derive(Template)]
        #[template(path = $path)]
        pub struct $name;

        pub async fn $fname() -> $name {
            $name
        }
    };
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Root;

pub async fn root_page(token: CsrfToken) -> impl IntoResponse {
    (token, Root).into_response()
}


page_no_params!(Login, login_page, "auth/login.html");
page_no_params!(Register, register_page, "auth/register.html");
