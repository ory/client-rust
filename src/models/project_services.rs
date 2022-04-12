/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.164
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServices {
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::ProjectServiceIdentity>,
}

impl ProjectServices {
    pub fn new(identity: crate::models::ProjectServiceIdentity) -> ProjectServices {
        ProjectServices {
            identity: Box::new(identity),
        }
    }
}


