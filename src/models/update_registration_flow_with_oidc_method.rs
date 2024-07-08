/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.13.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateRegistrationFlowWithOidcMethod : Update Registration Flow with OpenID Connect Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationFlowWithOidcMethod {
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// IDToken is an optional id token provided by an OIDC provider  If submitted, it is verified using the OIDC provider's public key set and the claims are used to populate the OIDC credentials of the identity. If the OIDC provider does not store additional claims (such as name, etc.) in the IDToken itself, you can use the `traits` field to populate the identity's traits. Note, that Apple only includes the users email in the IDToken.  Supported providers are Apple
    #[serde(rename = "id_token", skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    /// IDTokenNonce is the nonce, used when generating the IDToken. If the provider supports nonce validation, the nonce will be validated against this value and is required.
    #[serde(rename = "id_token_nonce", skip_serializing_if = "Option::is_none")]
    pub id_token_nonce: Option<String>,
    /// Method to use  This field must be set to `oidc` when using the oidc method.
    #[serde(rename = "method")]
    pub method: String,
    /// The provider to register with
    #[serde(rename = "provider")]
    pub provider: String,
    /// The identity traits
    #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
    pub traits: Option<serde_json::Value>,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
    /// UpstreamParameters are the parameters that are passed to the upstream identity provider.  These parameters are optional and depend on what the upstream identity provider supports. Supported parameters are: `login_hint` (string): The `login_hint` parameter suppresses the account chooser and either pre-fills the email box on the sign-in form, or selects the proper session. `hd` (string): The `hd` parameter limits the login/registration process to a Google Organization, e.g. `mycollege.edu`. `prompt` (string): The `prompt` specifies whether the Authorization Server prompts the End-User for reauthentication and consent, e.g. `select_account`.
    #[serde(rename = "upstream_parameters", skip_serializing_if = "Option::is_none")]
    pub upstream_parameters: Option<serde_json::Value>,
}

impl UpdateRegistrationFlowWithOidcMethod {
    /// Update Registration Flow with OpenID Connect Method
    pub fn new(method: String, provider: String) -> UpdateRegistrationFlowWithOidcMethod {
        UpdateRegistrationFlowWithOidcMethod {
            csrf_token: None,
            id_token: None,
            id_token_nonce: None,
            method,
            provider,
            traits: None,
            transient_payload: None,
            upstream_parameters: None,
        }
    }
}

