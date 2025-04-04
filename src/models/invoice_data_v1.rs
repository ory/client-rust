/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.20.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvoiceDataV1 {
    #[serde(rename = "billing_period")]
    pub billing_period: Box<models::TimeInterval>,
    /// The currency of the invoice.
    #[serde(rename = "currency")]
    pub currency: String,
    /// Deleted is true if the invoice has been soft-deleted.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The items that are part of this invoice.
    #[serde(rename = "items")]
    pub items: Vec<models::LineItemV1>,
    /// The plan that this invoice is based on, in the format \"Name@version\".
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "stripe_invoice_item", skip_serializing_if = "Option::is_none")]
    pub stripe_invoice_item: Option<String>,
    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`. [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview)
    #[serde(rename = "stripe_invoice_status", skip_serializing_if = "Option::is_none")]
    pub stripe_invoice_status: Option<String>,
    /// An optional link to the invoice on Stripe.
    #[serde(rename = "stripe_link", skip_serializing_if = "Option::is_none")]
    pub stripe_link: Option<String>,
    /// The subtitle of the invoice.
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<Box<models::TaxLineItem>>,
    /// The title of the invoice.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "total_in_cent")]
    pub total_in_cent: i64,
}

impl InvoiceDataV1 {
    pub fn new(billing_period: models::TimeInterval, currency: String, items: Vec<models::LineItemV1>, title: String, total_in_cent: i64) -> InvoiceDataV1 {
        InvoiceDataV1 {
            billing_period: Box::new(billing_period),
            currency,
            deleted: None,
            items,
            plan: None,
            stripe_invoice_item: None,
            stripe_invoice_status: None,
            stripe_link: None,
            subtitle: None,
            tax: None,
            title,
            total_in_cent,
        }
    }
}

