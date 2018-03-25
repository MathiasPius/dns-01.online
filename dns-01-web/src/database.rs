extern crate rand;
use self::rand::Rng;

extern crate crypto;
use self::crypto::sha3::Sha3;
use self::crypto::digest::Digest;

use std::env;
use models::{User, Record};

use mysql;
use mysql::prelude::FromRow;


pub fn create_pool() -> mysql::Pool {
    let connstr = format!(
        "mysql://{}:{}@{}/{}",
        env::var("DNS01_USERNAME").unwrap().as_str(),
        env::var("DNS01_PASSWORD").unwrap().as_str(),
        env::var("DNS01_HOSTNAME").unwrap().as_str(),
        env::var("DNS01_TABLE").unwrap().as_str()
    );

	return mysql::Pool::new(connstr).unwrap();
}


pub fn get_user(pool: &mysql::Pool, token: String) -> Result<User, String> {
	let row = pool.first_exec("
			SELECT id, username, passhash, salt, apikey
			FROM users
			WHERE apikey = :token",
		params!{ "token" => token }
	).unwrap();
	
    match row {
        None => Err("failed to get user".to_string()),
        Some(row) => Ok(User::from_row(row))
    }
}


pub fn login_user(pool: &mysql::Pool, username: &String, password: &String) -> Result<User, String> {
	let row = pool.first_exec("
			SELECT id, username, passhash, salt, apikey
			FROM users
			WHERE username = :username",
		params!{ "username" => username.to_string() }
	).unwrap();
	
    match row {
        None => Err("failed to get user".to_string()),
        Some(row) => {
            let user = User::from_row(row);
        
            let mut hasher = Sha3::sha3_256();
            hasher.input_str(&user.salt);
            hasher.input_str(&password);            
            
            if hasher.result_str() == user.passhash {
                Ok(user)
            } else {           
                Err("username or password invalid".to_string())
            }
        }
    }
}


pub fn create_user(pool: &mysql::Pool, username: &String, password: &String) -> Result<User, &'static str> {

    if username.len() < 6 {
        return Err("username must be at least 6 characters long");
    }

    if password.len() < 6 {
        return Err("password must be at least 6 characters long.");
    }

    let salt: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(32)
        .collect();

    let apikey: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(32)
        .collect();

    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&salt);
    hasher.input_str(&password);

    let passhash = hasher.result_str();

    // Prepare insertion statement
	let mut stmt = pool.prepare("
			INSERT INTO users (username, passhash, salt, apikey)
			VALUES(:username, :passhash, :salt, :apikey)
	").unwrap();

	// Insert row
    let insert = stmt.execute( params! { 
		"username" => &username, 
		"passhash" => &passhash,
        "salt" => &salt,
        "apikey" => &apikey
	});

    match insert {
        Err(_) => Err("failed to create new user. Username is most likely taken"),
        Ok(ins) => Ok(User {
                username: username.clone(),
                passhash: passhash,
                apikey: apikey,
                salt: salt,
                id: ins.last_insert_id()
            })
    }
}


pub fn get_record(pool: &mysql::Pool, name: &String) -> Option<Record> {
    let row = pool.first_exec("
			SELECT name, token, DATE_FORMAT(expiration, '%Y-%m-%d %H:%i:%s'), apikey
			FROM tokens 
			WHERE name = :name",
		params!{ "name" => name }
	).unwrap();
	
    match row {
        None => None,
        Some(row) => Some(Record::from_row(row))
    }
}

pub fn validate_apikey(pool: &mysql::Pool, apikey: &String) -> bool {
    let row = pool.first_exec("
			SELECT apikey 
			FROM users
			WHERE apikey = :apikey",
		params!{ "apikey" => apikey }
	).unwrap();
    
    match row {
        None => false,
        _ => true
    }
}


pub fn create_record(pool: &mysql::Pool, apikey: &String, name: &String, token: &String, ttl: u32) -> Result<Record, &'static str> {
    if name.len() > 63 {
        return Err("record names cannot be longer than 63 characters due to limitations in the dns specification");
    }

    if name.len() < 3 {
        return Err("record names must be at least 3 characters, but preferably longer");
    }

    if token.len() < 32 {
        return Err("the provided token does not appear to be valid");
    }

    if apikey.len() != 32 {
        return Err("the apikey is not valid");
    }

    if !validate_apikey(pool, apikey) {
        return Err("that apikey does not exist");
    }

    if ttl < 60 {
        return Err("ttl must be at least 60 seconds. You risk LetsEncrypt missing the update if less than that.");
    }

    if ttl > 86400 {
        return Err("ttl must be less than 24h (86400 seconds)");
    }

    // Check to make sure that if the token already exists, that we're the owners of it.
    match get_record(pool, &name) {
        Some(record) => {
            if record.apikey != *apikey {
                return Err("record with that name already exists, and you don't own it");
            }
        },
        _ => ()
    }
    
    // We use replace here so even if the row already exists, it gets regenerated with a new id,
    // Which is used to version the resulting zone file and keep the DNS updated.
	let mut stmt = pool.prepare("
			REPLACE INTO tokens (name, token, expiration, apikey)
			VALUES(:name, :token, DATE_ADD(NOW(), INTERVAL ABS(:ttl) SECOND), :apikey) 
	").unwrap();

    stmt.execute( params! { 
		"name" => name.clone(), 
		"token" => token.clone(), 
        "apikey" => apikey.clone(),
		"ttl" => ttl
	}).unwrap();

    match get_record(pool, name) {
        None => Err("could not create record"),
        Some(record) => Ok(record)
    }
}
















