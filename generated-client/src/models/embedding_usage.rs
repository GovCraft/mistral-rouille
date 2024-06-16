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
pub struct EmbeddingUsage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i32,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}

impl EmbeddingUsage {
    /// Creates a new `EmbeddingUsage` instance.
    ///
    /// # Arguments
    /// * `prompt_tokens`
    /// * `total_tokens`
    ///
    /// # Returns
    ///
    /// A new `EmbeddingUsage` instance.
    pub fn new(prompt_tokens: i32, total_tokens: i32) -> EmbeddingUsage {
        EmbeddingUsage {
            prompt_tokens,
            total_tokens,
        }
    }
}
