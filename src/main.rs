#![allow(unused)] // silence unused warnings while exploring (to comment out)

mod asyncr;

extern crate redis;
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

#[derive(Error, Debug)]
pub enum DirectError {
    #[error("error parsing string from redis result: {0}")]
    RedisTypeError(redis::RedisError),
    #[error("error executing redis command: {0}")]
    RedisCMDError(redis::RedisError),
    #[error("error creating Redis client: {0}")]
    RedisClientError(redis::RedisError),
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey hey there!")
}

#[tokio::main]
async fn main() -> //Result<(), std::io::Error> {
    Result<(), Box<dyn Error>> {
    let cm = asyncr::get_connection_manager().await;
    let mut my_con = cm.clone();
    let _: () = redis::pipe()
        .atomic()
        .set("pylly", "kakka")
        .expire("pylly", 600)
        .query_async(&mut my_con)
        .await?;

    panic!("oh noes");

    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("Missing environment variable REDIS_HOSTNAME");
    let redis_tls = env::var("REDIS_TLS")
        .map(|redis_tls| redis_tls == "1")
        .unwrap_or(false);
    let uri_scheme = match redis_tls {
        true => "rediss",
        false => "redis",
    };

    let redis_conn_url = format!("{}://{}", uri_scheme, redis_host_name);
    let client = redis::Client::open(redis_conn_url).expect("Invalid connection URL");
    let redis_connection_manager = client
        .get_tokio_connection_manager()
        .await
        .expect("Can't create Redis connection manager");
    let mut con = redis_connection_manager.clone();
    con.del("kakka").await?;

    let _: () = redis::pipe()
        .atomic()
        .set("kakka", "pylly")
        .expire("kakka", 600)
        .query_async(&mut con)
        .await?;

    // 1) Create Connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_tokio_connection().await?;

    // 2) Set / Get Key
    con.set("my_key", "Hello world!").await?;
    let result: String = con.get("my_key").await?;
    println!("->> my_key: {}\n", result);

    // 3) xadd to redis stream
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-01"), ("title", "title 01")],
    )
    .await?;
    let len: i32 = con.xlen("my_stream").await?;
    println!("->> my_stream len {}\n", len);

    // 4) xrevrange the read stream
    let result: Option<StreamRangeReply> = con.xrevrange_count("my_stream", "+", "-", 10).await?;
    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("->> xrevrange stream entity: {}  ", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!("  ->> {}: {}", name, from_redis_value::<String>(value)?);
            }
            println!();
        }
    }

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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();

    Ok(())
}
