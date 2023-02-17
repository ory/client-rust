/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.16
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// TokenPaginationResponseHeaders : The `Link` HTTP header contains multiple links (`first`, `next`, `last`, `previous`) formatted as: `<https://{project-slug}.projects.oryapis.com/admin/clients?page_size={limit}&page_token={offset}>; rel=\"{page}\"`  For details on pagination please head over to the [pagination documentation](https://www.ory.sh/docs/ecosystem/api-design#pagination).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenPaginationResponseHeaders {
    /// The Link HTTP Header  The `Link` header contains a comma-delimited list of links to the following pages:  first: The first page of results. next: The next page of results. prev: The previous page of results. last: The last page of results.  Pages are omitted if they do not exist. For example, if there is no next page, the `next` link is omitted. Examples:  </clients?page_size=5&page_token=0>; rel=\"first\",</clients?page_size=5&page_token=15>; rel=\"next\",</clients?page_size=5&page_token=5>; rel=\"prev\",</clients?page_size=5&page_token=20>; rel=\"last\"
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The X-Total-Count HTTP Header  The `X-Total-Count` header contains the total number of items in the collection.
    #[serde(rename = "x-total-count", skip_serializing_if = "Option::is_none")]
    pub x_total_count: Option<i64>,
}

impl Default for TokenPaginationResponseHeaders {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenPaginationResponseHeaders {
    /// The `Link` HTTP header contains multiple links (`first`, `next`, `last`, `previous`) formatted as: `<https://{project-slug}.projects.oryapis.com/admin/clients?page_size={limit}&page_token={offset}>; rel=\"{page}\"`  For details on pagination please head over to the [pagination documentation](https://www.ory.sh/docs/ecosystem/api-design#pagination).
    pub fn new() -> TokenPaginationResponseHeaders {
        TokenPaginationResponseHeaders {
                link: None,
                x_total_count: None,
        }
    }
}


