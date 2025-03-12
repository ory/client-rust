/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.18.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

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
    /// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0. In this struct it technically always is \"script\". text Text input Input img Image a Anchor script Script div Division
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
/// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0. In this struct it technically always is \"script\". text Text input Input img Image a Anchor script Script div Division
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
    #[serde(rename = "div")]
    Div,
}

impl Default for NodeTypeEnum {
    fn default() -> NodeTypeEnum {
        Self::Text
    }
}

