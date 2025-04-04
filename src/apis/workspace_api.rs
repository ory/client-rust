/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.20.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWorkspaceError {
    Status400(models::ErrorGeneric),
    Status401(models::ErrorGeneric),
    Status403(models::ErrorGeneric),
    Status500(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_workspace_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWorkspaceApiKeyError {
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_workspace_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorkspaceApiKeyError {
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorkspaceError {
    Status400(models::ErrorGeneric),
    Status401(models::ErrorGeneric),
    Status403(models::ErrorGeneric),
    Status500(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_workspace_api_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWorkspaceApiKeysError {
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_workspace_projects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWorkspaceProjectsError {
    Status400(models::ErrorGeneric),
    Status401(models::ErrorGeneric),
    Status403(models::ErrorGeneric),
    Status500(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_workspaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWorkspacesError {
    Status400(models::ErrorGeneric),
    Status401(models::ErrorGeneric),
    Status403(models::ErrorGeneric),
    Status500(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorkspaceError {
    Status400(models::ErrorGeneric),
    Status401(models::ErrorGeneric),
    Status403(models::ErrorGeneric),
    Status500(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}


pub async fn create_workspace(configuration: &configuration::Configuration, create_workspace_body: Option<models::CreateWorkspaceBody>) -> Result<models::Workspace, Error<CreateWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_workspace_body = create_workspace_body;

    let uri_str = format!("{}/workspaces", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_workspace_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Workspace`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Workspace`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Create an API key for a workspace.
pub async fn create_workspace_api_key(configuration: &configuration::Configuration, workspace: &str, create_workspace_api_key_body: Option<models::CreateWorkspaceApiKeyBody>) -> Result<models::WorkspaceApiKey, Error<CreateWorkspaceApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_workspace = workspace;
    let p_create_workspace_api_key_body = create_workspace_api_key_body;

    let uri_str = format!("{}/workspaces/{workspace}/tokens", configuration.base_path, workspace=crate::apis::urlencode(p_workspace));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_workspace_api_key_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WorkspaceApiKey`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WorkspaceApiKey`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateWorkspaceApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes an API key and immediately removes it.
pub async fn delete_workspace_api_key(configuration: &configuration::Configuration, workspace: &str, token_id: &str) -> Result<(), Error<DeleteWorkspaceApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_workspace = workspace;
    let p_token_id = token_id;

    let uri_str = format!("{}/workspaces/{workspace}/tokens/{token_id}", configuration.base_path, workspace=crate::apis::urlencode(p_workspace), token_id=crate::apis::urlencode(p_token_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteWorkspaceApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Any workspace member can access this endpoint.
pub async fn get_workspace(configuration: &configuration::Configuration, workspace: &str) -> Result<models::Workspace, Error<GetWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_workspace = workspace;

    let uri_str = format!("{}/workspaces/{workspace}", configuration.base_path, workspace=crate::apis::urlencode(p_workspace));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Workspace`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Workspace`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// A list of all the workspace's API keys.
pub async fn list_workspace_api_keys(configuration: &configuration::Configuration, workspace: &str) -> Result<Vec<models::WorkspaceApiKey>, Error<ListWorkspaceApiKeysError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_workspace = workspace;

    let uri_str = format!("{}/workspaces/{workspace}/tokens", configuration.base_path, workspace=crate::apis::urlencode(p_workspace));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::WorkspaceApiKey&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::WorkspaceApiKey&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListWorkspaceApiKeysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Any workspace member can access this endpoint.
pub async fn list_workspace_projects(configuration: &configuration::Configuration, workspace: &str) -> Result<models::ListWorkspaceProjects, Error<ListWorkspaceProjectsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_workspace = workspace;

    let uri_str = format!("{}/workspaces/{workspace}/projects", configuration.base_path, workspace=crate::apis::urlencode(p_workspace));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListWorkspaceProjects`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListWorkspaceProjects`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListWorkspaceProjectsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_workspaces(configuration: &configuration::Configuration, page_size: Option<i64>, page_token: Option<&str>) -> Result<models::ListWorkspaces, Error<ListWorkspacesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page_size = page_size;
    let p_page_token = page_token;

    let uri_str = format!("{}/workspaces", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_token {
        req_builder = req_builder.query(&[("page_token", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListWorkspaces`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListWorkspaces`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListWorkspacesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Workspace members with the role `OWNER` can access this endpoint.
pub async fn update_workspace(configuration: &configuration::Configuration, workspace: &str, update_workspace_body: Option<models::UpdateWorkspaceBody>) -> Result<models::Workspace, Error<UpdateWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_workspace = workspace;
    let p_update_workspace_body = update_workspace_body;

    let uri_str = format!("{}/workspaces/{workspace}", configuration.base_path, workspace=crate::apis::urlencode(p_workspace));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_update_workspace_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Workspace`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Workspace`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

