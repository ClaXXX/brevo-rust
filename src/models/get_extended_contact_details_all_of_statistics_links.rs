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
pub struct GetExtendedContactDetailsAllOfStatisticsLinks {
    /// Number of clicks on this link for the campaign
    #[serde(rename = "count")]
    pub count: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    pub event_time: String,
    /// IP from which the user has clicked on the link
    #[serde(rename = "ip")]
    pub ip: String,
    /// URL of the clicked link
    #[serde(rename = "url")]
    pub url: String,
}

impl GetExtendedContactDetailsAllOfStatisticsLinks {
    pub fn new(
        count: i64,
        event_time: String,
        ip: String,
        url: String,
    ) -> GetExtendedContactDetailsAllOfStatisticsLinks {
        GetExtendedContactDetailsAllOfStatisticsLinks {
            count,
            event_time,
            ip,
            url,
        }
    }
}
