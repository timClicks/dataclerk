/// client.rs
///
/// Very much work in progress!

#[macro_use]
use log;

use http::{header, Method};
use actix_web::web::{Form, Path};
use actix_web::{middleware, web}, client;
use actix_web::{HttpResponse, HttpServer, Result, Client};

fn register(host &str, port: u16, channel: &str) {
    actix::run (move || {
        client::post(format!("http://{}:{}/{}/register", host, port, channel))
        .header("User-Agent", "dataclerk/0.1")
        .finish()
        .unwrap()
        .send()
        .map_err(|err| error!("Unable to register {} at {}:{}", channel, host, port); debug!("{:#?}", error ); ())
        .and_then(|response| info!("{} registered"); debug!("{:#?}", error ); Ok(()))
    })
}