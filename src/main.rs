#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate rustyblog;
extern crate diesel;

use self::rustyblog::*;
use self::rustyblog::models::*;
use self::diesel::prelude::*;

#[get("/")]
fn index() -> &'static str {
    use rustyblog::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}\n", post.body);
    }

    "Welcome to RustyBlog!!!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}