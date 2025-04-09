/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.20.6
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CustomDomain : Custom Hostname
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[serde(rename = "cookie_domain", skip_serializing_if = "Option::is_none")]
    pub cookie_domain: Option<String>,
    #[serde(rename = "cors_allowed_origins", skip_serializing_if = "Option::is_none")]
    pub cors_allowed_origins: Option<Vec<String>>,
    #[serde(rename = "cors_enabled", skip_serializing_if = "Option::is_none")]
    pub cors_enabled: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "custom_ui_base_url", skip_serializing_if = "Option::is_none")]
    pub custom_ui_base_url: Option<String>,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ssl_status", skip_serializing_if = "Option::is_none")]
    pub ssl_status: Option<SslStatusEnum>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "verification_errors", skip_serializing_if = "Option::is_none")]
    pub verification_errors: Option<Vec<String>>,
    #[serde(rename = "verification_status", skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}

impl CustomDomain {
    /// Custom Hostname
    pub fn new() -> CustomDomain {
        CustomDomain {
            cookie_domain: None,
            cors_allowed_origins: None,
            cors_enabled: None,
            created_at: None,
            custom_ui_base_url: None,
            hostname: None,
            id: None,
            ssl_status: None,
            updated_at: None,
            verification_errors: None,
            verification_status: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SslStatusEnum {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "pending_validation")]
    PendingValidation,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "pending_issuance")]
    PendingIssuance,
    #[serde(rename = "pending_deployment")]
    PendingDeployment,
    #[serde(rename = "pending_deletion")]
    PendingDeletion,
    #[serde(rename = "pending_expiration")]
    PendingExpiration,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "initializing_timed_out")]
    InitializingTimedOut,
    #[serde(rename = "validation_timed_out")]
    ValidationTimedOut,
    #[serde(rename = "issuance_timed_out")]
    IssuanceTimedOut,
    #[serde(rename = "deployment_timed_out")]
    DeploymentTimedOut,
    #[serde(rename = "deletion_timed_out")]
    DeletionTimedOut,
    #[serde(rename = "pending_cleanup")]
    PendingCleanup,
    #[serde(rename = "staging_deployment")]
    StagingDeployment,
    #[serde(rename = "staging_active")]
    StagingActive,
    #[serde(rename = "deactivating")]
    Deactivating,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "backup_issued")]
    BackupIssued,
    #[serde(rename = "holding_deployment")]
    HoldingDeployment,
    #[serde(rename = "")]
    Empty,
}

impl Default for SslStatusEnum {
    fn default() -> SslStatusEnum {
        Self::Initializing
    }
}

