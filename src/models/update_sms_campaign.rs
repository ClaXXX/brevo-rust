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
pub struct UpdateSmsCampaign {
    /// Name of the campaign
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters**
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// Content of the message. The **maximum characters used per SMS is 160**, if used more than that, it will be counted as more than one SMS
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Box<models::CreateSmsCampaignRecipients>>,
    /// UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Format of the message. It indicates whether the content should be treated as unicode or not.
    #[serde(rename = "unicodeEnabled", skip_serializing_if = "Option::is_none")]
    pub unicode_enabled: Option<bool>,
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    #[serde(rename = "organisationPrefix", skip_serializing_if = "Option::is_none")]
    pub organisation_prefix: Option<String>,
    /// Instructions to unsubscribe from future communications. Recommended by U.S. carriers. Must include **STOP** keyword. This will be added as instructions after the end of message content. **Prefer verifying maximum length of 160 characters including this instructions in message content to avoid multiple sending of same sms.**
    #[serde(
        rename = "unsubscribeInstruction",
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_instruction: Option<String>,
}

impl UpdateSmsCampaign {
    pub fn new() -> UpdateSmsCampaign {
        UpdateSmsCampaign {
            name: None,
            sender: None,
            content: None,
            recipients: None,
            scheduled_at: None,
            unicode_enabled: None,
            organisation_prefix: None,
            unsubscribe_instruction: None,
        }
    }
}
