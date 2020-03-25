use rocket_contrib::templates::Template;

use crate::DbConnection;
use crate::database::Links;

use diesel::{ self, prelude::* };
use rocket::response::Redirect;
use rocket::http::{ Status, RawStr };
use rocket::Request;

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[get("/")]
pub fn index(conn: DbConnection) -> Template {
    use crate::schema::*;

    let mut context = tera::Context::new();
    context.insert("title", "jms.sc");
    // collect and send all links to the front end
    let links: Vec<(String, String)> = links::table
        .select((links::name, links::link))
        .load(&conn.0).expect("Error loading links");
    context.insert("data", &links);

    Template::render("home", &context)
}

#[get("/<forward>", rank = 9)]
pub fn forwarder(conn: DbConnection, forward: &RawStr) -> Result<Redirect, Status> {
    use crate::schema::*;

    links::table.filter(links::name.eq(forward.to_string()))
        .first::<Links>(&conn.0)
        .map_err(|e| Status::NotFound)
        .map(|res| Redirect::to(format!("{}", res.link)))
}

