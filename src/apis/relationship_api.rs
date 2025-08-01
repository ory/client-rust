/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.21.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`check_opl_syntax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckOplSyntaxError {
    Status400(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_relationship`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRelationshipError {
    Status400(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_relationships`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRelationshipsError {
    Status400(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_relationships`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRelationshipsError {
    Status404(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_relationship_namespaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRelationshipNamespacesError {
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`patch_relationships`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchRelationshipsError {
    Status400(models::ErrorGeneric),
    Status404(models::ErrorGeneric),
    DefaultResponse(models::ErrorGeneric),
    UnknownValue(serde_json::Value),
}


/// The OPL file is expected in the body of the request.
pub async fn check_opl_syntax(configuration: &configuration::Configuration, body: Option<&str>) -> Result<models::CheckOplSyntaxResult, Error<CheckOplSyntaxError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/opl/syntax/check", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CheckOplSyntaxResult`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CheckOplSyntaxResult`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckOplSyntaxError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Use this endpoint to create a relationship.
pub async fn create_relationship(configuration: &configuration::Configuration, create_relationship_body: Option<models::CreateRelationshipBody>) -> Result<models::Relationship, Error<CreateRelationshipError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_relationship_body = create_relationship_body;

    let uri_str = format!("{}/admin/relation-tuples", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_relationship_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Relationship`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Relationship`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateRelationshipError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Use this endpoint to delete relationships
pub async fn delete_relationships(configuration: &configuration::Configuration, namespace: Option<&str>, object: Option<&str>, relation: Option<&str>, subject_id: Option<&str>, subject_set_period_namespace: Option<&str>, subject_set_period_object: Option<&str>, subject_set_period_relation: Option<&str>) -> Result<(), Error<DeleteRelationshipsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_namespace = namespace;
    let p_object = object;
    let p_relation = relation;
    let p_subject_id = subject_id;
    let p_subject_set_period_namespace = subject_set_period_namespace;
    let p_subject_set_period_object = subject_set_period_object;
    let p_subject_set_period_relation = subject_set_period_relation;

    let uri_str = format!("{}/admin/relation-tuples", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

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
        let entity: Option<DeleteRelationshipsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get all relationships that match the query. Only the namespace field is required.
pub async fn get_relationships(configuration: &configuration::Configuration, page_token: Option<&str>, page_size: Option<i64>, namespace: Option<&str>, object: Option<&str>, relation: Option<&str>, subject_id: Option<&str>, subject_set_period_namespace: Option<&str>, subject_set_period_object: Option<&str>, subject_set_period_relation: Option<&str>) -> Result<models::Relationships, Error<GetRelationshipsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page_token = page_token;
    let p_page_size = page_size;
    let p_namespace = namespace;
    let p_object = object;
    let p_relation = relation;
    let p_subject_id = subject_id;
    let p_subject_set_period_namespace = subject_set_period_namespace;
    let p_subject_set_period_object = subject_set_period_object;
    let p_subject_set_period_relation = subject_set_period_relation;

    let uri_str = format!("{}/relation-tuples", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page_token {
        req_builder = req_builder.query(&[("page_token", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Relationships`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Relationships`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRelationshipsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get all namespaces
pub async fn list_relationship_namespaces(configuration: &configuration::Configuration, ) -> Result<models::RelationshipNamespaces, Error<ListRelationshipNamespacesError>> {

    let uri_str = format!("{}/namespaces", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RelationshipNamespaces`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RelationshipNamespaces`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListRelationshipNamespacesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Use this endpoint to patch one or more relationships.
pub async fn patch_relationships(configuration: &configuration::Configuration, relationship_patch: Option<Vec<models::RelationshipPatch>>) -> Result<(), Error<PatchRelationshipsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_relationship_patch = relationship_patch;

    let uri_str = format!("{}/admin/relation-tuples", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_relationship_patch);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PatchRelationshipsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

