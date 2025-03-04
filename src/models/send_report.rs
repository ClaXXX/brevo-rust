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
pub struct SendReport {
    /// Language of email content for campaign report sending.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(rename = "email")]
    pub email: Box<models::SendReportEmail>,
}

impl SendReport {
    pub fn new(email: models::SendReportEmail) -> SendReport {
        SendReport {
            language: None,
            email: Box::new(email),
        }
    }
}
/// Language of email content for campaign report sending.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
}

impl Default for Language {
    fn default() -> Language {
        Self::Fr
    }
}
