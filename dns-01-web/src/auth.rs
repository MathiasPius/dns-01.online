extern crate rocket;
use rocket::{State};
use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, FlashMessage};
use rocket::response::{Redirect, Flash};

extern crate rocket_contrib;
use rocket_contrib::Template;

use models::User;
use database::{create_user, login_user};
use build_template;

use mysql;

use std::collections::HashMap;

#[derive(FromForm)]
pub struct RegisterData {
    pub username: String,
    pub password: String
}

#[get("/overview")]
pub fn overview(user: Option<User>) -> Result<Template, Flash<Redirect>> {
    match user {
        Some(user) => Ok(Template::render("overview", build_template(&Some(user), None))),
        None => Err(Flash::error(Redirect::to("/login"), "You need to login or register to get an API key"))
    }
}

#[get("/register")]
pub fn register(user: Option<User>, flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
            context.insert("flash", msg.msg().to_string());
    }

    Template::render("register", build_template(&user, Some(context)))
}

#[post("/register", data = "<user>")]
pub fn register_post(
    mut cookies: Cookies, 
    pool: State<mysql::Pool>, 
    user: Form<RegisterData>
) -> Flash<Redirect> {
    let input = user.get();
    let user = create_user(pool.inner(), &input.username, &input.password);

    match user {
        Err(err) => Flash::error(Redirect::to("/register"), err.to_string()),
        Ok(usr) => {
            cookies.add_private(Cookie::new("usertoken", usr.apikey.to_string()));

            Flash::success(Redirect::to("/"), "User successfully created")
        }
    }
}

#[get("/login")]
pub fn login(user: Option<User>, flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
            context.insert("flash", msg.msg().to_string());
    }

    Template::render("login", build_template(&user, Some(context)))
}

#[post("/login", data = "<user>")]
pub fn login_post(
    mut cookies: Cookies, 
    pool: State<mysql::Pool>, 
    user: Form<RegisterData>
) -> Flash<Redirect> {
    let input = user.get();
    let user = login_user(pool.inner(), &input.username, &input.password);

    match user {
        Err(err) => Flash::error(Redirect::to("/login"), err.to_string()),
        Ok(usr) => {
            cookies.add_private(Cookie::new("usertoken", usr.apikey.to_string()));

            Flash::success(Redirect::to("/"), "Logged in")
        }
    }
}
#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("usertoken"));
    Redirect::to("/")
}
