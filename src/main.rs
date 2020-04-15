#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tera;

#[path = "model/database.rs"] mod database;
#[path = "model/schema.rs"] mod schema;
#[path = "controller/routes.rs"] mod routes;
#[path = "controller/authentication/login.rs"] mod login;
#[path = "controller/sanitization.rs"] mod sanitize;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use dotenv::dotenv;

#[database("link_shortener_app")]
pub struct DbConnection(diesel::MysqlConnection);

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes![
            routes::index,
            routes::admin,
            routes::new_forward,
            routes::edit_forward,
            routes::delete_forward,
            routes::visibility_forward,
            routes::forwarder
        ])
        .mount("/", login::login_routes())
        .mount("/assets", StaticFiles::from("src/view/assets"))
        .register(catchers![
            routes::internal_error,
            routes::not_found
        ])
        .attach(DbConnection::fairing())
        .attach(Template::fairing())
        .launch();
}