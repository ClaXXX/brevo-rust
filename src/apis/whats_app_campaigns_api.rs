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

/// struct for typed errors of method [`create_whats_app_campaign`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWhatsAppCampaignError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_whats_app_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWhatsAppTemplateError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_whats_app_campaign`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWhatsAppCampaignError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_whats_app_campaign`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWhatsAppCampaignError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_whats_app_campaigns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWhatsAppCampaignsError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_whats_app_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWhatsAppConfigError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_whats_app_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWhatsAppTemplatesError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_whats_app_template_approval`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendWhatsAppTemplateApprovalError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_whats_app_campaign`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWhatsAppCampaignError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

pub async fn create_whats_app_campaign(
    configuration: &configuration::Configuration,
    create_whats_app_campaign: models::CreateWhatsAppCampaign,
) -> Result<models::CreateModel, Error<CreateWhatsAppCampaignError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_whats_app_campaign = create_whats_app_campaign;

    let uri_str = format!("{}/whatsappCampaigns", configuration.base_path);
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
    req_builder = req_builder.json(&p_create_whats_app_campaign);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateModel`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateModel`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateWhatsAppCampaignError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn create_whats_app_template(
    configuration: &configuration::Configuration,
    create_whats_app_template: models::CreateWhatsAppTemplate,
) -> Result<models::CreateModel, Error<CreateWhatsAppTemplateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_whats_app_template = create_whats_app_template;

    let uri_str = format!("{}/whatsappCampaigns/template", configuration.base_path);
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
    req_builder = req_builder.json(&p_create_whats_app_template);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateModel`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateModel`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateWhatsAppTemplateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn delete_whats_app_campaign(
    configuration: &configuration::Configuration,
    campaign_id: i64,
) -> Result<(), Error<DeleteWhatsAppCampaignError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_campaign_id = campaign_id;

    let uri_str = format!(
        "{}/whatsappCampaigns/{campaignId}",
        configuration.base_path,
        campaignId = p_campaign_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

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
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteWhatsAppCampaignError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_whats_app_campaign(
    configuration: &configuration::Configuration,
    campaign_id: i64,
) -> Result<models::GetWhatsappCampaignOverview, Error<GetWhatsAppCampaignError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_campaign_id = campaign_id;

    let uri_str = format!(
        "{}/whatsappCampaigns/{campaignId}",
        configuration.base_path,
        campaignId = p_campaign_id
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetWhatsappCampaignOverview`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetWhatsappCampaignOverview`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWhatsAppCampaignError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_whats_app_campaigns(
    configuration: &configuration::Configuration,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<&str>,
) -> Result<models::GetWhatsappCampaigns, Error<GetWhatsAppCampaignsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_limit = limit;
    let p_offset = offset;
    let p_sort = sort;

    let uri_str = format!("{}/whatsappCampaigns", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetWhatsappCampaigns`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetWhatsappCampaigns`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWhatsAppCampaignsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_whats_app_config(
    configuration: &configuration::Configuration,
) -> Result<models::GetWhatsAppConfig, Error<GetWhatsAppConfigError>> {
    let uri_str = format!("{}/whatsappCampaigns/config", configuration.base_path);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetWhatsAppConfig`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetWhatsAppConfig`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWhatsAppConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_whats_app_templates(
    configuration: &configuration::Configuration,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<&str>,
    source: Option<&str>,
) -> Result<models::GetWhatsappTemplates, Error<GetWhatsAppTemplatesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_limit = limit;
    let p_offset = offset;
    let p_sort = sort;
    let p_source = source;

    let uri_str = format!(
        "{}/whatsappCampaigns/template-list",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
    if let Some(ref param_value) = p_source {
        req_builder = req_builder.query(&[("source", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetWhatsappTemplates`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetWhatsappTemplates`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWhatsAppTemplatesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn send_whats_app_template_approval(
    configuration: &configuration::Configuration,
    template_id: i64,
) -> Result<(), Error<SendWhatsAppTemplateApprovalError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_template_id = template_id;

    let uri_str = format!(
        "{}/whatsappCampaigns/template/approval/{templateId}",
        configuration.base_path,
        templateId = p_template_id
    );
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

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<SendWhatsAppTemplateApprovalError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn update_whats_app_campaign(
    configuration: &configuration::Configuration,
    campaign_id: i64,
    update_whats_app_campaign: models::UpdateWhatsAppCampaign,
) -> Result<(), Error<UpdateWhatsAppCampaignError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_campaign_id = campaign_id;
    let p_update_whats_app_campaign = update_whats_app_campaign;

    let uri_str = format!(
        "{}/whatsappCampaigns/{campaignId}",
        configuration.base_path,
        campaignId = p_campaign_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
    req_builder = req_builder.json(&p_update_whats_app_campaign);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateWhatsAppCampaignError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
