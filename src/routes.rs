use std::pin::Pin;

use actix_web::dev::{HttpServiceFactory, Payload};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{
    get, //modify here
    post,
    web::{Data, Json, Path}, //modify here
    FromRequest,
    HttpRequest,
    HttpResponse,
    Responder,
};
use futures::{future, Future, FutureExt};
use ulid::Ulid;

use crate::data::person::Person;
use crate::db::db::Db;
use crate::StdErr;

// #[actix_web::post("/persons")]
// async fn person(req_body: String) -> impl Responder {
//     let p = Person {
//         id: Ulid::new().to_string(),
//         name: "John".to_string(),
//         age: 30,
//     };
//     HttpResponse::Ok().body(serde_json::to_string(&p).unwrap())
// }

#[get("/person/{person_id}")]
async fn person(db: Data<crate::db::db::Db>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let p = db.persons(id.clone()).await.unwrap();
    print!("{:?}", id);
    print!("{:?}", p);
    HttpResponse::Ok().body(serde_json::to_string(&p).unwrap())
}

// async fn persons(
//     db: Data<Db>,
//     Path(person_id): Path<i64>,
//     _t: Person,
// ) -> Result<Json<Vec<Person>>, InternalError<StdErr>> {
//     db.persons(person_id)
//         .await
//         .map(Json)
//         .map_err(to_internal_error)
// }

fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/api").service(person)
}
