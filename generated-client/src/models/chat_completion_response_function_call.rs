/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * Build date: 2024-06-15T23:41:00.377209-06:00[America/Mexico_City]
 * Generated using: https://openapi-generator.tech
 * Open API specification v0.0.2 provided by Mistral @ https://docs.mistral.ai/redocusaurus/plugin-redoc-0.yaml
 * Custom generation templates by [GovCraft Ai](https://www.govcraft.ai)
 * Contact: roland@govcraft.ai
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseFunctionCall {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i32>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<models::Choices>>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<models::Usage>,
}

impl ChatCompletionResponseFunctionCall {
    /// Creates a new `ChatCompletionResponseFunctionCall` instance.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A new `ChatCompletionResponseFunctionCall` instance.
    pub fn new() -> ChatCompletionResponseFunctionCall {
        ChatCompletionResponseFunctionCall {
            id: None,
            object: None,
            created: None,
            model: None,
            choices: None,
            usage: None,
        }
    }
}
