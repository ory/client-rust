/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.7
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateSettingsFlowWithPasswordMethod : Update Settings Flow with Password Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsFlowWithPasswordMethod {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to password when trying to update a password.
    #[serde(rename = "method")]
    pub method: String,
    /// Password is the updated password
    #[serde(rename = "password")]
    pub password: String,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateSettingsFlowWithPasswordMethod {
    /// Update Settings Flow with Password Method
    pub fn new(method: String, password: String) -> UpdateSettingsFlowWithPasswordMethod {
        UpdateSettingsFlowWithPasswordMethod {
            csrf_token: None,
            method,
            password,
            transient_payload: None,
        }
    }
}

