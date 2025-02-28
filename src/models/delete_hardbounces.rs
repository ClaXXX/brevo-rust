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
pub struct DeleteHardbounces {
    /// Starting date (YYYY-MM-DD) of the time period for deletion. The hardbounces occurred after this date will be deleted. Must be less than or equal to the endDate
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Ending date (YYYY-MM-DD) of the time period for deletion. The hardbounces until this date will be deleted. Must be greater than or equal to the startDate
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Target a specific email address
    #[serde(rename = "contactEmail", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
}

impl DeleteHardbounces {
    pub fn new() -> DeleteHardbounces {
        DeleteHardbounces {
            start_date: None,
            end_date: None,
            contact_email: None,
        }
    }
}
