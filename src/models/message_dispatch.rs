/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.16.5
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MessageDispatch : MessageDispatch represents an attempt of sending a courier message It contains the status of the attempt (failed or successful) and the error if any occured
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDispatch {
    /// CreatedAt is a helper struct field for gobuffalo.pop.
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    /// The ID of this message dispatch
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the message being dispatched
    #[serde(rename = "message_id")]
    pub message_id: String,
    /// The status of this dispatch Either \"failed\" or \"success\" failed CourierMessageDispatchStatusFailed success CourierMessageDispatchStatusSuccess
    #[serde(rename = "status")]
    pub status: StatusEnum,
    /// UpdatedAt is a helper struct field for gobuffalo.pop.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl MessageDispatch {
    /// MessageDispatch represents an attempt of sending a courier message It contains the status of the attempt (failed or successful) and the error if any occured
    pub fn new(created_at: String, id: String, message_id: String, status: StatusEnum, updated_at: String) -> MessageDispatch {
        MessageDispatch {
            created_at,
            error: None,
            id,
            message_id,
            status,
            updated_at,
        }
    }
}
/// The status of this dispatch Either \"failed\" or \"success\" failed CourierMessageDispatchStatusFailed success CourierMessageDispatchStatusSuccess
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "success")]
    Success,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Failed
    }
}

