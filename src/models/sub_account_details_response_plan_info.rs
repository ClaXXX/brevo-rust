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

/// SubAccountDetailsResponsePlanInfo : Sub-account plan details
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubAccountDetailsResponsePlanInfo {
    #[serde(rename = "credits", skip_serializing_if = "Option::is_none")]
    pub credits: Option<Box<models::SubAccountDetailsResponsePlanInfoCredits>>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Box<models::SubAccountDetailsResponsePlanInfoFeatures>>,
    /// type of the plan
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
}

impl SubAccountDetailsResponsePlanInfo {
    /// Sub-account plan details
    pub fn new() -> SubAccountDetailsResponsePlanInfo {
        SubAccountDetailsResponsePlanInfo {
            credits: None,
            features: None,
            plan_type: None,
        }
    }
}
