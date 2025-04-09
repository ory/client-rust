/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.20.5
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NormalizedProjectRevisionSamlProvider {
    /// The Project's Revision Creation Date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Label represents an optional label which can be used in the UI generation.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Mapper specifies the JSONNet code snippet which uses the OpenID Connect Provider's data (e.g. GitHub or Google profile information) to hydrate the identity's data.
    #[serde(rename = "mapper_url", skip_serializing_if = "Option::is_none")]
    pub mapper_url: Option<String>,
    #[serde(rename = "organization_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<Option<String>>,
    /// The Revision's ID this schema belongs to
    #[serde(rename = "project_revision_id", skip_serializing_if = "Option::is_none")]
    pub project_revision_id: Option<String>,
    /// ID is the provider's ID
    #[serde(rename = "provider_id", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    /// RawIDPMetadataXML is the raw XML metadata of the IDP.
    #[serde(rename = "raw_idp_metadata_xml", skip_serializing_if = "Option::is_none")]
    pub raw_idp_metadata_xml: Option<String>,
    /// State indicates the state of the provider  Only providers with state `enabled` will be used for authentication enabled ThirdPartyProviderStateEnabled disabled ThirdPartyProviderStateDisabled
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<StateEnum>,
    /// Last Time Project's Revision was Updated
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl NormalizedProjectRevisionSamlProvider {
    pub fn new() -> NormalizedProjectRevisionSamlProvider {
        NormalizedProjectRevisionSamlProvider {
            created_at: None,
            id: None,
            label: None,
            mapper_url: None,
            organization_id: None,
            project_revision_id: None,
            provider_id: None,
            raw_idp_metadata_xml: None,
            state: None,
            updated_at: None,
        }
    }
}
/// State indicates the state of the provider  Only providers with state `enabled` will be used for authentication enabled ThirdPartyProviderStateEnabled disabled ThirdPartyProviderStateDisabled
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateEnum {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for StateEnum {
    fn default() -> StateEnum {
        Self::Enabled
    }
}

