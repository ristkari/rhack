extern crate redis;
use redis::AsyncCommands;
use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use aws_sdk_dynamodb::Client;

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

async fn connect() -> Option<redis::Connection> {
    //format - host:port
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");

    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default(); //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    let client = redis::Client::open(redis_conn_url).unwrap();
    client.get_async_connection().await?

    //.expect("failed to connect to Redis");
    //let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    //let mut con = client.get_async_connection().await?;
}

async fn basics() {
    let mut conn = connect();
    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(&mut conn)
        .expect("failed to execute SET for 'foo'");
    let bar: String = redis::cmd("GET")
        .arg("foo")
        .query(&mut conn)
        .expect("failed to execute GET for 'foo'");
    println!("value for 'foo' = {}", bar);
    let _: () = conn
        .incr("counter", 2)
        .expect("failed to execute INCR for 'counter'");
    let val: i32 = conn
        .get("counter")
        .expect("failed to execute GET for 'counter'");
    println!("counter = {}", val);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let redis_connection = connect().await;
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
