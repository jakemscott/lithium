use rocket_contrib::templates::Template;
use crate::DbConnection;
use crate::database::Links;
use diesel::{self, prelude::*};
use rocket::response::Redirect;
use rocket::http::RawStr;

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

#[get("/login" rank=1)]
pub fn logged_in(_user: Cookie) -> Template {

}


#[get("/<forward>")]
pub fn forwarder(conn: DbConnection, forward: &RawStr) -> Redirect {
    use crate::schema::*;

    let result = links::table.filter(links::name.eq(forward.to_string()))
        .first::<Links>(&conn.0)
        .expect("Error loading link");

    Redirect::to(format!("{}", &result.link))
}

