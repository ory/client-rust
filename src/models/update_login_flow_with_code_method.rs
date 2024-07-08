/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.13.10
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateLoginFlowWithCodeMethod : Update Login flow using the code method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLoginFlowWithCodeMethod {
    /// Code is the 6 digits code sent to the user
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token")]
    pub csrf_token: String,
    /// Identifier is the code identifier The identifier requires that the user has already completed the registration or settings with code flow.
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Method should be set to \"code\" when logging in using the code strategy.
    #[serde(rename = "method")]
    pub method: String,
    /// Resend is set when the user wants to resend the code
    #[serde(rename = "resend", skip_serializing_if = "Option::is_none")]
    pub resend: Option<String>,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateLoginFlowWithCodeMethod {
    /// Update Login flow using the code method
    pub fn new(csrf_token: String, method: String) -> UpdateLoginFlowWithCodeMethod {
        UpdateLoginFlowWithCodeMethod {
            code: None,
            csrf_token,
            identifier: None,
            method,
            resend: None,
            transient_payload: None,
        }
    }
}

