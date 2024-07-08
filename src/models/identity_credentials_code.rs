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

/// IdentityCredentialsCode : CredentialsCode represents a one time login/registration code
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityCredentialsCode {
    /// The type of the address for this code
    #[serde(rename = "address_type", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(rename = "used_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub used_at: Option<Option<String>>,
}

impl IdentityCredentialsCode {
    /// CredentialsCode represents a one time login/registration code
    pub fn new() -> IdentityCredentialsCode {
        IdentityCredentialsCode {
            address_type: None,
            used_at: None,
        }
    }
}

