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
pub struct MainPeriodRule {
    /// Selected rule condition
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Box<models::MainPeriodRuleConditionResponse>>,
    /// Timestamp when the rule was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Description of the rule
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Selected event in the rule
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<models::MainPeriodRuleEventResponse>>,
    /// Metric to identify if it's an internal rule or not
    #[serde(rename = "isInternal", skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    /// Loyalty Program id to which current rule is associated
    #[serde(rename = "loyaltyProgramId", skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Loyalty Version id to which current rule is associated
    #[serde(rename = "loyaltyVersionId", skip_serializing_if = "Option::is_none")]
    pub loyalty_version_id: Option<i32>,
    /// Additional data to define the rule
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Rule name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Results of the rule definition
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::MainPeriodRuleResultResponse>>,
    /// Unique identifier for the rule
    #[serde(rename = "ruleId", skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// Type of the rule
    #[serde(rename = "ruleType", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// Timestamp when the rule was updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl MainPeriodRule {
    pub fn new() -> MainPeriodRule {
        MainPeriodRule {
            condition: None,
            created_at: None,
            description: None,
            event: None,
            is_internal: None,
            loyalty_program_id: None,
            loyalty_version_id: None,
            meta: None,
            name: None,
            results: None,
            rule_id: None,
            rule_type: None,
            updated_at: None,
        }
    }
}
