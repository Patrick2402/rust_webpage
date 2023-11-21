use askama::Template;

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

page_no_params!(Root, root_page, "index.html");
page_no_params!(Login, login_page, "auth/login.html");
page_no_params!(Register, register_page, "auth/register.html");
