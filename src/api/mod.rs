use super::*;

mod home;

pub fn mount() {
    rocket::ignite().mount("/", routes![
        home::index
    ]).launch();
}