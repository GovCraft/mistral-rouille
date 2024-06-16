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
pub struct EmbeddingResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<models::EmbeddingData>,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "usage")]
    pub usage: models::EmbeddingUsage,
}

impl EmbeddingResponse {
    /// Creates a new `EmbeddingResponse` instance.
    ///
    /// # Arguments
    /// * `id`
    /// * `object`
    /// * `data`
    /// * `model`
    /// * `usage`
    ///
    /// # Returns
    ///
    /// A new `EmbeddingResponse` instance.
    pub fn new(
        id: String,
        object: String,
        data: Vec<models::EmbeddingData>,
        model: String,
        usage: models::EmbeddingUsage,
    ) -> EmbeddingResponse {
        EmbeddingResponse {
            id,
            object,
            data,
            model,
            usage,
        }
    }
}
