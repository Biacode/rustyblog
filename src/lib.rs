#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

mod api;
mod service;
mod persistence;

// re exports
pub use api::mount;