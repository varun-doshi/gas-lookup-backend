use actix_web::{post, web::Json, HttpResponse};
use crate::structs;
use crate::service;


#[post("/lookup")]
pub async fn gas_lookup(user_data:Json<structs::UserInput>)->HttpResponse{

    let result=service::start_service(user_data.clone()).await;

    HttpResponse::Ok()
    .content_type("application/json")
    .json(result)
}