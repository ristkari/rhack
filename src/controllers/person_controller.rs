use {
    actix_web::HttpResponse,
    actix_web::web::{Data, Json, Path},
    uuid::Uuid,

    crate::DBPool,
    crate::person::*,
    crate::util::{NotFoundMessage, ResponseType},
};


// List all Persons
#[get("/persons")]
pub async fn list_persons(pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let persons: Vec<Person> = fetch_all_persons(&conn);
    ResponseType::Ok(persons).get_response()
}

// Get a Person
#[get("/persons/{id}")]
pub async fn get_person(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let person: Option<Person> = fetch_person_by_id(
        Uuid::parse_str(path.0.0.as_str()).unwrap(), &conn);
    match person {
        Some(person) => ResponseType::Ok(person).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Person not found.".to_string())
        ).get_response(),
    }
}

// Create new Person
#[post("/wallets/{id}/persons")]
pub async fn create_person(path: Path<(String,)>, 
                        person_request: Json<NewPersonRequest>, 
                        pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    match create_new_person(person_request.0, Uuid::parse_str(path.0.0.as_str()).unwrap(), &conn) {
        Ok(created_person) => ResponseType::Created(created_person).get_response(),
        Err(_) => ResponseType::NotFound(
            NotFoundMessage::new("Error creating person.".to_string())
        ).get_response(),
    }
}