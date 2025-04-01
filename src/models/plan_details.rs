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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanDetails {
    /// BaseFeeMonthly is the monthly base fee for the plan.
    #[serde(rename = "base_fee_monthly")]
    pub base_fee_monthly: i64,
    /// BaseFeeYearly is the yearly base fee for the plan.
    #[serde(rename = "base_fee_yearly")]
    pub base_fee_yearly: i64,
    /// Custom is true if the plan is custom. This means it will be hidden from the pricing page.
    #[serde(rename = "custom")]
    pub custom: bool,
    /// Description is the description of the plan.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "features")]
    pub features: std::collections::HashMap<String, models::GenericUsage>,
    /// Latest is true if the plan is the latest version of a plan and should be available for self-service usage.
    #[serde(rename = "latest", skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    /// Name is the name of the plan.
    #[serde(rename = "name")]
    pub name: String,
    /// Version is the version of the plan. The combination of `name@version` must be unique.
    #[serde(rename = "version")]
    pub version: i64,
}

impl PlanDetails {
    pub fn new(base_fee_monthly: i64, base_fee_yearly: i64, custom: bool, description: String, features: std::collections::HashMap<String, models::GenericUsage>, name: String, version: i64) -> PlanDetails {
        PlanDetails {
            base_fee_monthly,
            base_fee_yearly,
            custom,
            description,
            features,
            latest: None,
            name,
            version,
        }
    }
}

