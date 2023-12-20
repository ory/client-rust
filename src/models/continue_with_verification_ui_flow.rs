/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.7
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithVerificationUiFlow {
    /// The ID of the verification flow
    #[serde(rename = "id")]
    pub id: String,
    /// The URL of the verification flow
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The address that should be verified in this flow
    #[serde(rename = "verifiable_address")]
    pub verifiable_address: String,
}


impl ContinueWithVerificationUiFlow {
    pub fn new(id: String, verifiable_address: String) -> ContinueWithVerificationUiFlow {
        ContinueWithVerificationUiFlow {
                id,
                url: None,
                verifiable_address,
        }
    }
}


