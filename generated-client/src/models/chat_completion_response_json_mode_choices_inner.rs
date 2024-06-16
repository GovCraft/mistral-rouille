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
pub struct ChatCompletionResponseJsonModeChoicesInner {
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<models::ChatMessage>,
    #[serde(rename = "finish_reason")]
    pub finish_reason: FinishReason,
}

impl ChatCompletionResponseJsonModeChoicesInner {
    /// Creates a new `ChatCompletionResponseJsonModeChoicesInner` instance.
    ///
    /// # Arguments
    /// * `index`
    /// * `finish_reason`
    ///
    /// # Returns
    ///
    /// A new `ChatCompletionResponseJsonModeChoicesInner` instance.
    pub fn new(
        index: i32,
        finish_reason: FinishReason,
    ) -> ChatCompletionResponseJsonModeChoicesInner {
        ChatCompletionResponseJsonModeChoicesInner {
            index,
            message: None,
            finish_reason,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "model_length")]
    ModelLength,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "tool_calls")]
    ToolCalls,
}

impl Default for FinishReason {
    fn default() -> FinishReason {
        Self::Stop
    }
}
