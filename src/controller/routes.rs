#[path = "authentication/authorization.rs"] mod auth;
#[path = "../model/administrator.rs"] mod admin;

use admin::*;
use auth::*;

use rocket_contrib::templates::Template;

use crate::DbConnection;
use crate::database::Links;

use diesel::{self, prelude::*, insert_into, update};
use diesel::dsl::not;
use rocket::response::{ Redirect, Flash };
use rocket::http::{ Status, RawStr };
use rocket::Request;
use rocket::request::{ Form, FlashMessage };

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
    context.insert("title", "Link Shortener");
    // collect and send all links to the front end
    let links: Vec<(String, String)> = links::table
        .select((links::name, links::link))
        .filter(links::visible.eq(true))
        .load(&conn.0).expect("Error loading links");
    context.insert("data", &links);

    Template::render("home", &context)
}

#[get("/admin")]
pub fn admin(_user: AuthCont<AdministratorCookie>, conn: DbConnection, flash_message: Option<FlashMessage>) -> Template {
    use crate::schema::*;

    let alert;
    if let Some(flash) = flash_message {
        let al_type = flash.name().to_string();
        if al_type == "error" {
            alert = vec!["danger".to_string(), flash.msg().to_string()];
        } else {
            alert = vec!["success".to_string(), flash.msg().to_string()];
        }
    } else {
        alert = Vec::new()
    }

    let mut context = tera::Context::new();
    context.insert("title", "Link Shortener");
    context.insert("alert", &alert);
    // collect and send all links to the front end
    let links: Vec<(i32, String, String, bool)> = links::table
        .select((links::id, links::name, links::link, links::visible))
        .load(&conn.0).expect("Error loading links");
    context.insert("data", &links);
    Template::render("admin", &context)
}

#[derive(FromForm)]
pub struct NewForwarder {
    name: String,
    url: String,
    visible: bool
}

#[post("/admin/new", data="<form>")]
pub fn new_forward(_user: AuthCont<AdministratorCookie>, conn: DbConnection, form: Form<NewForwarder>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    use crate::schema::*;

    let inner = form.into_inner();
    insert_into(links::table)
        .values((links::name.eq(inner.name), links::link.eq(inner.url), links::visible.eq(inner.visible)))
        .execute(&conn.0)
        .map_err(|e| Flash::error(Redirect::to("/admin"), e.to_string()))
        .map(|_res| Flash::success(Redirect::to("/admin"), "Entry added."))
}

#[derive(FromForm)]
pub struct EditForwarder {
    id: i32,
    name: String,
    url: String,
    visible: bool
}

#[post("/admin/edit", data="<form>")]
pub fn edit_forward(_user: AuthCont<AdministratorCookie>, conn: DbConnection, form: Form<EditForwarder>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    use crate::schema::*;

    let inner = form.into_inner();
    let target = links::table.filter(links::id.eq(&inner.id));
    update(target)
        .set((links::name.eq(&inner.name), links::link.eq(inner.url), links::visible.eq(inner.visible)))
        .execute(&conn.0)
        .map_err(|e| Flash::error(Redirect::to("/admin"), format!("{}", e.to_string())))
        .map(|_res| Flash::success(Redirect::to("/admin"), "Entry edited."))
}

#[derive(FromForm)]
pub struct IDForwarder {
    id: i32
}

#[post("/admin/visibility", data="<form>")]
pub fn visibility_forward(_user: AuthCont<AdministratorCookie>, conn: DbConnection, form: Form<IDForwarder>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    use crate::schema::*;

    let inner = form.into_inner();
    let target = links::table.filter(links::id.eq(&inner.id));
    update(target)
        .set(links::visible.eq(not(links::visible)))
        .execute(&conn.0)
        .map_err(|e| Flash::error(Redirect::to("/admin"), e.to_string()))
        .map(|_res| Flash::success(Redirect::to("/admin"), "Entry edited."))
}

#[post("/admin/delete", data="<form>")]
pub fn delete_forward(_user: AuthCont<AdministratorCookie>, conn: DbConnection, form: Form<IDForwarder>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    use crate::schema::*;

    let inner = form.into_inner();
    diesel::delete(links::table.filter(links::id.eq(inner.id)))
        .execute(&conn.0)
        .map_err(|e| Flash::error(Redirect::to("/admin"), e.to_string()))
        .map(|_res| Flash::success(Redirect::to("/admin"), "Entry deleted."))
}

#[get("/<forward>", rank = 9)]
pub fn forwarder(conn: DbConnection, forward: &RawStr) -> Result<Redirect, Status> {
    use crate::schema::*;

    links::table.filter(links::name.eq(forward.to_string()))
        .first::<Links>(&conn.0)
        .map_err(|_e| Status::NotFound)
        .map(|res| Redirect::to(format!("{}", res.link)))
}

