use std::env;
use std::fs::File;
use std::io::Write;
use mysql::prelude::FromRow;
use models::Record;
use rocket_contrib::Template;
use mysql;

#[derive(Serialize)]
pub struct ZoneFileContext {
    pub serial: u64,
    pub records: Vec<Record>
}

pub fn trigger_update(pool: &mysql::Pool) -> Result<usize, String> {
    let mut context = ZoneFileContext {
        serial: 0,
        records: Vec::new()
    };

    // We get the zone file serial based on the 
    let result = pool.prep_exec("
		SELECT t1.name, t1.token, DATE_FORMAT(t1.expiration, '%Y-%m-%d %H:%i:%s'), t1.apikey, t2.id
			FROM tokens AS t1
		LEFT JOIN (
			SELECT MAX(id) AS id FROM tokens
		) AS t2 ON 1=1
		WHERE t1.expiration > NOW()",
	()).map(|result| {
		for row in result {
			match row {
				Ok(row) => {
					context.serial = mysql::from_value(row.get(4).unwrap());
					context.records.push(Record::from_row(row));
				},
				Err(_) => ()
			}
		}
	});

    match result {
        Err(err) => println!("error! {:?}", err.to_string()),
        Ok(_) => ()
    };


    let template = match env::var("DNS01_ZONETEMPLATE") {
        Ok(templ) => templ,
        Err(err) => return Err(format!("DNS01_ZONETEMPLATE environment variable is undefined: {}", err.to_string()))
    };

    let output = match env::var("DNS01_ZONEFILE") {
        Ok(templ) => templ,
        Err(err) => return Err(format!("DNS01_ZONEFILE environment variable is undefined: {}", err.to_string()))
    };

    let contents = match Template::show("templates/", template, &context) {
        Some(contents) => contents,
        None => return Err("Failed to render template".to_string())
    };

    let mut file = match File::create(&output) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to create zone file: {}", err.to_string()))
    };

    match file.write(&contents.as_bytes()) {
        Err(err) => Err(err.to_string()),
        Ok(_) => Ok(context.records.len())
    }
}
