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
pub struct GetExtendedCampaignStats {
    /// Overall statistics of the campaign
    #[serde(rename = "globalStats")]
    pub global_stats: Box<models::GetCampaignStats>,
    /// List-wise statistics of the campaign.
    #[serde(rename = "campaignStats")]
    pub campaign_stats: Vec<models::GetCampaignStats>,
    /// Number of clicks on mirror link
    #[serde(rename = "mirrorClick")]
    pub mirror_click: i64,
    /// Number of remaning emails to send
    #[serde(rename = "remaining")]
    pub remaining: i64,
    /// Statistics about the number of clicks for the links
    #[serde(rename = "linksStats")]
    pub links_stats: serde_json::Value,
    #[serde(rename = "statsByDomain")]
    pub stats_by_domain: std::collections::HashMap<String, models::GetCampaignStats>,
    #[serde(rename = "statsByDevice")]
    pub stats_by_device: Box<models::GetStatsByDevice>,
    #[serde(rename = "statsByBrowser")]
    pub stats_by_browser: std::collections::HashMap<String, models::GetDeviceBrowserStats>,
}

impl GetExtendedCampaignStats {
    pub fn new(
        global_stats: models::GetCampaignStats,
        campaign_stats: Vec<models::GetCampaignStats>,
        mirror_click: i64,
        remaining: i64,
        links_stats: serde_json::Value,
        stats_by_domain: std::collections::HashMap<String, models::GetCampaignStats>,
        stats_by_device: models::GetStatsByDevice,
        stats_by_browser: std::collections::HashMap<String, models::GetDeviceBrowserStats>,
    ) -> GetExtendedCampaignStats {
        GetExtendedCampaignStats {
            global_stats: Box::new(global_stats),
            campaign_stats,
            mirror_click,
            remaining,
            links_stats,
            stats_by_domain,
            stats_by_device: Box::new(stats_by_device),
            stats_by_browser,
        }
    }
}
