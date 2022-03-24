use rocket::serde::{json::Json};

#[path = "../types/pipeline_event.rs"]
mod pipeline_event;

type PipelineEvent = pipeline_event::PipelineEvent;

// Routes should be defined here

#[get("/health")]
pub fn handle_health() -> String {
    return format!("Healthy");
}

#[post("/pipeline_event", format="application/json", data="<pipeline_event>")]
pub fn handle_pipeline_event(pipeline_event: Json<PipelineEvent>) -> Json<PipelineEvent> {
    println!("{:#?}", pipeline_event);
    return pipeline_event;
}