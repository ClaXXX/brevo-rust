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

/// TransactionHistoryResp : Response containing transaction history details for a specific balance and contact.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionHistoryResp {
    /// Unique identifier of the associated balance definition.
    #[serde(
        rename = "balanceDefinitionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_definition_id: Option<String>,
    /// Unique identifier of the contact related to the transactions.
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i32>,
    /// Total number of transactions in the history.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Unique identifier of the associated loyalty program.
    #[serde(rename = "loyaltyProgramId", skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// List of past transactions associated with the balance.
    #[serde(rename = "transactionHistory", skip_serializing_if = "Option::is_none")]
    pub transaction_history: Option<Vec<models::TransactionHistory>>,
}

impl TransactionHistoryResp {
    /// Response containing transaction history details for a specific balance and contact.
    pub fn new() -> TransactionHistoryResp {
        TransactionHistoryResp {
            balance_definition_id: None,
            contact_id: None,
            count: None,
            loyalty_program_id: None,
            transaction_history: None,
        }
    }
}
