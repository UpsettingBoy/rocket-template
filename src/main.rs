#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{fs::FileServer, get, launch, routes};

mod templates;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
}

#[get("/")]
fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
    }
}
