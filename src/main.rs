#![allow(unused)] // silence unused warnings while exploring (to comment out)

mod data;
mod db;
mod logger;
mod redis_datastore;
mod routes;

extern crate redis;
use rand::{distributions::Alphanumeric, Rng};
use redis_datastore::connection_manager;
use serde::{Deserialize, Serialize};
use serde_json::Result as sj;
use std::{error::Error, time::Duration};
use tokio::time::sleep;
use ulid::Ulid;

use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};

use redis::aio::ConnectionManager;
use std::convert::Infallible;
use std::env;
use thiserror::Error;

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

//use db::db::Db;

type StdErr = Box<dyn std::error::Error>;

const REDIS_CON_STRING: &str = "redis://127.0.0.1/";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// async fn do_redis_stuff() -> Result<(), Box<dyn Error>> {
//     match env::var("REDIS_HOSTNAME") {
//         Ok(redis) => {
//             let mut my_con =
//                 match redis_datastore::connection_manager::get_connection_manager().await {
//                     Ok(res) => res.clone(),
//                     Err(e) => {
//                         //println!("{:?}", e);
//                         println!("{}", e);
//                         panic!("oh noes");
//                     }
//                 };

//             let p = Person {
//                 name: rand::thread_rng()
//                     .sample_iter(&Alphanumeric)
//                     .take(7)
//                     .map(char::from)
//                     .collect(),
//                 age: 30,
//                 phones: vec!["123-456-7890".to_string(), "123-456-7891".to_string()],
//             };

//             let _: () = redis::pipe()
//                 .atomic()
//                 .set(Ulid::new().to_string(), serde_json::to_string(&p).unwrap())
//                 .expire("pylly", 600)
//                 .query_async(&mut my_con)
//                 .await?;
//         }
//         Err(_) => {}
//     }
//     Ok(())
// }

// #[get("/person")]
// async fn person(req_body: String) -> impl Responder {
//     let p = Person {
//         name: "John".to_string(),
//         age: 30,
//         phones: vec!["123-456-7890".to_string(), "123-456-7891".to_string()],
//     };
//     do_redis_stuff().await;
//     HttpResponse::Ok().body(serde_json::to_string(&p).unwrap())
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey hey there!")
// }

// #[derive(Serialize, Deserialize)]
// struct Person {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let db = db::db::Db::connect().await?;

    #[cfg(debug_assertions)]
    logger::init()?;
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            // .service(person)
            .service(routes::api())
            .app_data(Data::new(db.clone()))
        //.route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();

    Ok(())
}
