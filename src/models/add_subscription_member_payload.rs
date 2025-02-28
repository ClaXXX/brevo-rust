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
pub struct AddSubscriptionMemberPayload {
    /// Required if LoyaltySubscriptionId is not provided, must be greater than 0
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i32>,
    /// Required if ContactId is not provided, max length 64
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Required, each item must be greater than or equal to 1
    #[serde(rename = "memberContactIds")]
    pub member_contact_ids: Vec<i32>,
}

impl AddSubscriptionMemberPayload {
    pub fn new(member_contact_ids: Vec<i32>) -> AddSubscriptionMemberPayload {
        AddSubscriptionMemberPayload {
            contact_id: None,
            loyalty_subscription_id: None,
            member_contact_ids,
        }
    }
}
