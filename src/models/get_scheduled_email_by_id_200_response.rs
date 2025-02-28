/*
 * Brevo API
 *
 * Brevo provide a RESTFul API that can be used with any languages. With this API, you will be able to :   - Manage your campaigns and get the statistics   - Manage your contacts   - Send transactional Emails and SMS   - and much more...  You can download our wrappers at https://github.com/orgs/brevo  **Possible responses**   | Code | Message |   | :-------------: | ------------- |   | 200  | OK. Successful Request  |   | 201  | OK. Successful Creation |   | 202  | OK. Request accepted |   | 204  | OK. Successful Update/Deletion  |   | 400  | Error. Bad Request  |   | 401  | Error. Authentication Needed  |   | 402  | Error. Not enough credit, plan upgrade needed  |   | 403  | Error. Permission denied  |   | 404  | Error. Object does not exist |   | 405  | Error. Method not allowed  |   | 406  | Error. Not Acceptable  |   | 422  | Error. Unprocessable Entity |
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: contact@brevo.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetScheduledEmailById200Response {
    GetScheduledEmailByBatchId(Box<models::GetScheduledEmailByBatchId>),
    GetScheduledEmailByMessageId(Box<models::GetScheduledEmailByMessageId>),
}

impl Default for GetScheduledEmailById200Response {
    fn default() -> Self {
        Self::GetScheduledEmailByBatchId(Default::default())
    }
}
/// Current status of the scheduled email
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "error")]
    Error,
}

impl Default for Status {
    fn default() -> Status {
        Self::InProgress
    }
}
