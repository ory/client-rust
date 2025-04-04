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

/// UiNodeInputAttributes : InputAttributes represents the attributes of an input node
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiNodeInputAttributes {
    /// The autocomplete attribute for the input. email InputAttributeAutocompleteEmail tel InputAttributeAutocompleteTel url InputAttributeAutocompleteUrl current-password InputAttributeAutocompleteCurrentPassword new-password InputAttributeAutocompleteNewPassword one-time-code InputAttributeAutocompleteOneTimeCode
    #[serde(rename = "autocomplete", skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<AutocompleteEnum>,
    /// Sets the input's disabled field to true or false.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<models::UiText>>,
    /// MaxLength may contain the input's maximum length.
    #[serde(rename = "maxlength", skip_serializing_if = "Option::is_none")]
    pub maxlength: Option<i64>,
    /// The input's element name.
    #[serde(rename = "name")]
    pub name: String,
    /// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0.  In this struct it technically always is \"input\". text Text input Input img Image a Anchor script Script div Division
    #[serde(rename = "node_type")]
    pub node_type: NodeTypeEnum,
    /// OnClick may contain javascript which should be executed on click. This is primarily used for WebAuthn.  Deprecated: Using OnClick requires the use of eval() which is a security risk. Use OnClickTrigger instead.
    #[serde(rename = "onclick", skip_serializing_if = "Option::is_none")]
    pub onclick: Option<String>,
    /// OnClickTrigger may contain a WebAuthn trigger which should be executed on click.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
    #[serde(rename = "onclickTrigger", skip_serializing_if = "Option::is_none")]
    pub onclick_trigger: Option<OnclickTriggerEnum>,
    /// OnLoad may contain javascript which should be executed on load. This is primarily used for WebAuthn.  Deprecated: Using OnLoad requires the use of eval() which is a security risk. Use OnLoadTrigger instead.
    #[serde(rename = "onload", skip_serializing_if = "Option::is_none")]
    pub onload: Option<String>,
    /// OnLoadTrigger may contain a WebAuthn trigger which should be executed on load.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
    #[serde(rename = "onloadTrigger", skip_serializing_if = "Option::is_none")]
    pub onload_trigger: Option<OnloadTriggerEnum>,
    /// The input's pattern.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Mark this input field as required.
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// The input's element type. text InputAttributeTypeText password InputAttributeTypePassword number InputAttributeTypeNumber checkbox InputAttributeTypeCheckbox hidden InputAttributeTypeHidden email InputAttributeTypeEmail tel InputAttributeTypeTel submit InputAttributeTypeSubmit button InputAttributeTypeButton datetime-local InputAttributeTypeDateTimeLocal date InputAttributeTypeDate url InputAttributeTypeURI
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// The input's value.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<serde_json::Value>>,
}

impl UiNodeInputAttributes {
    /// InputAttributes represents the attributes of an input node
    pub fn new(disabled: bool, name: String, node_type: NodeTypeEnum, r#type: TypeEnum) -> UiNodeInputAttributes {
        UiNodeInputAttributes {
            autocomplete: None,
            disabled,
            label: None,
            maxlength: None,
            name,
            node_type,
            onclick: None,
            onclick_trigger: None,
            onload: None,
            onload_trigger: None,
            pattern: None,
            required: None,
            r#type,
            value: None,
        }
    }
}
/// The autocomplete attribute for the input. email InputAttributeAutocompleteEmail tel InputAttributeAutocompleteTel url InputAttributeAutocompleteUrl current-password InputAttributeAutocompleteCurrentPassword new-password InputAttributeAutocompleteNewPassword one-time-code InputAttributeAutocompleteOneTimeCode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutocompleteEnum {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "current-password")]
    CurrentPassword,
    #[serde(rename = "new-password")]
    NewPassword,
    #[serde(rename = "one-time-code")]
    OneTimeCode,
}

impl Default for AutocompleteEnum {
    fn default() -> AutocompleteEnum {
        Self::Email
    }
}
/// NodeType represents this node's types. It is a mirror of `node.type` and is primarily used to allow compatibility with OpenAPI 3.0.  In this struct it technically always is \"input\". text Text input Input img Image a Anchor script Script div Division
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
/// OnClickTrigger may contain a WebAuthn trigger which should be executed on click.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnclickTriggerEnum {
    #[serde(rename = "oryWebAuthnRegistration")]
    OryWebAuthnRegistration,
    #[serde(rename = "oryWebAuthnLogin")]
    OryWebAuthnLogin,
    #[serde(rename = "oryPasskeyLogin")]
    OryPasskeyLogin,
    #[serde(rename = "oryPasskeyLoginAutocompleteInit")]
    OryPasskeyLoginAutocompleteInit,
    #[serde(rename = "oryPasskeyRegistration")]
    OryPasskeyRegistration,
    #[serde(rename = "oryPasskeySettingsRegistration")]
    OryPasskeySettingsRegistration,
}

impl Default for OnclickTriggerEnum {
    fn default() -> OnclickTriggerEnum {
        Self::OryWebAuthnRegistration
    }
}
/// OnLoadTrigger may contain a WebAuthn trigger which should be executed on load.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnloadTriggerEnum {
    #[serde(rename = "oryWebAuthnRegistration")]
    OryWebAuthnRegistration,
    #[serde(rename = "oryWebAuthnLogin")]
    OryWebAuthnLogin,
    #[serde(rename = "oryPasskeyLogin")]
    OryPasskeyLogin,
    #[serde(rename = "oryPasskeyLoginAutocompleteInit")]
    OryPasskeyLoginAutocompleteInit,
    #[serde(rename = "oryPasskeyRegistration")]
    OryPasskeyRegistration,
    #[serde(rename = "oryPasskeySettingsRegistration")]
    OryPasskeySettingsRegistration,
}

impl Default for OnloadTriggerEnum {
    fn default() -> OnloadTriggerEnum {
        Self::OryWebAuthnRegistration
    }
}
/// The input's element type. text InputAttributeTypeText password InputAttributeTypePassword number InputAttributeTypeNumber checkbox InputAttributeTypeCheckbox hidden InputAttributeTypeHidden email InputAttributeTypeEmail tel InputAttributeTypeTel submit InputAttributeTypeSubmit button InputAttributeTypeButton datetime-local InputAttributeTypeDateTimeLocal date InputAttributeTypeDate url InputAttributeTypeURI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "submit")]
    Submit,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "datetime-local")]
    DatetimeLocal,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "url")]
    Url,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Text
    }
}

