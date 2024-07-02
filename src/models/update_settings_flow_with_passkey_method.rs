/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.12.2
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// UpdateSettingsFlowWithPasskeyMethod : Update Settings Flow with Passkey Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsFlowWithPasskeyMethod {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to \"passkey\" when trying to add, update, or remove a webAuthn pairing.
    #[serde(rename = "method")]
    pub method: String,
    /// Remove a WebAuthn Security Key  This must contain the ID of the WebAuthN connection.
    #[serde(rename = "passkey_remove", skip_serializing_if = "Option::is_none")]
    pub passkey_remove: Option<String>,
    /// Register a WebAuthn Security Key  It is expected that the JSON returned by the WebAuthn registration process is included here.
    #[serde(rename = "passkey_settings_register", skip_serializing_if = "Option::is_none")]
    pub passkey_settings_register: Option<String>,
}

impl UpdateSettingsFlowWithPasskeyMethod {
    /// Update Settings Flow with Passkey Method
    pub fn new(method: String) -> UpdateSettingsFlowWithPasskeyMethod {
        UpdateSettingsFlowWithPasskeyMethod {
            csrf_token: None,
            method,
            passkey_remove: None,
            passkey_settings_register: None,
        }
    }
}

