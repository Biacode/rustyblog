#[get("/")]
fn index() -> &'static str {
    "Welcome to RustyBlog!!!"
}