#[macro_use] extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![
        routes::handle_health,
        routes::handle_pipeline_event,
    ])
}