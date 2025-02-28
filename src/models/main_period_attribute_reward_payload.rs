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
pub struct MainPeriodAttributeRewardPayload {
    /// Value of the selected reward config
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Code generated to attribute reward to a contact
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Contact to attribute the reward
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Reward expiration date
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// One of contactId or loyaltySubscriptionId is required
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Offer meta information (key/value object)
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Reward id
    #[serde(rename = "rewardId")]
    pub reward_id: uuid::Uuid,
}

impl MainPeriodAttributeRewardPayload {
    pub fn new(reward_id: uuid::Uuid) -> MainPeriodAttributeRewardPayload {
        MainPeriodAttributeRewardPayload {
            value: None,
            code: None,
            contact_id: None,
            expiration_date: None,
            loyalty_subscription_id: None,
            meta: None,
            reward_id,
        }
    }
}
