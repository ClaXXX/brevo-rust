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
pub struct Tier {
    /// Tier id
    #[serde(rename = "tierId", skip_serializing_if = "Option::is_none")]
    pub tier_id: Option<uuid::Uuid>,
    /// Tier name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tier image reference
    #[serde(rename = "imageRef", skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Associated loyalty program Id
    #[serde(rename = "loyaltyProgramId", skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<uuid::Uuid>,
    /// Associated group Id
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Conditions required to access this tier
    #[serde(rename = "accessConditions", skip_serializing_if = "Option::is_none")]
    pub access_conditions: Option<Vec<models::TierAccessConditionsInner>>,
    /// Rewards associated with this tier
    #[serde(rename = "tierRewards", skip_serializing_if = "Option::is_none")]
    pub tier_rewards: Option<Vec<models::TierTierRewardsInner>>,
}

impl Tier {
    pub fn new() -> Tier {
        Tier {
            tier_id: None,
            name: None,
            image_ref: None,
            loyalty_program_id: None,
            group_id: None,
            created_at: None,
            updated_at: None,
            access_conditions: None,
            tier_rewards: None,
        }
    }
}
