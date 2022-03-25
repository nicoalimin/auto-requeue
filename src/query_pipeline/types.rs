use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Pipeline {
    // TODO implement these two if they become important in the future
    pub _links: Option<String>,
    pub configuration: Option<String>,

    pub folder: String,
    pub id: String,
    pub name: String,
    pub revision: i32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Run {
    // TODO implement these two if they become important in the future
    pub _links: Option<String>,
    pub variables: Option<String>,

    pub createdDate: String,
    pub finalYaml: String,
    pub finishedDate: String,
    pub id: String,
    pub name: String,
    pub pipeline: Pipeline,
    pub resources: Option<Vec<String>>,
    pub result: String, // RunResult
    pub state: String, // TODO come up with type for this
    pub url: String,
}

// pub enum RunResult {
//     Cancelled,
//     Failed,
//     Succeeded,
//     Unknown,
// }

// impl RunResult {
//     fn to_string(&self) -> &str {
//         match self {
//             RunResult::Cancelled => "cancelled",
//             RunResult::Failed => "failed",
//             RunResult::Succeeded => "succeeded",
//             RunResult::Unknown => "unknown",
//         }
//     }

//     fn from_string(s: &str) -> RunResult {
//         match s {
//             "cancelled" => RunResult::Cancelled,
//             "failed" => RunResult::Failed,
//             "succeeded" => RunResult::Succeeded,
//             _ => RunResult::Unknown
//         }
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct RunRequest {
    pub previewRun: bool,
    pub resources: RunRequestResources,
    pub stagesToSkip: Option<Vec<String>>,
    pub templateParameters: Option<HashMap<String, String>>,
    pub variables: Option<String>,
    pub yamlOverride: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunRequestResources {
    pub builds: Option<String>,
    pub containers: Option<String>,
    pub packages: Option<String>,
    pub pipelines: Option<String>,
    pub repositories: Option<String>,
}