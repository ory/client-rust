/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.18.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`batch_check_permission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchCheckPermissionError {
    Status400(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`check_permission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckPermissionError {
    Status400(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`check_permission_or_error`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckPermissionOrErrorError {
    Status400(models::ErrorGeneric),
    Status403(models::CheckPermissionResult),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`expand_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExpandPermissionsError {
    Status400(models::ErrorGeneric),
    Status404(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_check_permission`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCheckPermissionError {
    Status400(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_check_permission_or_error`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCheckPermissionOrErrorError {
    Status400(models::ErrorGeneric),
    Status403(models::CheckPermissionResult),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}


/// To learn how relationship tuples and the check works, head over to [the documentation](https://www.ory.sh/docs/keto/concepts/api-overview).
pub async fn batch_check_permission(configuration: &configuration::Configuration, max_depth: Option<i64>, batch_check_permission_body: Option<models::BatchCheckPermissionBody>) -> Result<models::BatchCheckPermissionResult, Error<BatchCheckPermissionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_max_depth = max_depth;
    let p_batch_check_permission_body = batch_check_permission_body;

    let uri_str = format!("{}/relation-tuples/batch/check", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_max_depth {
        req_builder = req_builder.query(&[("max-depth", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_batch_check_permission_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BatchCheckPermissionResult`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BatchCheckPermissionResult`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BatchCheckPermissionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To learn how relationship tuples and the check works, head over to [the documentation](https://www.ory.sh/docs/keto/concepts/api-overview).
pub async fn check_permission(configuration: &configuration::Configuration, namespace: Option<&str>, object: Option<&str>, relation: Option<&str>, subject_id: Option<&str>, subject_set_period_namespace: Option<&str>, subject_set_period_object: Option<&str>, subject_set_period_relation: Option<&str>, max_depth: Option<i64>) -> Result<models::CheckPermissionResult, Error<CheckPermissionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_namespace = namespace;
    let p_object = object;
    let p_relation = relation;
    let p_subject_id = subject_id;
    let p_subject_set_period_namespace = subject_set_period_namespace;
    let p_subject_set_period_object = subject_set_period_object;
    let p_subject_set_period_relation = subject_set_period_relation;
    let p_max_depth = max_depth;

    let uri_str = format!("{}/relation-tuples/check/openapi", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_namespace {
        req_builder = req_builder.query(&[("namespace", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_object {
        req_builder = req_builder.query(&[("object", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_relation {
        req_builder = req_builder.query(&[("relation", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_id {
        req_builder = req_builder.query(&[("subject_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_set_period_namespace {
        req_builder = req_builder.query(&[("subject_set.namespace", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_set_period_object {
        req_builder = req_builder.query(&[("subject_set.object", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_set_period_relation {
        req_builder = req_builder.query(&[("subject_set.relation", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_max_depth {
        req_builder = req_builder.query(&[("max-depth", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CheckPermissionResult`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CheckPermissionResult`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckPermissionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To learn how relationship tuples and the check works, head over to [the documentation](https://www.ory.sh/docs/keto/concepts/api-overview).
pub async fn check_permission_or_error(configuration: &configuration::Configuration, namespace: Option<&str>, object: Option<&str>, relation: Option<&str>, subject_id: Option<&str>, subject_set_period_namespace: Option<&str>, subject_set_period_object: Option<&str>, subject_set_period_relation: Option<&str>, max_depth: Option<i64>) -> Result<models::CheckPermissionResult, Error<CheckPermissionOrErrorError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_namespace = namespace;
    let p_object = object;
    let p_relation = relation;
    let p_subject_id = subject_id;
    let p_subject_set_period_namespace = subject_set_period_namespace;
    let p_subject_set_period_object = subject_set_period_object;
    let p_subject_set_period_relation = subject_set_period_relation;
    let p_max_depth = max_depth;

    let uri_str = format!("{}/relation-tuples/check", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_namespace {
        req_builder = req_builder.query(&[("namespace", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_object {
        req_builder = req_builder.query(&[("object", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_relation {
        req_builder = req_builder.query(&[("relation", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_id {
        req_builder = req_builder.query(&[("subject_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_set_period_namespace {
        req_builder = req_builder.query(&[("subject_set.namespace", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_set_period_object {
        req_builder = req_builder.query(&[("subject_set.object", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_subject_set_period_relation {
        req_builder = req_builder.query(&[("subject_set.relation", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_max_depth {
        req_builder = req_builder.query(&[("max-depth", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CheckPermissionResult`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CheckPermissionResult`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckPermissionOrErrorError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Use this endpoint to expand a relationship tuple into permissions.
pub async fn expand_permissions(configuration: &configuration::Configuration, namespace: &str, object: &str, relation: &str, max_depth: Option<i64>) -> Result<models::ExpandedPermissionTree, Error<ExpandPermissionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_namespace = namespace;
    let p_object = object;
    let p_relation = relation;
    let p_max_depth = max_depth;

    let uri_str = format!("{}/relation-tuples/expand", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("namespace", &p_namespace.to_string())]);
    req_builder = req_builder.query(&[("object", &p_object.to_string())]);
    req_builder = req_builder.query(&[("relation", &p_relation.to_string())]);
    if let Some(ref param_value) = p_max_depth {
        req_builder = req_builder.query(&[("max-depth", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ExpandedPermissionTree`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ExpandedPermissionTree`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ExpandPermissionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To learn how relationship tuples and the check works, head over to [the documentation](https://www.ory.sh/docs/keto/concepts/api-overview).
pub async fn post_check_permission(configuration: &configuration::Configuration, max_depth: Option<i64>, post_check_permission_body: Option<models::PostCheckPermissionBody>) -> Result<models::CheckPermissionResult, Error<PostCheckPermissionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_max_depth = max_depth;
    let p_post_check_permission_body = post_check_permission_body;

    let uri_str = format!("{}/relation-tuples/check/openapi", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_max_depth {
        req_builder = req_builder.query(&[("max-depth", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_post_check_permission_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CheckPermissionResult`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CheckPermissionResult`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostCheckPermissionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To learn how relationship tuples and the check works, head over to [the documentation](https://www.ory.sh/docs/keto/concepts/api-overview).
pub async fn post_check_permission_or_error(configuration: &configuration::Configuration, max_depth: Option<i64>, post_check_permission_or_error_body: Option<models::PostCheckPermissionOrErrorBody>) -> Result<models::CheckPermissionResult, Error<PostCheckPermissionOrErrorError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_max_depth = max_depth;
    let p_post_check_permission_or_error_body = post_check_permission_or_error_body;

    let uri_str = format!("{}/relation-tuples/check", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_max_depth {
        req_builder = req_builder.query(&[("max-depth", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_post_check_permission_or_error_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CheckPermissionResult`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CheckPermissionResult`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostCheckPermissionOrErrorError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

