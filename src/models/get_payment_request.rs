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
pub struct GetPaymentRequest {
    /// Reference of the payment request, it will appear on the payment page.
    #[serde(rename = "reference")]
    pub reference: String,
    /// Status of the payment request.
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<models::Configuration>>,
    /// Brevo ID of the contact requested to pay.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// number of reminders sent.
    #[serde(
        rename = "numberOfRemindersSent",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_reminders_sent: Option<i64>,
    #[serde(rename = "cart")]
    pub cart: Box<models::Cart>,
    #[serde(rename = "notification")]
    pub notification: Box<models::Notification>,
}

impl GetPaymentRequest {
    pub fn new(
        reference: String,
        status: Status,
        cart: models::Cart,
        notification: models::Notification,
    ) -> GetPaymentRequest {
        GetPaymentRequest {
            reference,
            status,
            configuration: None,
            contact_id: None,
            number_of_reminders_sent: None,
            cart: Box::new(cart),
            notification: Box::new(notification),
        }
    }
}
/// Status of the payment request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "reminderSent")]
    ReminderSent,
    #[serde(rename = "paid")]
    Paid,
}

impl Default for Status {
    fn default() -> Status {
        Self::Created
    }
}
