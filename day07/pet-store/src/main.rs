mod common;
mod pet;
use rocket::{get, launch, routes, fairing::AdHoc, http::Header};
use common::RequestId;
use rocket_db_pools::{Database};
use common::DbClient;



#[get("/")]
fn index(request_id: RequestId) -> &'static str {
    log::debug!("request_id is {}", &request_id.0);
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    log::debug!("Logger init with debug level");
    rocket::build()
    .attach(DbClient::init())
    .mount("/", routes![index])
    .attach(AdHoc::on_response("request_id", |req, res| Box::pin(async move {
        let request_id = req.guard::<RequestId>().await.unwrap().0;
        res.set_header(Header::new("x-request-id", request_id.clone()));
    })))
    .attach(AdHoc::on_response("CORS", |_, res| Box::pin(async move {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "HEAD, GET, POST, PUT, PATCH, DELETE"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        res.set_header(Header::new("Access-Control-Max-Age", "86400"));
        res.set_header(Header::new("Access-Control-Expose-Headers", "x-request-id"));
    })))
    .attach(pet::stage())
}