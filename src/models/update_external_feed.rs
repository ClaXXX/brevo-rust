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
pub struct UpdateExternalFeed {
    /// Name of the feed
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the feed
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Auth type of the feed:  * `basic`  * `token`  * `noAuth`
    #[serde(rename = "authType", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
    /// Username for authType `basic`
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password for authType `basic`
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Token for authType `token`
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Custom headers for the feed
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<models::GetExternalFeedByUuidHeadersInner>>,
    /// Maximum number of retries on the feed url
    #[serde(rename = "maxRetries", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    /// Toggle caching of feed url response
    #[serde(rename = "cache", skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
}

impl UpdateExternalFeed {
    pub fn new() -> UpdateExternalFeed {
        UpdateExternalFeed {
            name: None,
            url: None,
            auth_type: None,
            username: None,
            password: None,
            token: None,
            headers: None,
            max_retries: None,
            cache: None,
        }
    }
}
/// Auth type of the feed:  * `basic`  * `token`  * `noAuth`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "noAuth")]
    NoAuth,
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::Basic
    }
}
