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

/// struct for typed errors of method [`companies_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompaniesGetError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`companies_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompaniesIdDeleteError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`companies_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompaniesIdGetError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`companies_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompaniesIdPatchError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`companies_import_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompaniesImportPostError {
    Status400(models::CompaniesImportPost400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`companies_link_unlink_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompaniesLinkUnlinkIdPatchError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`companies_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompaniesPostError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_attributes_companies_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmAttributesCompaniesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_attributes_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmAttributesPostError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

pub async fn companies_get(
    configuration: &configuration::Configuration,
    filters: Option<&str>,
    linked_contacts_ids: Option<i64>,
    linked_deals_ids: Option<&str>,
    modified_since: Option<&str>,
    created_since: Option<&str>,
    page: Option<i64>,
    limit: Option<i64>,
    sort: Option<&str>,
    sort_by: Option<&str>,
) -> Result<models::CompaniesList, Error<CompaniesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_filters = filters;
    let p_linked_contacts_ids = linked_contacts_ids;
    let p_linked_deals_ids = linked_deals_ids;
    let p_modified_since = modified_since;
    let p_created_since = created_since;
    let p_page = page;
    let p_limit = limit;
    let p_sort = sort;
    let p_sort_by = sort_by;

    let uri_str = format!("{}/companies", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_linked_contacts_ids {
        req_builder = req_builder.query(&[("linkedContactsIds", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_linked_deals_ids {
        req_builder = req_builder.query(&[("linkedDealsIds", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_modified_since {
        req_builder = req_builder.query(&[("modifiedSince", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_since {
        req_builder = req_builder.query(&[("createdSince", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort_by {
        req_builder = req_builder.query(&[("sortBy", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CompaniesList`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CompaniesList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CompaniesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn companies_id_delete(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<(), Error<CompaniesIdDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!(
        "{}/companies/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_id)
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
        let entity: Option<CompaniesIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn companies_id_get(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Company, Error<CompaniesIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!(
        "{}/companies/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Company`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Company`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CompaniesIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn companies_id_patch(
    configuration: &configuration::Configuration,
    id: &str,
    companies_id_patch_request: models::CompaniesIdPatchRequest,
) -> Result<models::Company, Error<CompaniesIdPatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_companies_id_patch_request = companies_id_patch_request;

    let uri_str = format!(
        "{}/companies/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

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
    req_builder = req_builder.json(&p_companies_id_patch_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Company`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Company`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CompaniesIdPatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Import companies from a CSV file with mapping options.
pub async fn companies_import_post(
    configuration: &configuration::Configuration,
    file: Option<std::path::PathBuf>,
    mapping: Option<serde_json::Value>,
) -> Result<models::CompaniesImportPost200Response, Error<CompaniesImportPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let _p_file = file;
    let p_mapping = mapping;

    let uri_str = format!("{}/companies/import", configuration.base_path);
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
    let mut multipart_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    if let Some(param_value) = p_mapping {
        multipart_form = multipart_form.text("mapping", param_value.to_string());
    }
    req_builder = req_builder.multipart(multipart_form);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CompaniesImportPost200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CompaniesImportPost200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CompaniesImportPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn companies_link_unlink_id_patch(
    configuration: &configuration::Configuration,
    id: &str,
    companies_link_unlink_id_patch_request: models::CompaniesLinkUnlinkIdPatchRequest,
) -> Result<(), Error<CompaniesLinkUnlinkIdPatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_companies_link_unlink_id_patch_request = companies_link_unlink_id_patch_request;

    let uri_str = format!(
        "{}/companies/link-unlink/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

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
    req_builder = req_builder.json(&p_companies_link_unlink_id_patch_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CompaniesLinkUnlinkIdPatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn companies_post(
    configuration: &configuration::Configuration,
    companies_post_request: models::CompaniesPostRequest,
) -> Result<models::CompaniesPost200Response, Error<CompaniesPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_companies_post_request = companies_post_request;

    let uri_str = format!("{}/companies", configuration.base_path);
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
    req_builder = req_builder.json(&p_companies_post_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CompaniesPost200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CompaniesPost200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CompaniesPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_attributes_companies_get(
    configuration: &configuration::Configuration,
) -> Result<Vec<models::CompanyAttributesInner>, Error<CrmAttributesCompaniesGetError>> {
    let uri_str = format!("{}/crm/attributes/companies", configuration.base_path);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::CompanyAttributesInner&gt;`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::CompanyAttributesInner&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmAttributesCompaniesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_attributes_post(
    configuration: &configuration::Configuration,
    crm_attributes_post_request: models::CrmAttributesPostRequest,
) -> Result<models::CrmAttributesPost200Response, Error<CrmAttributesPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_crm_attributes_post_request = crm_attributes_post_request;

    let uri_str = format!("{}/crm/attributes", configuration.base_path);
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
    req_builder = req_builder.json(&p_crm_attributes_post_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CrmAttributesPost200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CrmAttributesPost200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmAttributesPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
