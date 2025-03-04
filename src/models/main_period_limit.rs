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
pub struct MainPeriodLimit {
    /// Timestamp when the reward limit was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Unit of time for the reward limit's availability (e.g., day/week/month/year).
    #[serde(rename = "durationUnit", skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<String>,
    /// Number of days/weeks/month/year for reward limit
    #[serde(rename = "durationValue", skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i32>,
    /// Value of the limit
    #[serde(rename = "limitValue", skip_serializing_if = "Option::is_none")]
    pub limit_value: Option<i32>,
    /// Unique identifier for the reward limit
    #[serde(rename = "rewardLimitId", skip_serializing_if = "Option::is_none")]
    pub reward_limit_id: Option<String>,
    /// Select true to calculate all redeems/attributions from the previous value of selected durationUnit to the current time
    #[serde(rename = "slidingSchedule", skip_serializing_if = "Option::is_none")]
    pub sliding_schedule: Option<bool>,
    /// Type of reward
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Timestamp when the reward limit was created
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl MainPeriodLimit {
    pub fn new() -> MainPeriodLimit {
        MainPeriodLimit {
            created_at: None,
            duration_unit: None,
            duration_value: None,
            limit_value: None,
            reward_limit_id: None,
            sliding_schedule: None,
            r#type: None,
            updated_at: None,
        }
    }
}
