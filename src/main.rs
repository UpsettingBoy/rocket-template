#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;

mod templates;

fn main() {
    rocket::ignite()
        .mount("/public/css", StaticFiles::from("public/css"))
        .mount("/", routes![index])
        .launch();
}

#[get("/")]
fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
    }
}
