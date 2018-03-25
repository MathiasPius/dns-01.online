extern crate serde_json;

use std::io::Read;

use mysql;

use rocket::{Data, State};
use rocket::{Request};
use rocket::data::{self, FromData};
use rocket::Outcome::{Failure, Success};
use rocket::http::Status;

use rocket_contrib::Template;

use database::create_record;
use control::trigger_update;

use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordData {
    pub apikey: String,
    pub name: String,
    pub token: String,
    pub ttl: u32
}

#[post("/api/record", format = "application/json", data="<record>")]
pub fn record_post(record: Result<RecordData, String>, pool: State<mysql::Pool>) -> Template {
    match record {
        Ok(record) => {
            let record = create_record(pool.inner(), &record.apikey, &record.name, &record.token, record.ttl);
            match record {
                Err(err) => {
                    let mut context = HashMap::new();
                    context.insert("error", err.to_string());

                    Template::render("apierror", context)
                },
                Ok(record) => {
                    let mut context = HashMap::new();
                    context.insert("name", &record.name);
                    context.insert("token", &record.token);
                    context.insert("expires_at", &record.expiration);
                    
                    match trigger_update(pool.inner()) {
                        Err(err) => println!("failed to trigger zone file update: {}", err.to_string()),
                        _ => ()
                    };

                    Template::render("apirecord", context)
                }
            }
        },
        Err(record) => {
            let mut context = HashMap::new();
            context.insert("error", record);
            Template::render("apierror", context)
        }
    }
}

#[get("/api/<file..>")]
pub fn missing_endpoint(file: Option<PathBuf>) -> Template {
    let mut context = HashMap::new();

    match file {
        None =>  context.insert("error", "use POST /api/record".to_string()),
        Some(file) => context.insert("error", 
            format!("the endpoint '{}' does not exist. use POST /api/record", file.to_str().unwrap().to_string())
        )
    };

    Template::render("apierror", context)
}

#[post("/api/<file..>")]
pub fn missing_endpoint_post(file: PathBuf) -> Template {
    missing_endpoint(Some(file))
}

#[get("/api")] pub fn no_endpoint() -> Template { missing_endpoint(None) }
#[post("/api")] pub fn no_endpoint_post() -> Template { missing_endpoint(None) }

impl FromData for RecordData {
    type Error = String;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, String> {
	
		let mut reader = data.open();
		let mut buf = String::new();
				
		reader.read_to_string(&mut buf).unwrap();
		
        match serde_json::from_str(&buf).map(|val| val) {
            Ok(value) => Success(value),
            Err(e) => Failure((Status::BadRequest, e.to_string()))
        }
    }
}
