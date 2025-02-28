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

/// InviteuserPrivilegesInner : Privileges given to the user
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteuserPrivilegesInner {
    /// Feature name
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none")]
    pub feature: Option<Feature>,
    /// Permissions for a given feature
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permissions>>,
}

impl InviteuserPrivilegesInner {
    /// Privileges given to the user
    pub fn new() -> InviteuserPrivilegesInner {
        InviteuserPrivilegesInner {
            feature: None,
            permissions: None,
        }
    }
}
/// Feature name
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Feature {
    #[serde(rename = "email_campaigns")]
    EmailCampaigns,
    #[serde(rename = "sms_campaigns")]
    SmsCampaigns,
    #[serde(rename = "contacts")]
    Contacts,
    #[serde(rename = "templates")]
    Templates,
    #[serde(rename = "workflows")]
    Workflows,
    #[serde(rename = "facebook_ads")]
    FacebookAds,
    #[serde(rename = "landing_pages")]
    LandingPages,
    #[serde(rename = "transactional_emails")]
    TransactionalEmails,
    #[serde(rename = "smtp_api")]
    SmtpApi,
    #[serde(rename = "user_management")]
    UserManagement,
    #[serde(rename = "sales_platform")]
    SalesPlatform,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "conversations")]
    Conversations,
    #[serde(rename = "senders_domains_dedicated_ips")]
    SendersDomainsDedicatedIps,
    #[serde(rename = "push_notifications")]
    PushNotifications,
    #[serde(rename = "companies")]
    Companies,
}

impl Default for Feature {
    fn default() -> Feature {
        Self::EmailCampaigns
    }
}
/// Permissions for a given feature
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "create_edit_delete")]
    CreateEditDelete,
    #[serde(rename = "send_schedule_suspend")]
    SendScheduleSuspend,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "export")]
    Export,
    #[serde(rename = "list_and_attributes")]
    ListAndAttributes,
    #[serde(rename = "forms")]
    Forms,
    #[serde(rename = "activate_deactivate")]
    ActivateDeactivate,
    #[serde(rename = "activate_deactivate_pause")]
    ActivateDeactivatePause,
    #[serde(rename = "settings")]
    Settings,
    #[serde(rename = "schedule_pause")]
    SchedulePause,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "logs")]
    Logs,
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "assign")]
    Assign,
    #[serde(rename = "configure")]
    Configure,
    #[serde(rename = "create_edit_deals")]
    CreateEditDeals,
    #[serde(rename = "delete_deals")]
    DeleteDeals,
    #[serde(rename = "manage_others_deals_tasks")]
    ManageOthersDealsTasks,
    #[serde(rename = "manage_owned_companies")]
    ManageOwnedCompanies,
    #[serde(rename = "manage_others_companies")]
    ManageOthersCompanies,
    #[serde(rename = "reports")]
    Reports,
    #[serde(rename = "senders_management")]
    SendersManagement,
    #[serde(rename = "domains_management")]
    DomainsManagement,
    #[serde(rename = "dedicated_ips_management")]
    DedicatedIpsManagement,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "smtp")]
    Smtp,
    #[serde(rename = "api_keys")]
    ApiKeys,
    #[serde(rename = "authorized_ips")]
    AuthorizedIps,
    #[serde(rename = "none")]
    None,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Self::CreateEditDelete
    }
}
