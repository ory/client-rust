/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.29
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// ErrorGeneric : The standard Ory JSON API error format.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorGeneric {
    #[serde(rename = "error")]
    pub error: Box<crate::models::GenericErrorContent>,
}


impl ErrorGeneric {
    /// The standard Ory JSON API error format.
    pub fn new(error: crate::models::GenericErrorContent) -> ErrorGeneric {
        ErrorGeneric {
                error: Box::new(error),
        }
    }
}


