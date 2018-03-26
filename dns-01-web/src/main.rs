#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;

extern crate chrono;
use chrono::offset::Utc;
use chrono::Datelike;

extern crate rocket;
use rocket::response::{NamedFile};

extern crate rocket_contrib;
use rocket_contrib::Template;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

mod api;
mod auth;
mod models;
use models::User; 

mod database;
mod control;

pub fn build_template(
    user: &Option<User>, 
    context: Option<HashMap<&'static str, String>>
) -> HashMap<&'static str, String> {
    let year = Utc::now().year().to_string();

    let username = match user {
        &Some(ref usr) => usr.username.clone(),
        &None => "".to_string()
    };

    let apikey = match user {
        &Some(ref usr) => usr.apikey.clone(),
        &None => "".to_string()
    };

    match context {
        None => {
            let mut ctx = HashMap::new();
            ctx.insert("year", year);
            ctx.insert("username", username);
            ctx.insert("apikey", apikey);
            ctx
        },
        Some(mut ctx) => {
            ctx.insert("year", year);
            ctx.insert("apikey", apikey);
            ctx
        }
    }
}

fn main() {
    rocket::ignite()
        .manage(database::create_pool())
        .attach(Template::fairing())
        .mount("/", routes![
            static_files,
            index,
            usage,
            auth::overview,
            auth::register,
            auth::register_post,
            auth::login,
            auth::login_post,
            auth::logout,
            api::record_post,
            api::missing_endpoint,
            api::missing_endpoint_post,
            api::no_endpoint,
            api::no_endpoint_post
        ])
        .launch();
}

#[get("/")]
fn index(user: Option<User>) -> Template {
    Template::render("index", build_template(&user, None))
}

#[get("/usage")]
fn usage(user: Option<User>) -> Template {
    Template::render("usage", build_template(&user, None))
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
