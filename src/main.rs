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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    {
        let resp = req.send().await.unwrap();
        println!("Current DynamoDB tables: {:?}", resp.table_names);
    }

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
