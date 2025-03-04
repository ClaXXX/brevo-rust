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
pub struct CreateTierGroupRequest {
    /// Name of the tier group
    #[serde(rename = "name")]
    pub name: String,
    /// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    #[serde(rename = "upgradeStrategy", skip_serializing_if = "Option::is_none")]
    pub upgrade_strategy: Option<UpgradeStrategy>,
    /// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    #[serde(rename = "downgradeStrategy", skip_serializing_if = "Option::is_none")]
    pub downgrade_strategy: Option<DowngradeStrategy>,
    /// Order of the tiers in the group in ascending order
    #[serde(rename = "tierOrder", skip_serializing_if = "Option::is_none")]
    pub tier_order: Option<Vec<String>>,
}

impl CreateTierGroupRequest {
    pub fn new(name: String) -> CreateTierGroupRequest {
        CreateTierGroupRequest {
            name,
            upgrade_strategy: None,
            downgrade_strategy: None,
            tier_order: None,
        }
    }
}
/// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UpgradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}

impl Default for UpgradeStrategy {
    fn default() -> UpgradeStrategy {
        Self::RealTime
    }
}
/// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DowngradeStrategy {
    #[serde(rename = "real_time")]
    RealTime,
    #[serde(rename = "membership_anniversary")]
    MembershipAnniversary,
    #[serde(rename = "tier_anniversary")]
    TierAnniversary,
}

impl Default for DowngradeStrategy {
    fn default() -> DowngradeStrategy {
        Self::RealTime
    }
}
