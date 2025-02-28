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
pub struct CreateEmailCampaign {
    /// Tag of the campaign
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "sender")]
    pub sender: Box<models::CreateEmailCampaignSender>,
    /// Name of the campaign
    #[serde(rename = "name")]
    pub name: String,
    /// Mandatory if htmlUrl and templateId are empty. Body of the message (HTML).
    #[serde(rename = "htmlContent", skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// **Mandatory if htmlContent and templateId are empty**. Url to the message (HTML). For example: **https://html.domain.com**
    #[serde(rename = "htmlUrl", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    /// **Mandatory if htmlContent and htmlUrl are empty**. Id of the transactional email template with status _active_. Used to copy only its content fetched from htmlContent/htmlUrl to an email campaign for RSS feature.
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result**. If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part). For example: **2017-06-01T12:30:00+02:00**
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Subject of the campaign. **Mandatory if abTesting is false**. Ignored if abTesting is true.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Preview text or preheader of the email campaign
    #[serde(rename = "previewText", skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// Email on which the campaign recipients will be able to reply to
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField", skip_serializing_if = "Option::is_none")]
    pub to_field: Option<String>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Box<models::CreateEmailCampaignRecipients>>,
    /// Absolute url of the attachment (no local file). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    #[serde(rename = "attachmentUrl", skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// Use true to embedded the images in your email. Final size of the email should be less than **4MB**. Campaigns with embedded images can _not be sent to more than 5000 contacts_
    #[serde(
        rename = "inlineImageActivation",
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_image_activation: Option<bool>,
    /// Use true to enable the mirror link
    #[serde(rename = "mirrorActive", skip_serializing_if = "Option::is_none")]
    pub mirror_active: Option<bool>,
    /// Footer of the email campaign
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// Header of the email campaign
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed
    #[serde(rename = "utmCampaign", skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    /// Pass the set of attributes to customize the type classic campaign. For example: **{\"FNAME\":\"Joe\", \"LNAME\":\"Doe\"}**. Only available if **type** is **classic**. It's considered only if campaign is in _New Template Language format_. The New Template Language is dependent on the values of **subject, htmlContent/htmlUrl, sender.name & toField**
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Set this to true if you want to send your campaign at best time.
    #[serde(rename = "sendAtBestTime", skip_serializing_if = "Option::is_none")]
    pub send_at_best_time: Option<bool>,
    /// Status of A/B Test. abTesting = false means it is disabled & abTesting = true means it is enabled. **subjectA, subjectB, splitRule, winnerCriteria & winnerDelay** will be considered when abTesting is set to true. subjectA & subjectB are mandatory together & subject if passed is ignored. **Can be set to true only if sendAtBestTime is false**. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B
    #[serde(rename = "abTesting", skip_serializing_if = "Option::is_none")]
    pub ab_testing: Option<bool>,
    /// Subject A of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectA", skip_serializing_if = "Option::is_none")]
    pub subject_a: Option<String>,
    /// Subject B of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectB", skip_serializing_if = "Option::is_none")]
    pub subject_b: Option<String>,
    /// Add the size of your test groups. **Mandatory if abTesting = true & 'recipients' is passed**. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else
    #[serde(rename = "splitRule", skip_serializing_if = "Option::is_none")]
    pub split_rule: Option<i64>,
    /// Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
    #[serde(rename = "winnerCriteria", skip_serializing_if = "Option::is_none")]
    pub winner_criteria: Option<WinnerCriteria>,
    /// Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerDelay` is ignored if passed
    #[serde(rename = "winnerDelay", skip_serializing_if = "Option::is_none")]
    pub winner_delay: Option<i64>,
    /// **Available for dedicated ip clients**. Set this to true if you wish to warm up your ip.
    #[serde(rename = "ipWarmupEnable", skip_serializing_if = "Option::is_none")]
    pub ip_warmup_enable: Option<bool>,
    /// **Mandatory if ipWarmupEnable is set to true**. Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000.
    #[serde(rename = "initialQuota", skip_serializing_if = "Option::is_none")]
    pub initial_quota: Option<i64>,
    /// **Mandatory if ipWarmupEnable is set to true**. Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%.
    #[serde(rename = "increaseRate", skip_serializing_if = "Option::is_none")]
    pub increase_rate: Option<i64>,
    /// Enter an unsubscription page id. The page id is a 24 digit alphanumeric id that can be found in the URL when editing the page. If not entered, then the default unsubscription page will be used.
    #[serde(
        rename = "unsubscriptionPageId",
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscription_page_id: Option<String>,
    /// **Mandatory if templateId is used containing the {{ update_profile }} tag**. Enter an update profile form id. The form id is a 24 digit alphanumeric id that can be found in the URL when editing the form. If not entered, then the default update profile form will be used.
    #[serde(rename = "updateFormId", skip_serializing_if = "Option::is_none")]
    pub update_form_id: Option<String>,
    #[serde(
        rename = "emailExpirationDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub email_expiration_date: Option<Box<models::CreateEmailCampaignEmailExpirationDate>>,
}

impl CreateEmailCampaign {
    pub fn new(sender: models::CreateEmailCampaignSender, name: String) -> CreateEmailCampaign {
        CreateEmailCampaign {
            tag: None,
            sender: Box::new(sender),
            name,
            html_content: None,
            html_url: None,
            template_id: None,
            scheduled_at: None,
            subject: None,
            preview_text: None,
            reply_to: None,
            to_field: None,
            recipients: None,
            attachment_url: None,
            inline_image_activation: None,
            mirror_active: None,
            footer: None,
            header: None,
            utm_campaign: None,
            params: None,
            send_at_best_time: None,
            ab_testing: None,
            subject_a: None,
            subject_b: None,
            split_rule: None,
            winner_criteria: None,
            winner_delay: None,
            ip_warmup_enable: None,
            initial_quota: None,
            increase_rate: None,
            unsubscription_page_id: None,
            update_form_id: None,
            email_expiration_date: None,
        }
    }
}
/// Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WinnerCriteria {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "click")]
    Click,
}

impl Default for WinnerCriteria {
    fn default() -> WinnerCriteria {
        Self::Open
    }
}
