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
pub struct GetExtendedContactDetails {
    /// Email address of the contact for which you requested the details
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// ID of the contact for which you requested the details
    #[serde(rename = "id")]
    pub id: i64,
    /// Blacklist status for email campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "emailBlacklisted")]
    pub email_blacklisted: bool,
    /// Blacklist status for SMS campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "smsBlacklisted")]
    pub sms_blacklisted: bool,
    /// Creation UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Last modification UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "listIds")]
    pub list_ids: Vec<i64>,
    #[serde(rename = "listUnsubscribed", skip_serializing_if = "Option::is_none")]
    pub list_unsubscribed: Option<Vec<i64>>,
    /// Set of attributes of the contact
    #[serde(rename = "attributes")]
    pub attributes: serde_json::Value,
    #[serde(rename = "statistics")]
    pub statistics: Box<models::GetExtendedContactDetailsAllOfStatistics>,
}

impl GetExtendedContactDetails {
    pub fn new(
        id: i64,
        email_blacklisted: bool,
        sms_blacklisted: bool,
        created_at: String,
        modified_at: String,
        list_ids: Vec<i64>,
        attributes: serde_json::Value,
        statistics: models::GetExtendedContactDetailsAllOfStatistics,
    ) -> GetExtendedContactDetails {
        GetExtendedContactDetails {
            email: None,
            id,
            email_blacklisted,
            sms_blacklisted,
            created_at,
            modified_at,
            list_ids,
            list_unsubscribed: None,
            attributes,
            statistics: Box::new(statistics),
        }
    }
}
