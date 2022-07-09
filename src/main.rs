#![allow(unused)] // silence unused warnings while exploring (to comment out)

mod redis_datastore;

extern crate redis;
use redis_datastore::connection_manager;
use serde::{Deserialize, Serialize};
use serde_json::Result as sj;
use std::{error::Error, time::Duration};
use tokio::time::sleep;

use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};

use redis::aio::ConnectionManager;
use std::convert::Infallible;
use std::env;
use thiserror::Error;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//use aws_sdk_dynamodb::Client;

const REDIS_CON_STRING: &str = "redis://127.0.0.1/";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/person")]
async fn person(req_body: String) -> impl Responder {
    let p = Person {
        name: "John".to_string(),
        age: 30,
        phones: vec!["123-456-7890".to_string(), "123-456-7891".to_string()],
    };
    HttpResponse::Ok().body(serde_json::to_string(&p).unwrap())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey hey there!")
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let cm = redis_datastore::connection_manager::get_connection_manager().await;
    let mut my_con = match redis_datastore::connection_manager::get_connection_manager().await {
        Ok(res) => {
            println!("{:?}", "Got Connection Manager");
            res.clone()
        }
        Err(e) => {
            //println!("{:?}", e);
            println!("{}", e);
            panic!("oh noes");
        }
    };

    let p = Person {
        name: "John".to_string(),
        age: 30,
        phones: vec!["123-456-7890".to_string(), "123-456-7891".to_string()],
    };

    let _: () = redis::pipe()
        .atomic()
        .set("pylly", serde_json::to_string(&p).unwrap())
        .expire("pylly", 600)
        .query_async(&mut my_con)
        .await?;

    /*
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    {
        let resp = req.send().await.unwrap();
        println!("Current DynamoDB tables: {:?}", resp.table_names);
    }
    */

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(person)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();

    Ok(())
}
