use std::result::Result;
use reqwest::blocking::{Client, Response}; // TODO use non-blocking request
use reqwest::Error;
use base64::encode;

mod types;

type Pipeline = types::Pipeline;
type Run = types::Run;
type RunRequest = types::RunRequest;
type RunRequestResources = types::RunRequestResources;


fn get_auth_token() -> String {
    let username = "visionado@microsoft.com";
    let pat = "my_pat";

    // base64 encode auth token of "username:pat"
    let auth_token = format!(
        "{}:{}",
        encode(username),
        encode(pat)
    ); 

    return auth_token;
}

/**
 * Returns a 
 */
pub fn get_all_pipelines(organization: String, project: String) -> Result<Vec<Pipeline>, Error> {
    let url: String = format!(
        "https://dev.azure.com/{organization}/{project}/_apis/pipelines", // ?orderBy={orderBy}&$top={$top}&continuationToken={continuationToken}&api-version=6.0-preview.1",
        organization=organization,
        project=project
    );

    // base64 encode auth token of "username:pat"
    let auth_token = get_auth_token();

    println!("Url: {}", url);
    println!("Auth token: {}", auth_token);

    let response: Response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", auth_token)) // Bearer token auth
        .send()?;
    

    let pipeline: Vec<Pipeline> = response.json()?;
    

    println!("{:#?}", pipeline);

    return Ok(pipeline);
}

pub fn get_pipeline_runs(organization: String, project: String, pipeline_id: String) -> Result<Vec<Run>, Error> {
    let url: String = format!(
        "https://dev.azure.com/{organization}/{project}/_apis/pipelines/{pipeline_id}/runs?api-version=6.0-preview.1",
        organization=organization,
        project=project,
        pipeline_id=pipeline_id,
    );

    // base64 encode auth token of "username:pat"
    let auth_token = get_auth_token();

    println!("Url: {}", url);
    println!("Auth token: {}", auth_token);

    let response: Response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", auth_token)) // Bearer token auth
        .send()?;
    

    let runs: Vec<Run> = response.json()?;
    

    println!("{:#?}", runs);

    return Ok(runs);
}

pub fn get_pipeline_run_details(organization: String, project: String, pipeline_id: String, run_id: String) -> Result<Run, Error> {
    let url: String = format!(
        "https://dev.azure.com/{organization}/{project}/_apis/pipelines/{pipeline_id}/runs/{run_id}?api-version=6.0-preview.1",
        organization=organization,
        project=project,
        pipeline_id=pipeline_id,
        run_id=run_id,
    );

    // base64 encode auth token of "username:pat"
    let auth_token = get_auth_token();

    println!("Url: {}", url);
    println!("Auth token: {}", auth_token);

    let response: Response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", auth_token)) // Bearer token auth
        .send()?;
    

    let run: Run = response.json()?;
    

    println!("{:#?}", run);

    return Ok(run);
}

pub fn run_pipeline(organization: String, project: String, pipeline_id: String) -> Result<Run, Error> {
    let url: String = format!(
        "https://dev.azure.com/{organization}/{project}/_apis/pipelines/{pipeline_id}/runs?api-version=6.0-preview.1",
        organization=organization,
        project=project,
        pipeline_id=pipeline_id,
    );

    // base64 encode auth token of "username:pat"
    let auth_token = get_auth_token();

    println!("Url: {}", url);
    println!("Auth token: {}", auth_token);

    let run_request_resources = RunRequestResources {
        builds: None,
        containers: None,
        packages: None,
        pipelines: None,
        repositories: None,
    };

    let body: RunRequest = RunRequest {
        previewRun: false,
        resources: run_request_resources,
        stagesToSkip: None,
        templateParameters: None,
        variables: None,
        yamlOverride: None,
    };

    let response: Response = Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {}", auth_token)) // Bearer token auth
        .json(&body)
        .send()?;
    

    let run: Run = response.json()?;
    

    println!("{:#?}", run);

    return Ok(run);
}