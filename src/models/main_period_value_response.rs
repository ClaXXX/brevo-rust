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
pub struct MainPeriodValueResponse {
    /// Array values to define the rule
    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<models::MainPeriodValueResponse>>,
    /// Boolean values for rule definition
    #[serde(rename = "boolean", skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    /// string
    #[serde(rename = "contactProperty", skip_serializing_if = "Option::is_none")]
    pub contact_property: Option<String>,
    /// Selected date for rule definition
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Selected event property for rule definition
    #[serde(rename = "eventProperty", skip_serializing_if = "Option::is_none")]
    pub event_property: Option<String>,
    /// Created expression for rule definition
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<models::MainPeriodNodeResponse>>,
    /// Boolean values for rule definition
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    /// String values for rule definition
    #[serde(rename = "string", skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

impl MainPeriodValueResponse {
    pub fn new() -> MainPeriodValueResponse {
        MainPeriodValueResponse {
            array: None,
            boolean: None,
            contact_property: None,
            date: None,
            event_property: None,
            expression: None,
            number: None,
            string: None,
        }
    }
}
