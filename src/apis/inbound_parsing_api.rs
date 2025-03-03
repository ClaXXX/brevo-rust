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

/// struct for typed errors of method [`get_inbound_email_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboundEmailAttachmentError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_inbound_email_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboundEmailEventsError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_inbound_email_events_by_uuid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboundEmailEventsByUuidError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// This endpoint will retrieve inbound attachment with download token.
pub async fn get_inbound_email_attachment(
    configuration: &configuration::Configuration,
    download_token: &str,
) -> Result<reqwest::Response, Error<GetInboundEmailAttachmentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_download_token = download_token;

    let uri_str = format!(
        "{}/inbound/attachments/{downloadToken}",
        configuration.base_path,
        downloadToken = crate::apis::urlencode(p_download_token)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInboundEmailAttachmentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint will show the list of all the events for the received emails.
pub async fn get_inbound_email_events(
    configuration: &configuration::Configuration,
    sender: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<&str>,
) -> Result<models::GetInboundEmailEvents, Error<GetInboundEmailEventsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sender = sender;
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_limit = limit;
    let p_offset = offset;
    let p_sort = sort;

    let uri_str = format!("{}/inbound/events", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sender {
        req_builder = req_builder.query(&[("sender", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetInboundEmailEvents`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetInboundEmailEvents`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInboundEmailEventsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint will show the list of all events history for one particular received email.
pub async fn get_inbound_email_events_by_uuid(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::GetInboundEmailEventsByUuid, Error<GetInboundEmailEventsByUuidError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_uuid = uuid;

    let uri_str = format!(
        "{}/inbound/events/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetInboundEmailEventsByUuid`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetInboundEmailEventsByUuid`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInboundEmailEventsByUuidError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
