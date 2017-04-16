mod home;

use super::super::*;

pub fn mount() {
    rocket::ignite().mount("/", routes![
        home::index
    ]).launch();
}