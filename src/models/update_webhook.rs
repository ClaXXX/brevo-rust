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
pub struct UpdateWebhook {
    /// URL of the webhook
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Description of the webhook
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// - Events triggering the webhook. Possible values for **Transactional** type webhook: #### `sent` OR `request`, `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and `unsubscribed` - Possible values for **Marketing** type webhook: #### `spam`, `opened`, `click`, `hardBounce`, `softBounce`, `unsubscribed`, `listAddition` & `delivered` - Possible values for **Inbound** type webhook: #### `inboundEmailProcessed`
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Events>>,
    /// Inbound domain of webhook, used in case of event type `inbound`
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Batching configuration of the webhook, we send batched webhooks if its true
    #[serde(rename = "batched", skip_serializing_if = "Option::is_none")]
    pub batched: Option<bool>,
    /// Authentication header to be send with the webhook requests
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Value>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<serde_json::Value>>,
}

impl UpdateWebhook {
    pub fn new() -> UpdateWebhook {
        UpdateWebhook {
            url: None,
            description: None,
            events: None,
            domain: None,
            batched: None,
            auth: None,
            headers: None,
        }
    }
}
/// - Events triggering the webhook. Possible values for **Transactional** type webhook: #### `sent` OR `request`, `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and `unsubscribed` - Possible values for **Marketing** type webhook: #### `spam`, `opened`, `click`, `hardBounce`, `softBounce`, `unsubscribed`, `listAddition` & `delivered` - Possible values for **Inbound** type webhook: #### `inboundEmailProcessed`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "hardBounce")]
    HardBounce,
    #[serde(rename = "softBounce")]
    SoftBounce,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "uniqueOpened")]
    UniqueOpened,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    #[serde(rename = "listAddition")]
    ListAddition,
    #[serde(rename = "contactUpdated")]
    ContactUpdated,
    #[serde(rename = "contactDeleted")]
    ContactDeleted,
    #[serde(rename = "inboundEmailProcessed")]
    InboundEmailProcessed,
}

impl Default for Events {
    fn default() -> Events {
        Self::Sent
    }
}
