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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceIdentity {
    #[serde(rename = "config")]
    pub config: serde_json::Value,
}

impl ProjectServiceIdentity {
    pub fn new(config: serde_json::Value) -> ProjectServiceIdentity {
        ProjectServiceIdentity {
            config,
        }
    }
}

