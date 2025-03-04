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
pub struct GetWhatsappCampaignOverview {
    /// ID of the WhatsApp Campaign
    #[serde(rename = "id")]
    pub id: i64,
    /// Name of the WhatsApp Campaign
    #[serde(rename = "campaignName")]
    pub campaign_name: String,
    /// Status of the WhatsApp Campaign
    #[serde(rename = "campaignStatus")]
    pub campaign_status: CampaignStatus,
    /// UTC date-time on which WhatsApp campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Sender of the WhatsApp Campaign
    #[serde(rename = "senderNumber")]
    pub sender_number: String,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<models::WhatsappCampStats>>,
    #[serde(rename = "template")]
    pub template: Box<models::WhatsappCampTemplate>,
    /// Creation UTC date-time of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// UTC date-time of last modification of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
}

impl GetWhatsappCampaignOverview {
    pub fn new(
        id: i64,
        campaign_name: String,
        campaign_status: CampaignStatus,
        sender_number: String,
        template: models::WhatsappCampTemplate,
        created_at: String,
        modified_at: String,
    ) -> GetWhatsappCampaignOverview {
        GetWhatsappCampaignOverview {
            id,
            campaign_name,
            campaign_status,
            scheduled_at: None,
            sender_number,
            stats: None,
            template: Box::new(template),
            created_at,
            modified_at,
        }
    }
}
/// Status of the WhatsApp Campaign
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CampaignStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "sent")]
    Sent,
}

impl Default for CampaignStatus {
    fn default() -> CampaignStatus {
        Self::Draft
    }
}
