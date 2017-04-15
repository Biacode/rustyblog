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

fn foo() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();

    let post = create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";