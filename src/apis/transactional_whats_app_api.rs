/*
 * Brevo API
 *
 * Brevo provide a RESTFul API that can be used with any languages. With this API, you will be able to :   - Manage your campaigns and get the statistics   - Manage your contacts   - Send transactional Emails and SMS   - and much more...  You can download our wrappers at https://github.com/orgs/brevo  **Possible responses**   | Code | Message |   | :-------------: | ------------- |   | 200  | OK. Successful Request  |   | 201  | OK. Successful Creation |   | 202  | OK. Request accepted |   | 204  | OK. Successful Update/Deletion  |   | 400  | Error. Bad Request  |   | 401  | Error. Authentication Needed  |   | 402  | Error. Not enough credit, plan upgrade needed  |   | 403  | Error. Permission denied  |   | 404  | Error. Object does not exist |   | 405  | Error. Method not allowed  |   | 406  | Error. Not Acceptable  |   | 422  | Error. Unprocessable Entity |
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: contact@brevo.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`get_whatsapp_event_report`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWhatsappEventReportError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_whatsapp_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendWhatsappMessageError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// This endpoint will show the unaggregated statistics for WhatsApp activity (30 days by default if `startDate` and `endDate` or `days` is not passed. The date range can not exceed 90 days)
pub async fn get_whatsapp_event_report(
    configuration: &configuration::Configuration,
    limit: Option<i64>,
    offset: Option<i64>,
    start_date: Option<&str>,
    end_date: Option<&str>,
    days: Option<i64>,
    contact_number: Option<&str>,
    event: Option<&str>,
    sort: Option<&str>,
) -> Result<models::GetWhatsappEventReport, Error<GetWhatsappEventReportError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_limit = limit;
    let p_offset = offset;
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_days = days;
    let p_contact_number = contact_number;
    let p_event = event;
    let p_sort = sort;

    let uri_str = format!("{}/whatsapp/statistics/events", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_days {
        req_builder = req_builder.query(&[("days", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_contact_number {
        req_builder = req_builder.query(&[("contactNumber", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_event {
        req_builder = req_builder.query(&[("event", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("api-key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetWhatsappEventReport`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetWhatsappEventReport`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWhatsappEventReportError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint is used to send a WhatsApp message. <br/>(**The first message you send using the API must contain a Template ID. You must create a template on WhatsApp on the Brevo platform to fetch the Template ID.**)
pub async fn send_whatsapp_message(
    configuration: &configuration::Configuration,
    send_whatsapp_message_request: models::SendWhatsappMessageRequest,
) -> Result<models::SendWhatsappMessage201Response, Error<SendWhatsappMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_send_whatsapp_message_request = send_whatsapp_message_request;

    let uri_str = format!("{}/whatsapp/sendMessage", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("api-key", value);
    };
    req_builder = req_builder.json(&p_send_whatsapp_message_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendWhatsappMessage201Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendWhatsappMessage201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendWhatsappMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
