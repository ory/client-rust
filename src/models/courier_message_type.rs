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

/// CourierMessageType : It can either be `email` or `phone`
/// It can either be `email` or `phone`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CourierMessageType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,

}

impl std::fmt::Display for CourierMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Phone => write!(f, "phone"),
        }
    }
}

impl Default for CourierMessageType {
    fn default() -> CourierMessageType {
        Self::Email
    }
}

