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

/// TransactionHistory : Represents a record of a past transaction, including status and key timestamps.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionHistory {
    /// The transaction amount.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Expiration date of the balance associated with this transaction.
    #[serde(
        rename = "balanceExpirationDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_expiration_date: Option<String>,
    /// Timestamp when the transaction was canceled, if applicable.
    #[serde(rename = "cancelledAt", skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    /// Timestamp when the transaction was successfully completed.
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Timestamp when the transaction was initiated.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Unique identifier of the transaction.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional metadata associated with the transaction.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Reason for rejection, if the transaction was declined.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// Timestamp when the transaction was rejected.
    #[serde(rename = "rejectedAt", skip_serializing_if = "Option::is_none")]
    pub rejected_at: Option<String>,
    /// Current status of the transaction (e.g., pending, completed, rejected).
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl TransactionHistory {
    /// Represents a record of a past transaction, including status and key timestamps.
    pub fn new() -> TransactionHistory {
        TransactionHistory {
            amount: None,
            balance_expiration_date: None,
            cancelled_at: None,
            completed_at: None,
            created_at: None,
            id: None,
            meta: None,
            reject_reason: None,
            rejected_at: None,
            status: None,
        }
    }
}
