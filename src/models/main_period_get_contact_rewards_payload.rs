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
pub struct MainPeriodGetContactRewardsPayload {
    /// Contact to attribute the reward
    #[serde(rename = "contactId")]
    pub contact_id: i32,
    /// Number of documents per page
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Data to define the reward for that particular contact
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<models::MainPeriodFilter>>,
    /// Index of the first document in the page
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Unique identifier of the associated reward
    #[serde(rename = "rewardId", skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    /// Sort the documents in ascending or descending order
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
    /// Sort documents by field
    #[serde(rename = "sortField", skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<SortField>,
}

impl MainPeriodGetContactRewardsPayload {
    pub fn new(contact_id: i32) -> MainPeriodGetContactRewardsPayload {
        MainPeriodGetContactRewardsPayload {
            contact_id,
            limit: None,
            metadata: None,
            offset: None,
            reward_id: None,
            sort: None,
            sort_field: None,
        }
    }
}
/// Sort the documents in ascending or descending order
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Sort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for Sort {
    fn default() -> Sort {
        Self::Asc
    }
}
/// Sort documents by field
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortField {
    #[serde(rename = "updatedAt")]
    UpdatedAt,
    #[serde(rename = "createdAt")]
    CreatedAt,
}

impl Default for SortField {
    fn default() -> SortField {
        Self::UpdatedAt
    }
}
