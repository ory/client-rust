/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// InternalIsOwnerForProjectBySlugBody : Is Owner For Project By Slug Request Body
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalIsOwnerForProjectBySlugBody {
    /// Namespace is the namespace of the subject.
    #[serde(rename = "namespace")]
    pub namespace: NamespaceEnum,
    /// ProjectScope is the project_id resolved from the API Token.
    #[serde(rename = "project_scope", skip_serializing_if = "Option::is_none")]
    pub project_scope: Option<String>,
    /// ProjectSlug is the project's slug.
    #[serde(rename = "project_slug")]
    pub project_slug: String,
    /// Subject is the subject acting (user or API key).
    #[serde(rename = "subject")]
    pub subject: String,
}

impl InternalIsOwnerForProjectBySlugBody {
    /// Is Owner For Project By Slug Request Body
    pub fn new(namespace: NamespaceEnum, project_slug: String, subject: String) -> InternalIsOwnerForProjectBySlugBody {
        InternalIsOwnerForProjectBySlugBody {
            namespace,
            project_scope: None,
            project_slug,
            subject,
        }
    }
}
/// Namespace is the namespace of the subject.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NamespaceEnum {
    #[serde(rename = "User")]
    User,
    #[serde(rename = " ApiKey")]
    ApiKey,
}

impl Default for NamespaceEnum {
    fn default() -> NamespaceEnum {
        Self::User
    }
}

