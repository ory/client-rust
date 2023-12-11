/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.6
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// RelationshipNamespaces : Relationship Namespace List



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipNamespaces {
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<crate::models::Namespace>>,
}

impl Default for RelationshipNamespaces {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipNamespaces {
    /// Relationship Namespace List
    pub fn new() -> RelationshipNamespaces {
        RelationshipNamespaces {
                namespaces: None,
        }
    }
}


