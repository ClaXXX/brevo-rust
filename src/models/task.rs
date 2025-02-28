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

/// Task : Task Details
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    /// Unique task id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Id for type of task e.g Call / Email / Meeting etc.
    #[serde(rename = "taskTypeId")]
    pub task_type_id: String,
    /// Name of task
    #[serde(rename = "name")]
    pub name: String,
    /// Contact ids for contacts linked to this task
    #[serde(rename = "contactsIds", skip_serializing_if = "Option::is_none")]
    pub contacts_ids: Option<Vec<i32>>,
    /// Deal ids for deals a task is linked to
    #[serde(rename = "dealsIds", skip_serializing_if = "Option::is_none")]
    pub deals_ids: Option<Vec<String>>,
    /// Companies ids for companies a task is linked to
    #[serde(rename = "companiesIds", skip_serializing_if = "Option::is_none")]
    pub companies_ids: Option<Vec<String>>,
}

impl Task {
    /// Task Details
    pub fn new(task_type_id: String, name: String) -> Task {
        Task {
            id: None,
            task_type_id,
            name,
            contacts_ids: None,
            deals_ids: None,
            companies_ids: None,
        }
    }
}
