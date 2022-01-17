/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.49
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiNodeTextAttributes {
    /// A unique identifier
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "node_type")]
    pub node_type: String,
    #[serde(rename = "text")]
    pub text: Box<crate::models::UiText>,
}

impl UiNodeTextAttributes {
    pub fn new(id: String, node_type: String, text: crate::models::UiText) -> UiNodeTextAttributes {
        UiNodeTextAttributes {
            id,
            node_type,
            text: Box::new(text),
        }
    }
}


