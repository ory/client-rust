/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.12.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// EventStream : Event Stream
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventStream {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "role_arn", skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "topic_arn", skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl EventStream {
    /// Event Stream
    pub fn new() -> EventStream {
        EventStream {
            created_at: None,
            id: None,
            role_arn: None,
            topic_arn: None,
            r#type: None,
            updated_at: None,
        }
    }
}

