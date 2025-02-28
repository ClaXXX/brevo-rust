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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFolder {
    /// ID of the folder
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the folder
    #[serde(rename = "name")]
    pub name: String,
    /// Number of blacklisted contacts in the folder
    #[serde(rename = "totalBlacklisted")]
    pub total_blacklisted: i64,
    /// Number of contacts in the folder
    #[serde(rename = "totalSubscribers")]
    pub total_subscribers: i64,
    /// Number of unique contacts in the folder
    #[serde(rename = "uniqueSubscribers")]
    pub unique_subscribers: i64,
}

impl GetFolder {
    pub fn new(
        id: i64,
        name: String,
        total_blacklisted: i64,
        total_subscribers: i64,
        unique_subscribers: i64,
    ) -> GetFolder {
        GetFolder {
            id,
            name,
            total_blacklisted,
            total_subscribers,
            unique_subscribers,
        }
    }
}
