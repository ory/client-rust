/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.20.2
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectBrandingColors {
    /// AccentDefaultColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "accent_default_color", skip_serializing_if = "Option::is_none")]
    pub accent_default_color: Option<String>,
    /// AccentDisabledColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "accent_disabled_color", skip_serializing_if = "Option::is_none")]
    pub accent_disabled_color: Option<String>,
    /// AccentEmphasisColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "accent_emphasis_color", skip_serializing_if = "Option::is_none")]
    pub accent_emphasis_color: Option<String>,
    /// AccentMutedColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "accent_muted_color", skip_serializing_if = "Option::is_none")]
    pub accent_muted_color: Option<String>,
    /// AccentSubtleColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "accent_subtle_color", skip_serializing_if = "Option::is_none")]
    pub accent_subtle_color: Option<String>,
    /// BackgroundCanvasColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "background_canvas_color", skip_serializing_if = "Option::is_none")]
    pub background_canvas_color: Option<String>,
    /// BackgroundSubtleColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "background_subtle_color", skip_serializing_if = "Option::is_none")]
    pub background_subtle_color: Option<String>,
    /// BackgroundSurfaceColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "background_surface_color", skip_serializing_if = "Option::is_none")]
    pub background_surface_color: Option<String>,
    /// BorderDefaultColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "border_default_color", skip_serializing_if = "Option::is_none")]
    pub border_default_color: Option<String>,
    /// ErrorDefaultColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "error_default_color", skip_serializing_if = "Option::is_none")]
    pub error_default_color: Option<String>,
    /// ErrorEmphasisColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "error_emphasis_color", skip_serializing_if = "Option::is_none")]
    pub error_emphasis_color: Option<String>,
    /// ErrorMutedColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "error_muted_color", skip_serializing_if = "Option::is_none")]
    pub error_muted_color: Option<String>,
    /// ErrorSubtleColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "error_subtle_color", skip_serializing_if = "Option::is_none")]
    pub error_subtle_color: Option<String>,
    /// ForegroundDefaultColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "foreground_default_color", skip_serializing_if = "Option::is_none")]
    pub foreground_default_color: Option<String>,
    /// ForegroundDisabledColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "foreground_disabled_color", skip_serializing_if = "Option::is_none")]
    pub foreground_disabled_color: Option<String>,
    /// ForegroundMutedColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "foreground_muted_color", skip_serializing_if = "Option::is_none")]
    pub foreground_muted_color: Option<String>,
    /// ForegroundOnAccentColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "foreground_on_accent_color", skip_serializing_if = "Option::is_none")]
    pub foreground_on_accent_color: Option<String>,
    /// ForegroundOnDarkColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "foreground_on_dark_color", skip_serializing_if = "Option::is_none")]
    pub foreground_on_dark_color: Option<String>,
    /// ForegroundOnDisabledColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "foreground_on_disabled_color", skip_serializing_if = "Option::is_none")]
    pub foreground_on_disabled_color: Option<String>,
    /// ForegroundSubtleColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "foreground_subtle_color", skip_serializing_if = "Option::is_none")]
    pub foreground_subtle_color: Option<String>,
    /// InputBackgroundColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "input_background_color", skip_serializing_if = "Option::is_none")]
    pub input_background_color: Option<String>,
    /// InputDisabledColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "input_disabled_color", skip_serializing_if = "Option::is_none")]
    pub input_disabled_color: Option<String>,
    /// InputPlaceholderColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "input_placeholder_color", skip_serializing_if = "Option::is_none")]
    pub input_placeholder_color: Option<String>,
    /// InputTextColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "input_text_color", skip_serializing_if = "Option::is_none")]
    pub input_text_color: Option<String>,
    /// Primary color is an hsla color value used to derive the other colors from for the Ory Account Experience theme.
    #[serde(rename = "primary_color", skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    /// Secondary color is a hsla color code used to derive the other colors from for the Ory Account Experience theme.
    #[serde(rename = "secondary_color", skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    /// SuccessEmphasisColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "success_emphasis_color", skip_serializing_if = "Option::is_none")]
    pub success_emphasis_color: Option<String>,
    /// TextDefaultColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "text_default_color", skip_serializing_if = "Option::is_none")]
    pub text_default_color: Option<String>,
    /// TextDisabledColor is a hex color code used by the Ory Account Experience theme.
    #[serde(rename = "text_disabled_color", skip_serializing_if = "Option::is_none")]
    pub text_disabled_color: Option<String>,
}

impl ProjectBrandingColors {
    pub fn new() -> ProjectBrandingColors {
        ProjectBrandingColors {
            accent_default_color: None,
            accent_disabled_color: None,
            accent_emphasis_color: None,
            accent_muted_color: None,
            accent_subtle_color: None,
            background_canvas_color: None,
            background_subtle_color: None,
            background_surface_color: None,
            border_default_color: None,
            error_default_color: None,
            error_emphasis_color: None,
            error_muted_color: None,
            error_subtle_color: None,
            foreground_default_color: None,
            foreground_disabled_color: None,
            foreground_muted_color: None,
            foreground_on_accent_color: None,
            foreground_on_dark_color: None,
            foreground_on_disabled_color: None,
            foreground_subtle_color: None,
            input_background_color: None,
            input_disabled_color: None,
            input_placeholder_color: None,
            input_text_color: None,
            primary_color: None,
            secondary_color: None,
            success_emphasis_color: None,
            text_default_color: None,
            text_disabled_color: None,
        }
    }
}

