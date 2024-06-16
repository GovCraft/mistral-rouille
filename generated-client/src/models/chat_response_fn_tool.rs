/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * Build date: 2024-06-15T23:59:31.144857-06:00[America/Mexico_City]
 * Generated using: https://openapi-generator.tech
 * Open API specification v0.0.2 provided by Mistral @ https://docs.mistral.ai/redocusaurus/plugin-redoc-0.yaml
 * Custom generation templates by [GovCraft Ai](https://www.govcraft.ai)
 * Contact: roland@govcraft.ai
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatResponseFnTool {
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<models::ChatFnTools>,
}

impl ChatResponseFnTool {
    /// Creates a new `ChatResponseFnTool` instance.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A new `ChatResponseFnTool` instance.
    pub fn new() -> ChatResponseFnTool {
        ChatResponseFnTool { function: None }
    }
}
