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
pub struct EmbeddingRequest {
    /// The ID of the model to use for this request.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The list of strings to embed.
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<String>>,
    /// The format of the output data.
    #[serde(rename = "encoding_format", skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,
}

impl EmbeddingRequest {
    /// Creates a new `EmbeddingRequest` instance.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A new `EmbeddingRequest` instance.
    pub fn new() -> EmbeddingRequest {
        EmbeddingRequest {
            model: None,
            input: None,
            encoding_format: None,
        }
    }
}
/// The format of the output data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EncodingFormat {
    #[serde(rename = "float")]
    Float,
}

impl Default for EncodingFormat {
    fn default() -> EncodingFormat {
        Self::Float
    }
}
