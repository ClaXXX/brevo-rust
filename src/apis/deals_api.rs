/*
 * Brevo API
 *
 * Brevo provide a RESTFul API that can be used with any languages. With this API, you will be able to :   - Manage your campaigns and get the statistics   - Manage your contacts   - Send transactional Emails and SMS   - and much more...  You can download our wrappers at https://github.com/orgs/brevo  **Possible responses**   | Code | Message |   | :-------------: | ------------- |   | 200  | OK. Successful Request  |   | 201  | OK. Successful Creation |   | 202  | OK. Request accepted |   | 204  | OK. Successful Update/Deletion  |   | 400  | Error. Bad Request  |   | 401  | Error. Authentication Needed  |   | 402  | Error. Not enough credit, plan upgrade needed  |   | 403  | Error. Permission denied  |   | 404  | Error. Object does not exist |   | 405  | Error. Method not allowed  |   | 406  | Error. Not Acceptable  |   | 422  | Error. Unprocessable Entity |
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: contact@brevo.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`crm_attributes_deals_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmAttributesDealsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_attributes_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmAttributesPostError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_deals_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmDealsGetError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_deals_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmDealsIdDeleteError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_deals_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmDealsIdGetError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_deals_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmDealsIdPatchError {
    Status400(models::ErrorModel),
    Status404(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_deals_import_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmDealsImportPostError {
    Status400(models::CompaniesImportPost400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_deals_link_unlink_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmDealsLinkUnlinkIdPatchError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_deals_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmDealsPostError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_pipeline_details_all_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmPipelineDetailsAllGetError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_pipeline_details_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmPipelineDetailsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crm_pipeline_details_pipeline_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrmPipelineDetailsPipelineIdGetError {
    Status400(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

pub async fn crm_attributes_deals_get(
    configuration: &configuration::Configuration,
) -> Result<Vec<models::DealAttributesInner>, Error<CrmAttributesDealsGetError>> {
    let uri_str = format!("{}/crm/attributes/deals", configuration.base_path);
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
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmAttributesDealsGetError> = serde_json::from_str(&content).ok();
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

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
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

pub async fn crm_deals_get(
    configuration: &configuration::Configuration,
    filters_left_square_bracket_attributes_period_deal_name_right_square_bracket: Option<&str>,
    filters_left_square_bracket_linked_companies_ids_right_square_bracket: Option<&str>,
    filters_left_square_bracket_linked_contacts_ids_right_square_bracket: Option<&str>,
    modified_since: Option<&str>,
    created_since: Option<&str>,
    offset: Option<i64>,
    limit: Option<i64>,
    sort: Option<&str>,
) -> Result<models::DealsList, Error<CrmDealsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_filters_left_square_bracket_attributes_period_deal_name_right_square_bracket =
        filters_left_square_bracket_attributes_period_deal_name_right_square_bracket;
    let p_filters_left_square_bracket_linked_companies_ids_right_square_bracket =
        filters_left_square_bracket_linked_companies_ids_right_square_bracket;
    let p_filters_left_square_bracket_linked_contacts_ids_right_square_bracket =
        filters_left_square_bracket_linked_contacts_ids_right_square_bracket;
    let p_modified_since = modified_since;
    let p_created_since = created_since;
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;

    let uri_str = format!("{}/crm/deals", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) =
        p_filters_left_square_bracket_attributes_period_deal_name_right_square_bracket
    {
        req_builder =
            req_builder.query(&[("filters[attributes.deal_name]", &param_value.to_string())]);
    }
    if let Some(ref param_value) =
        p_filters_left_square_bracket_linked_companies_ids_right_square_bracket
    {
        req_builder =
            req_builder.query(&[("filters[linkedCompaniesIds]", &param_value.to_string())]);
    }
    if let Some(ref param_value) =
        p_filters_left_square_bracket_linked_contacts_ids_right_square_bracket
    {
        req_builder =
            req_builder.query(&[("filters[linkedContactsIds]", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_modified_since {
        req_builder = req_builder.query(&[("modifiedSince", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_since {
        req_builder = req_builder.query(&[("createdSince", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
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

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmDealsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_deals_id_delete(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<(), Error<CrmDealsIdDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!(
        "{}/crm/deals/{id}",
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
        let entity: Option<CrmDealsIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_deals_id_get(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Deal, Error<CrmDealsIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!(
        "{}/crm/deals/{id}",
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

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmDealsIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_deals_id_patch(
    configuration: &configuration::Configuration,
    id: &str,
    crm_deals_id_patch_request: models::CrmDealsIdPatchRequest,
) -> Result<(), Error<CrmDealsIdPatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_crm_deals_id_patch_request = crm_deals_id_patch_request;

    let uri_str = format!(
        "{}/crm/deals/{id}",
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
    req_builder = req_builder.json(&p_crm_deals_id_patch_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmDealsIdPatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Import deals from a CSV file with mapping options.
pub async fn crm_deals_import_post(
    configuration: &configuration::Configuration,
    file: Option<std::path::PathBuf>,
    mapping: Option<serde_json::Value>,
) -> Result<models::CompaniesImportPost200Response, Error<CrmDealsImportPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_file = file;
    let p_mapping = mapping;

    let uri_str = format!("{}/crm/deals/import", configuration.base_path);
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

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmDealsImportPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_deals_link_unlink_id_patch(
    configuration: &configuration::Configuration,
    id: &str,
    crm_deals_link_unlink_id_patch_request: models::CrmDealsLinkUnlinkIdPatchRequest,
) -> Result<(), Error<CrmDealsLinkUnlinkIdPatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_crm_deals_link_unlink_id_patch_request = crm_deals_link_unlink_id_patch_request;

    let uri_str = format!(
        "{}/crm/deals/link-unlink/{id}",
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
    req_builder = req_builder.json(&p_crm_deals_link_unlink_id_patch_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmDealsLinkUnlinkIdPatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_deals_post(
    configuration: &configuration::Configuration,
    crm_deals_post_request: models::CrmDealsPostRequest,
) -> Result<models::CrmDealsPost201Response, Error<CrmDealsPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_crm_deals_post_request = crm_deals_post_request;

    let uri_str = format!("{}/crm/deals", configuration.base_path);
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
    req_builder = req_builder.json(&p_crm_deals_post_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmDealsPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_pipeline_details_all_get(
    configuration: &configuration::Configuration,
) -> Result<Vec<models::Pipeline>, Error<CrmPipelineDetailsAllGetError>> {
    let uri_str = format!("{}/crm/pipeline/details/all", configuration.base_path);
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
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmPipelineDetailsAllGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint is deprecated. Prefer /crm/pipeline/details/{pipelineID} instead.
pub async fn crm_pipeline_details_get(
    configuration: &configuration::Configuration,
) -> Result<models::Pipeline, Error<CrmPipelineDetailsGetError>> {
    let uri_str = format!("{}/crm/pipeline/details", configuration.base_path);
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
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmPipelineDetailsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn crm_pipeline_details_pipeline_id_get(
    configuration: &configuration::Configuration,
    pipeline_id: &str,
) -> Result<Vec<models::Pipeline>, Error<CrmPipelineDetailsPipelineIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_pipeline_id = pipeline_id;

    let uri_str = format!(
        "{}/crm/pipeline/details/{pipelineID}",
        configuration.base_path,
        pipelineID = crate::apis::urlencode(p_pipeline_id)
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
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CrmPipelineDetailsPipelineIdGetError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
