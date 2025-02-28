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
pub struct GetAggregatedReport {
    /// Time frame of the report
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// Number of requests for the timeframe
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    /// Number of delivered emails for the timeframe
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// Number of hardbounces for the timeframe
    #[serde(rename = "hardBounces", skip_serializing_if = "Option::is_none")]
    pub hard_bounces: Option<i64>,
    /// Number of softbounces for the timeframe
    #[serde(rename = "softBounces", skip_serializing_if = "Option::is_none")]
    pub soft_bounces: Option<i64>,
    /// Number of clicks for the timeframe
    #[serde(rename = "clicks", skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// Number of unique clicks for the timeframe
    #[serde(rename = "uniqueClicks", skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i64>,
    /// Number of openings for the timeframe
    #[serde(rename = "opens", skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    /// Number of unique openings for the timeframe
    #[serde(rename = "uniqueOpens", skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
    /// Number of complaint (spam report) for the timeframe
    #[serde(rename = "spamReports", skip_serializing_if = "Option::is_none")]
    pub spam_reports: Option<i64>,
    /// Number of blocked contact emails for the timeframe
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<i64>,
    /// Number of invalid emails for the timeframe
    #[serde(rename = "invalid", skip_serializing_if = "Option::is_none")]
    pub invalid: Option<i64>,
    /// Number of unsubscribed emails for the timeframe
    #[serde(rename = "unsubscribed", skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
}

impl GetAggregatedReport {
    pub fn new() -> GetAggregatedReport {
        GetAggregatedReport {
            range: None,
            requests: None,
            delivered: None,
            hard_bounces: None,
            soft_bounces: None,
            clicks: None,
            unique_clicks: None,
            opens: None,
            unique_opens: None,
            spam_reports: None,
            blocked: None,
            invalid: None,
            unsubscribed: None,
        }
    }
}
