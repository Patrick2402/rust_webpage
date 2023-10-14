use askama::Template;

#[derive(Template)]
#[template(path = "auth/index.html")]
pub struct Login;

pub async fn login() -> Login {
    Login
}
