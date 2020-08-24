/*
 * CSML engine microservices
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse200 {
    #[serde(rename = "has_open")]
    pub has_open: bool,
    #[serde(rename = "conversation", skip_serializing_if = "Option::is_none")]
    pub conversation: Option<crate::models::ConversationModel>,
}

impl InlineResponse200 {
    pub fn new(has_open: bool) -> InlineResponse200 {
        InlineResponse200 {
            has_open,
            conversation: None,
        }
    }
}