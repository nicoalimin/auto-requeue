#[macro_use] extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![
        routes::handle_health,
        routes::handle_pipeline_event,
    ])
}

// TODO use these somewhere
fn example_query_pipelines() {
    # [path="query_pipeline/mod.rs"]
    mod query_pipeline;

    // get all pipelines
    query_pipeline::get_all_pipelines(String::from("organizationID"), String::from("projectID"));

    // get runs for a pipeline
    query_pipeline::get_pipeline_runs(String::from("organizationID"), String::from("projectID"), String::from("pipelineID"));

    // get run details
    query_pipeline::get_pipeline_run_details(String::from("organizationID"), String::from("projectID"), String::from("pipelineID"), String::from("runID"));

    // start a pipeline
    query_pipeline::run_pipeline(String::from("organizationID"), String::from("projectID"), String::from("pipelineID"));
}