/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.12.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiNodeScriptAttributes {
    /// The script async type
    #[serde(rename = "async")]
    pub r#async: bool,
    /// The script cross origin policy
    #[serde(rename = "crossorigin")]
    pub crossorigin: String,
    /// A unique identifier
    #[serde(rename = "id")]
    pub id: String,
    /// The script's integrity hash
    #[serde(rename = "integrity")]
    pub integrity: String,
    /// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0. In this struct it technically always is \"script\". text Text input Input img Image a Anchor script Script
    #[serde(rename = "node_type")]
    pub node_type: NodeTypeEnum,
    /// Nonce for CSP  A nonce you may want to use to improve your Content Security Policy. You do not have to use this value but if you want to improve your CSP policies you may use it. You can also choose to use your own nonce value!
    #[serde(rename = "nonce")]
    pub nonce: String,
    /// The script referrer policy
    #[serde(rename = "referrerpolicy")]
    pub referrerpolicy: String,
    /// The script source
    #[serde(rename = "src")]
    pub src: String,
    /// The script MIME type
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UiNodeScriptAttributes {
    pub fn new(r#async: bool, crossorigin: String, id: String, integrity: String, node_type: NodeTypeEnum, nonce: String, referrerpolicy: String, src: String, r#type: String) -> UiNodeScriptAttributes {
        UiNodeScriptAttributes {
            r#async,
            crossorigin,
            id,
            integrity,
            node_type,
            nonce,
            referrerpolicy,
            src,
            r#type,
        }
    }
}
/// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0. In this struct it technically always is \"script\". text Text input Input img Image a Anchor script Script
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeTypeEnum {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "img")]
    Img,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "script")]
    Script,
}

impl Default for NodeTypeEnum {
    fn default() -> NodeTypeEnum {
        Self::Text
    }
}

