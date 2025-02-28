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

/// ConversationsMessageFileImageInfo : image info is passed in case the file is an image
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationsMessageFileImageInfo {
    /// Width of the image
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// height of the image
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// URL of the preview
    #[serde(rename = "previewUrl", skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

impl ConversationsMessageFileImageInfo {
    /// image info is passed in case the file is an image
    pub fn new() -> ConversationsMessageFileImageInfo {
        ConversationsMessageFileImageInfo {
            width: None,
            height: None,
            preview_url: None,
        }
    }
}
