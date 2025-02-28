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
pub struct MainPeriodValidateRewardPayload {
    /// Unique identifier for the attributed reward
    #[serde(rename = "attributedRewardId", skip_serializing_if = "Option::is_none")]
    pub attributed_reward_id: Option<uuid::Uuid>,
    /// Validation code for the reward
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Unique identifier for the contact
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Identifier for the loyalty subscription
    #[serde(
        rename = "loyaltySubscriptionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub loyalty_subscription_id: Option<String>,
    /// Identifier for the point of sale
    #[serde(rename = "pointOfSellId", skip_serializing_if = "Option::is_none")]
    pub point_of_sell_id: Option<String>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<uuid::Uuid>,
}

impl MainPeriodValidateRewardPayload {
    pub fn new() -> MainPeriodValidateRewardPayload {
        MainPeriodValidateRewardPayload {
            attributed_reward_id: None,
            code: None,
            contact_id: None,
            loyalty_subscription_id: None,
            point_of_sell_id: None,
            reward_id: None,
        }
    }
}
