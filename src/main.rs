use actix_web::{get, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;
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
        let cors = Cors::permissive()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allow_any_header();


        App::new().wrap(cors).
        service(index).
        service(lookup::gas_lookup)
    }).bind(("0.0.0.0",8080))?.
    run().
    await;


    Ok(())
}


