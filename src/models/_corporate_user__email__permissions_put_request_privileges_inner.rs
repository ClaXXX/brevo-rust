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

/// CorporateUserEmailPermissionsPutRequestPrivilegesInner : Permission on features
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorporateUserEmailPermissionsPutRequestPrivilegesInner {
    /// feature name
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none")]
    pub feature: Option<Feature>,
    /// Permission for the feature
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permissions>>,
}

impl CorporateUserEmailPermissionsPutRequestPrivilegesInner {
    /// Permission on features
    pub fn new() -> CorporateUserEmailPermissionsPutRequestPrivilegesInner {
        CorporateUserEmailPermissionsPutRequestPrivilegesInner {
            feature: None,
            permissions: None,
        }
    }
}
/// feature name
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Feature {
    #[serde(rename = "user_management")]
    UserManagement,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "my_plan")]
    MyPlan,
    #[serde(rename = "apps_management")]
    AppsManagement,
    #[serde(rename = "analytics")]
    Analytics,
    #[serde(rename = "sub_organization_groups")]
    SubOrganizationGroups,
    #[serde(rename = "create_sub_organizations")]
    CreateSubOrganizations,
    #[serde(rename = "manage_sub_organizations")]
    ManageSubOrganizations,
    #[serde(rename = "security")]
    Security,
}

impl Default for Feature {
    fn default() -> Feature {
        Self::UserManagement
    }
}
/// Permission for the feature
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "edit_delete")]
    EditDelete,
    #[serde(rename = "create_alerts")]
    CreateAlerts,
    #[serde(rename = "download_data")]
    DownloadData,
    #[serde(rename = "my_looks")]
    MyLooks,
    #[serde(rename = "explore_create")]
    ExploreCreate,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Self::All
    }
}
