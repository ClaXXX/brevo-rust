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

/// OrderIdentifiers : Identifies the contact associated with the order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderIdentifiers {
    /// ext_id associated with the order
    #[serde(rename = "ext_id", skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// loyalty_subscription_id associated with the order
    #[serde(
        rename = "loyalty_subscription_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Phone number of the contact associated with the order
    #[serde(rename = "phone_id", skip_serializing_if = "Option::is_none")]
    pub phone_id: Option<String>,
    /// Email of the contact associated with the order
    #[serde(rename = "email_id", skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
}

impl OrderIdentifiers {
    /// Identifies the contact associated with the order.
    pub fn new() -> OrderIdentifiers {
        OrderIdentifiers {
            ext_id: None,
            loyalty_subscription_id: None,
            phone_id: None,
            email_id: None,
        }
    }
}
