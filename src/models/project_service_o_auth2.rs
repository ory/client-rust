/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.8.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceOAuth2 {
    #[serde(rename = "config")]
    pub config: serde_json::Value,
}


impl ProjectServiceOAuth2 {
    pub fn new(config: serde_json::Value) -> ProjectServiceOAuth2 {
        ProjectServiceOAuth2 {
                config,
        }
    }
}


