#[path = "authorization.rs"] mod auth;
#[path = "hashing.rs"] mod hashing;
#[path = "../../model/administrator.rs"] mod admin;

use crate::login::auth::*;
use crate::login::admin::*;

use rocket_contrib::templates::Template;
use rocket::request::{ FlashMessage, Form };
use rocket::response::{ Redirect, Flash };
use rocket::http::{ Cookies };
use crate::DbConnection;


//use a request guard to ensure that a user is logged in. If cookie present redirect to admin page
#[get("/login", rank = 1)]
fn logged_in(_user: AuthCont<AdministratorCookie>) -> Redirect {
    Redirect::to("/admin")
}

#[get("/login", rank = 2)]
fn login() -> Template {
    let mut context = tera::Context::new();
    context.insert("title", "Login");
    Template::render("login", &context)
}

/// if there is a user query string, and an optional flash message
/// display an optional flash message indicating why the login failed
/// and the login screen with user filled in
#[get("/login?<user>")]
fn retry_login_user(user: UserQuery, flash_msg_opt: Option<FlashMessage>) -> Template {
    let alert;
    if let Some(flash) = flash_msg_opt {
        alert = flash.msg().to_string();
    } else {
        alert = String::new();
    }

    let mut context = tera::Context::new();
    context.insert("title", "Login");
    context.insert("alert", &alert);
    context.insert("user", &user.user);

    Template::render("login", &context)
}

/// if there is a flash message but no user query string
/// display why the login failed and display the login screen
#[get("/login", rank = 3)]
fn retry_login_flash(flash_msg: FlashMessage) -> Template {
    println!("Retrying login...");
    let mut context = tera::Context::new();

    context.insert("title", "Login");
    context.insert("alert", &flash_msg.msg());

    Template::render("login", &context)
}

#[allow(unused_mut)]
#[post("/login", data = "<form>")]
fn process_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies, conn: DbConnection) -> Result<Redirect, Flash<Redirect>> {
    let inner = form.into_inner();
    let login = inner.form;
    login.flash_redirect("/admin", "/login", &mut cookies, conn)
}

#[get("/admin")]
pub fn admin(_user: AuthCont<AdministratorCookie>) -> Template {
    let mut context = tera::Context::new();
    context.insert("title", "jms.sc");
    Template::render("admin", &context)
}

// make all login routes available
pub fn login_routes() -> Vec<rocket::Route> {
    routes![
        admin,
        login,
        logged_in,
        retry_login_user,
        retry_login_flash,
        process_login
    ]
}