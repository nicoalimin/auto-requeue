use rocket::serde::{Serialize, Deserialize};
use std::fmt::{Debug};

/* Sample Payload */
// {
//     "eventType": "git.push",
//     ...
//     "messages": {
//         "text": "...",
//         "html": "...",
//         "markdown": "..."
//     },
//     "detailedMessage": {
//         "text": "...",
//         "html": "...",
//         "markdown": "..."
//     },
//     "resource": {
//         "id": "...",
//         "url": "https://...",
//         "name": "...",
//         "field1:": "..."
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineEvent {
    pub eventType: String,
    pub messages: Messages,
    #[allow(non_snake_case)]
    pub detailedMessage: Messages,
    pub resource: Resource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub text: String,
    pub html: String,
    pub markdown: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub url: String,
    pub name: String,
    // TODO add more relevant fields
    pub field1: Option<String>,
}