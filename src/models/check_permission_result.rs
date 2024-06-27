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

/// CheckPermissionResult : The content of the allowed field is mirrored in the HTTP status code.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckPermissionResult {
    /// whether the relation tuple is allowed
    #[serde(rename = "allowed")]
    pub allowed: bool,
}

impl CheckPermissionResult {
    /// The content of the allowed field is mirrored in the HTTP status code.
    pub fn new(allowed: bool) -> CheckPermissionResult {
        CheckPermissionResult {
            allowed,
        }
    }
}

