//! # dataclerk
//!
//! A small data tool for storing structured logging data in an easy-to-use format.
//!
//! ## Technology choices
//!
//! `dataclerk` uses [Sqlite 3][] to store data. Unlike a text file, it's possible
//! to plug your

extern crate actix_web;

extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

use std::collections::HashMap;

use actix_web::web::{Form, Path};
use actix_web::{middleware, web};
use actix_web::{App, HttpResponse, HttpServer, Result};
use clap::{App as Cli, Arg, SubCommand};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use regex;
use uuid::Uuid;

/// ArbitraryInputData represents data sent to the server via a HTTP POST
/// request that has been marshalled into a hashmap.
type ArbitraryInputData = Form<HashMap<String, String>>;

type Database = web::Data<Pool<SqliteConnectionManager>>;

fn ok_no_content() -> HttpResponse {
    let no_content = actix_web::http::StatusCode::from_u16(204).unwrap();
    HttpResponse::build(no_content).finish()
}

/// Checks to see whether `name` is a safe input string.
/// A _safe_ input string is one that starts with an ASCII letter
/// and only contains ASCII letters, numerals, hyphens and underscores.
///
/// ```
/// let is_valid = is_valid_table_name("shiny_relation");
/// assert!(is_valid);
/// ```
///
/// ```
/// let is_valid = is_valid_table_name("Robert'; DROP TABLE Students; --");
/// assert!(!is_valid);
/// ```
#[inline]
fn is_valid_table_name(name: &str) -> bool {
    lazy_static! {
        static ref TABLE_NAME: regex::Regex = regex::Regex::new("[a-zA-Z][a-zA-Z0-9-_]*").unwrap();
    }

    return TABLE_NAME.is_match(name);
}
//
// assisted greatly by https://github.com/actix/examples/tree/master/r2d2
fn register(path: Path<String>, db: Database) -> HttpResponse {
    let channel = path.into_inner();
    info!("registering channel {:#?}", channel);
    if !is_valid_table_name(&channel) {
        return HttpResponse::NotAcceptable().finish();
    };
    let conn = db.get().unwrap();
    let stmt = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            uuid TEXT,
            data TEXT
        )",
        channel
    );
    let res = conn.execute(&stmt, rusqlite::NO_PARAMS);
    if res.is_err() {
        error!("{}", format!("{:#?}", res));
        return HttpResponse::InternalServerError().finish();
    };
    let stmt = format!(
        "CREATE INDEX IF NOT EXISTS {}_recorded_at_idx
        ON {} (
            recorded_at
        )",
        channel, channel
    );
    let res = conn.execute(&stmt, rusqlite::NO_PARAMS);
    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            error!("{:#?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// assisted greatly by https://github.com/actix/examples/tree/master/r2d2
fn recv((channel, data, db): (Path<String>, ArbitraryInputData, Database)) -> HttpResponse {
    if !is_valid_table_name(&channel) {
        debug!("recv: <illegal channel>");
        return HttpResponse::NotAcceptable().finish();
    }
    debug!("recv: channel:{:?}, data: {:?}", channel, data);

    let data_str = serde_json::to_string(&data.0).unwrap();
    let uuid = Uuid::new_v4().to_string();

    let conn = db.get().unwrap();
    let stmt = format!("INSERT INTO {} (uuid, data) VALUES ($1,$2)", channel);
    let res = conn.execute(&stmt, &[&uuid, &data_str]);

    // not using 201 Created because we don't provide the ability to read any records yet
    match res {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            error!("{:#?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn serve(
    system: actix_rt::SystemRunner,
    db_manager: SqliteConnectionManager,
    bind_to: &str,
) -> Result<()> {
    let pool = r2d2::Pool::new(db_manager).unwrap();

    info!("Hello!");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(ok_no_content)))
            .service(web::resource("/v1/channel/{channel}").route(web::put().to(register)))
            .service(web::resource("/v1/send/{channel}").route(web::post().to(recv)))
            .service(web::resource("/+/{channel}").route(web::post().to(recv)))
    })
    .bind(bind_to)?
    .start();

    system.run()?;
    info!("Goodbye");

    Ok(())
}

fn main() -> Result<()> {
    let logging_env_var = "RUST_LOG";
    match std::env::var(logging_env_var) {
        Ok(_) => {}
        Err(_) => {
            std::env::set_var("RUST_LOG", "info,dataclerk");
        }
    }
    env_logger::init();

    let options = Cli::new("dataclerk")
        .author("Tim McNmamara <dataclerk@timmcnamara.co.nz>")
        .version("v0.1")
        .about("HTTP data logger")
            .arg(Arg::from_usage("<address> 'Hostname/IP address and port pair for the server to listen to.'").default_value("localhost:4499"))
            .arg(Arg::from_usage("<database> 'Database file to connect to. Will be appended to if it already exists. Use :memory: for an in-memory database.").default_value("./dataclerk.sqlite"))
        // .subcommand(SubCommand::with_name("serve")
        //     .about("Starts a dataclerk server that receives messages.")
        //     .arg(Arg::from_usage("<address> 'Hostname/IP address and port pair for the server to listen to.'").default_value("localhost:4499"))
        //     .arg(Arg::from_usage("<database> 'Database file to connect to. Will be appended to if it already exists. Use :memory: for an in-memory database.").default_value("./dataclerk.sqlite"))
        // )
        // .subcommand(SubCommand::with_name("send")
        //     .about("Send data to a registered channel on a dataclerk server that's currently listening.")
        //     .arg(Arg::from_usage("<address> 'Hostname/IP address and port for dataclerk to connect to.'").default_value("localhost:4499"))
        //     .arg(Arg::from_usage("<channel> 'Channel to add message to'"))
        //     .arg(Arg::from_usage("-d 'key=value data pair to be sent'"))
        //     .arg(Arg::from_usage("-f <file> 'File to be sent'"))
        // )
        // .subcommand(SubCommand::with_name("register")
        //     .about("Create a channel to record data to.")
        //     .arg(Arg::from_usage("<address> 'Hostname/IP address and port for dataclerk to connect to.'").default_value("localhost:4499"))
        //     .arg(Arg::from_usage("<channel> 'Channel to register.'").long_help("Channel to register.\nMust start with a Latin letter (a-z, A-Z) and contain only letters (a-z, A-Z), numerals (0-9), hyphens (-) and underscores (_)."))
        // )
        .after_help("dataclerk receives data via HTTP POST and stores it in a sqlite database for later analysis. All entries are stored as well-formed JSON and tagged with a timestamp and UUID.")
        .get_matches();

    let database = options.value_of("database").unwrap_or("./dataclerk.sqlite");
    let address = options.value_of("address").unwrap_or("localhost:4499");
    let manager = if database == ":memory:" {
        SqliteConnectionManager::memory()
    } else {
        SqliteConnectionManager::file(database)
    };
    let sys = actix_rt::System::new("dataclerk");
    serve(sys, manager, address)

    // let result = match options.subcommand_name() {
    //     None => { unreachable!(); Err("huh?") },
    //     Some("serve") => {
    //        let sys = actix_rt::System::new("dataclerk");

    //         let manager = if database == ":memory:" {
    //             SqliteConnectionManager::memory()
    //         } else {
    //             SqliteConnectionManager::file(database)
    //         };
    //         serve(sys, manager, address)
    //     },
    //     Some("register") =>
    //         let channel = options.value_of("channel").expect("<channel> option is missing.");
    //         client::register(hostname, port, channel)
    //     },
    // }
}
