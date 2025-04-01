/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.20.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateIdentityBody : Update Identity Body
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIdentityBody {
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<models::IdentityWithCredentials>>,
    /// Store metadata about the user which is only accessible through admin APIs such as `GET /admin/identities/<id>`.
    #[serde(rename = "metadata_admin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata_admin: Option<Option<serde_json::Value>>,
    /// Store metadata about the identity which the identity itself can see when calling for example the session endpoint. Do not store sensitive information (e.g. credit score) about the identity in this field.
    #[serde(rename = "metadata_public", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata_public: Option<Option<serde_json::Value>>,
    /// SchemaID is the ID of the JSON Schema to be used for validating the identity's traits. If set will update the Identity's SchemaID.
    #[serde(rename = "schema_id")]
    pub schema_id: String,
    /// State is the identity's state. active StateActive inactive StateInactive
    #[serde(rename = "state")]
    pub state: StateEnum,
    /// Traits represent an identity's traits. The identity is able to create, modify, and delete traits in a self-service manner. The input will always be validated against the JSON Schema defined in `schema_id`.
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
}

impl UpdateIdentityBody {
    /// Update Identity Body
    pub fn new(schema_id: String, state: StateEnum, traits: serde_json::Value) -> UpdateIdentityBody {
        UpdateIdentityBody {
            credentials: None,
            metadata_admin: None,
            metadata_public: None,
            schema_id,
            state,
            traits,
        }
    }
}
/// State is the identity's state. active StateActive inactive StateInactive
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateEnum {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for StateEnum {
    fn default() -> StateEnum {
        Self::Active
    }
}

