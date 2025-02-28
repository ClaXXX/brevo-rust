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
pub struct PostSendSmsTestFailed {
    /// Response code
    #[serde(rename = "code")]
    pub code: String,
    /// Response message
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "unexistingSms", skip_serializing_if = "Option::is_none")]
    pub unexisting_sms: Option<Vec<String>>,
    #[serde(rename = "withoutListSms", skip_serializing_if = "Option::is_none")]
    pub without_list_sms: Option<Vec<String>>,
}

impl PostSendSmsTestFailed {
    pub fn new(code: String, message: String) -> PostSendSmsTestFailed {
        PostSendSmsTestFailed {
            code,
            message,
            unexisting_sms: None,
            without_list_sms: None,
        }
    }
}
