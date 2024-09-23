/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RelationshipNamespaces : Relationship Namespace List
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipNamespaces {
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<models::Namespace>>,
}

impl RelationshipNamespaces {
    /// Relationship Namespace List
    pub fn new() -> RelationshipNamespaces {
        RelationshipNamespaces {
            namespaces: None,
        }
    }
}

