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
pub struct Event {
    /// The name of the event that occurred. This is how you will find your event in Brevo. Limited to 255 characters, alphanumerical characters and - _ only.
    #[serde(rename = "event_name")]
    pub event_name: String,
    /// Timestamp of when the event occurred (e.g. \"2024-01-24T17:39:57+01:00\"). If no value is passed, the timestamp of the event creation is used.
    #[serde(rename = "event_date", skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "identifiers")]
    pub identifiers: Box<models::EventIdentifiers>,
    /// Properties defining the state of the contact associated to this event. Useful to update contact attributes defined in your contacts database while passing the event. For example: **\"FIRSTNAME\": \"Jane\" , \"AGE\": 37**
    #[serde(rename = "contact_properties", skip_serializing_if = "Option::is_none")]
    pub contact_properties:
        Option<std::collections::HashMap<String, models::EventContactPropertiesValue>>,
    /// Properties of the event. Top level properties and nested properties can be used to better segment contacts and personalise workflow conditions. The following field type are supported: string, number, boolean (true/false), date (Timestamp e.g. \"2024-01-24T17:39:57+01:00\"). Keys are limited to 255 characters, alphanumerical characters and - _ only. Size is limited to 50Kb.
    #[serde(rename = "event_properties", skip_serializing_if = "Option::is_none")]
    pub event_properties:
        Option<std::collections::HashMap<String, models::EventEventPropertiesValue>>,
}

impl Event {
    pub fn new(event_name: String, identifiers: models::EventIdentifiers) -> Event {
        Event {
            event_name,
            event_date: None,
            identifiers: Box::new(identifiers),
            contact_properties: None,
            event_properties: None,
        }
    }
}
