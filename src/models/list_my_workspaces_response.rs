/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.6
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListMyWorkspacesResponse {
    #[serde(rename = "has_next_page")]
    pub has_next_page: bool,
    #[serde(rename = "next_page_token")]
    pub next_page_token: String,
    #[serde(rename = "workspaces")]
    pub workspaces: Vec<crate::models::Workspace>,
}


impl ListMyWorkspacesResponse {
    pub fn new(has_next_page: bool, next_page_token: String, workspaces: Vec<crate::models::Workspace>) -> ListMyWorkspacesResponse {
        ListMyWorkspacesResponse {
                has_next_page,
                next_page_token,
                workspaces,
        }
    }
}


