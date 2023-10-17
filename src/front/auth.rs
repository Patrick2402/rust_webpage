use askama::Template;

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

page_no_params!(Root, root, "index.html");
page_no_params!(Login, login, "auth/login.html");
page_no_params!(Register, register, "auth/register.html");
