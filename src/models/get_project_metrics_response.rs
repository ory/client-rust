/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.12.2
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// GetProjectMetricsResponse : Response of the getMetrics endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProjectMetricsResponse {
    /// The list of data points.
    #[serde(rename = "data")]
    pub data: Vec<models::MetricsDatapoint>,
}

impl GetProjectMetricsResponse {
    /// Response of the getMetrics endpoint
    pub fn new(data: Vec<models::MetricsDatapoint>) -> GetProjectMetricsResponse {
        GetProjectMetricsResponse {
            data,
        }
    }
}

