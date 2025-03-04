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

/// RequestContactExportCustomContactFilter : Set the filter for the contacts to be exported.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestContactExportCustomContactFilter {
    /// **Mandatory if neither actionForEmailCampaigns nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on contacts as per the list id. * **allContacts** - Fetch the list of all contacts for a particular list. * **subscribed & unsubscribed** - Fetch the list of subscribed / unsubscribed (blacklisted via any means) contacts for a particular list. * **unsubscribedPerList** - Fetch the list of contacts that are unsubscribed from a particular list only.
    #[serde(rename = "actionForContacts", skip_serializing_if = "Option::is_none")]
    pub action_for_contacts: Option<ActionForContacts>,
    /// **Mandatory if neither actionForContacts nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on email campaigns. * **openers & nonOpeners** - emailCampaignId is mandatory. Fetch the list of readers / non-readers for a particular email campaign. * **clickers & nonClickers** - emailCampaignId is mandatory. Fetch the list of clickers / non-clickers for a particular email campaign. * **unsubscribed** - emailCampaignId is mandatory. Fetch the list of all unsubscribed (blacklisted via any means) contacts for a particular email campaign. * **hardBounces & softBounces** - emailCampaignId is optional. Fetch the list of hard bounces / soft bounces for a particular / all email campaign(s).
    #[serde(
        rename = "actionForEmailCampaigns",
        skip_serializing_if = "Option::is_none"
    )]
    pub action_for_email_campaigns: Option<ActionForEmailCampaigns>,
    /// **Mandatory if neither actionForContacts nor actionForEmailCampaigns is passed.** This will export the contacts on the basis of provided action applied on sms campaigns. * **unsubscribed** - Fetch the list of all unsubscribed (blacklisted via any means) contacts for all / particular sms campaigns. * **hardBounces & softBounces** - Fetch the list of hard bounces / soft bounces for all / particular sms campaigns.
    #[serde(
        rename = "actionForSmsCampaigns",
        skip_serializing_if = "Option::is_none"
    )]
    pub action_for_sms_campaigns: Option<ActionForSmsCampaigns>,
    /// **Mandatory if actionForContacts is passed, ignored otherwise.** Id of the list for which the corresponding action shall be applied in the filter.
    #[serde(rename = "listId", skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i64>,
    /// Considered only if **actionForEmailCampaigns** is passed, ignored otherwise. **Mandatory if action is one of the following - openers, nonOpeners, clickers, nonClickers, unsubscribed.** The id of the email campaign for which the corresponding action shall be applied in the filter.
    #[serde(rename = "emailCampaignId", skip_serializing_if = "Option::is_none")]
    pub email_campaign_id: Option<i64>,
    /// Considered only if **actionForSmsCampaigns** is passed, ignored otherwise. The id of sms campaign for which the corresponding action shall be applied in the filter.
    #[serde(rename = "smsCampaignId", skip_serializing_if = "Option::is_none")]
    pub sms_campaign_id: Option<i64>,
}

impl RequestContactExportCustomContactFilter {
    /// Set the filter for the contacts to be exported.
    pub fn new() -> RequestContactExportCustomContactFilter {
        RequestContactExportCustomContactFilter {
            action_for_contacts: None,
            action_for_email_campaigns: None,
            action_for_sms_campaigns: None,
            list_id: None,
            email_campaign_id: None,
            sms_campaign_id: None,
        }
    }
}
/// **Mandatory if neither actionForEmailCampaigns nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on contacts as per the list id. * **allContacts** - Fetch the list of all contacts for a particular list. * **subscribed & unsubscribed** - Fetch the list of subscribed / unsubscribed (blacklisted via any means) contacts for a particular list. * **unsubscribedPerList** - Fetch the list of contacts that are unsubscribed from a particular list only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionForContacts {
    #[serde(rename = "allContacts")]
    AllContacts,
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "unsubscribedPerList")]
    UnsubscribedPerList,
}

impl Default for ActionForContacts {
    fn default() -> ActionForContacts {
        Self::AllContacts
    }
}
/// **Mandatory if neither actionForContacts nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on email campaigns. * **openers & nonOpeners** - emailCampaignId is mandatory. Fetch the list of readers / non-readers for a particular email campaign. * **clickers & nonClickers** - emailCampaignId is mandatory. Fetch the list of clickers / non-clickers for a particular email campaign. * **unsubscribed** - emailCampaignId is mandatory. Fetch the list of all unsubscribed (blacklisted via any means) contacts for a particular email campaign. * **hardBounces & softBounces** - emailCampaignId is optional. Fetch the list of hard bounces / soft bounces for a particular / all email campaign(s).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionForEmailCampaigns {
    #[serde(rename = "openers")]
    Openers,
    #[serde(rename = "nonOpeners")]
    NonOpeners,
    #[serde(rename = "clickers")]
    Clickers,
    #[serde(rename = "nonClickers")]
    NonClickers,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
}

impl Default for ActionForEmailCampaigns {
    fn default() -> ActionForEmailCampaigns {
        Self::Openers
    }
}
/// **Mandatory if neither actionForContacts nor actionForEmailCampaigns is passed.** This will export the contacts on the basis of provided action applied on sms campaigns. * **unsubscribed** - Fetch the list of all unsubscribed (blacklisted via any means) contacts for all / particular sms campaigns. * **hardBounces & softBounces** - Fetch the list of hard bounces / soft bounces for all / particular sms campaigns.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionForSmsCampaigns {
    #[serde(rename = "hardBounces")]
    HardBounces,
    #[serde(rename = "softBounces")]
    SoftBounces,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}

impl Default for ActionForSmsCampaigns {
    fn default() -> ActionForSmsCampaigns {
        Self::HardBounces
    }
}
