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
pub struct CrmDealsIdPatchRequest {
    /// Name of deal
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Attributes for deal update  To assign owner of a Deal you can send attributes.deal_owner and utilize the account email or ID.  If you wish to update the pipeline of a deal you need to provide the `pipeline` and the `deal_stage`  Pipeline and deal_stage are ids you can fetch using this endpoint `/crm/pipeline/details/{pipelineID}`
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    /// Warning - Using PATCH on linkedContactIds replaces the list of linked contacts. Omitted IDs will be removed.
    #[serde(rename = "linkedContactIds", skip_serializing_if = "Option::is_none")]
    pub linked_contact_ids: Option<Vec<i64>>,
    /// Warning - Using PATCH on linkedCompaniesIds replaces the list of linked contacts. Omitted IDs will be removed.
    #[serde(rename = "linkedCompaniesIds", skip_serializing_if = "Option::is_none")]
    pub linked_companies_ids: Option<Vec<String>>,
}

impl CrmDealsIdPatchRequest {
    pub fn new() -> CrmDealsIdPatchRequest {
        CrmDealsIdPatchRequest {
            name: None,
            attributes: None,
            linked_contact_ids: None,
            linked_companies_ids: None,
        }
    }
}
