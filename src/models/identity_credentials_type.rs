/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.167
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// IdentityCredentialsType : and so on.

/// and so on.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentityCredentialsType {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "oidc")]
    Oidc,
    #[serde(rename = "webauthn")]
    Webauthn,
    #[serde(rename = "lookup_secret")]
    LookupSecret,

}

impl ToString for IdentityCredentialsType {
    fn to_string(&self) -> String {
        match self {
            Self::Password => String::from("password"),
            Self::Totp => String::from("totp"),
            Self::Oidc => String::from("oidc"),
            Self::Webauthn => String::from("webauthn"),
            Self::LookupSecret => String::from("lookup_secret"),
        }
    }
}




