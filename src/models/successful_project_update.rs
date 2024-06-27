/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.12.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulProjectUpdate {
    #[serde(rename = "project")]
    pub project: Box<models::Project>,
    /// Import Warnings  Not all configuration items can be imported to the Ory Network. For example, setting the port does not make sense because the Ory Network provides the runtime and networking.  This field contains warnings where configuration keys were found but can not be imported. These keys will be ignored by the Ory Network. This field will help you understand why certain configuration keys might not be respected!
    #[serde(rename = "warnings")]
    pub warnings: Vec<models::Warning>,
}

impl SuccessfulProjectUpdate {
    pub fn new(project: models::Project, warnings: Vec<models::Warning>) -> SuccessfulProjectUpdate {
        SuccessfulProjectUpdate {
            project: Box::new(project),
            warnings,
        }
    }
}

