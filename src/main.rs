use actix_web::{get, web, App, HttpServer};
// use serde::{Deserialize,Serialize};
// use std::sync::Mutex;
mod lookup;
mod structs;
mod service;

#[get("/")]
async fn index()->String{
    return "Smoke check".to_string();
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    let _ = HttpServer::new(move||{
        App::new().
        service(index).
        service(lookup::gas_lookup)
    }).bind(("0.0.0.0",8080))?.
    run().
    await;


    Ok(())
}


