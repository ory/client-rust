/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberInvite {
    /// The Project's Revision Creation Date
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The invite's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The invitee's email
    #[serde(rename = "invitee_email")]
    pub invitee_email: String,
    #[serde(rename = "invitee_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub invitee_id: Option<Option<String>>,
    /// The invite owner's email Usually the project's owner email
    #[serde(rename = "owner_email")]
    pub owner_email: String,
    /// The invite owner's ID Usually the project's owner
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    #[serde(rename = "project_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Option<String>>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<String>>,
    /// The invite's status Keeps track of the invites status such as pending, accepted, declined, expired pending PENDING accepted ACCEPTED declined DECLINED expired EXPIRED cancelled CANCELLED removed REMOVED
    #[serde(rename = "status")]
    pub status: StatusEnum,
    /// Last Time Project's Revision was Updated
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "workspace_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Option<String>>,
}

impl MemberInvite {
    pub fn new(created_at: String, id: String, invitee_email: String, owner_email: String, owner_id: String, status: StatusEnum, updated_at: String) -> MemberInvite {
        MemberInvite {
            created_at,
            id,
            invitee_email,
            invitee_id: None,
            owner_email,
            owner_id,
            project_id: None,
            role: None,
            status,
            updated_at,
            workspace_id: None,
        }
    }
}
/// The invite's status Keeps track of the invites status such as pending, accepted, declined, expired pending PENDING accepted ACCEPTED declined DECLINED expired EXPIRED cancelled CANCELLED removed REMOVED
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "removed")]
    Removed,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Pending
    }
}

