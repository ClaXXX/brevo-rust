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

/// CompanyAttributesInner : List of attributes
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyAttributesInner {
    #[serde(rename = "internalName", skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "attributeTypeName", skip_serializing_if = "Option::is_none")]
    pub attribute_type_name: Option<String>,
    #[serde(rename = "attributeOptions", skip_serializing_if = "Option::is_none")]
    pub attribute_options: Option<Vec<serde_json::Value>>,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}

impl CompanyAttributesInner {
    /// List of attributes
    pub fn new() -> CompanyAttributesInner {
        CompanyAttributesInner {
            internal_name: None,
            label: None,
            attribute_type_name: None,
            attribute_options: None,
            is_required: None,
        }
    }
}
