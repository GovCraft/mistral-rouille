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
pub struct RetrieveFileOut {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "bytes")]
    pub bytes: i32,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "purpose", deserialize_with = "Option::deserialize")]
    pub purpose: Option<serde_json::Value>,
}

impl RetrieveFileOut {
    /// Creates a new `RetrieveFileOut` instance.
    ///
    /// # Arguments
    /// * `id`
    /// * `object`
    /// * `bytes`
    /// * `created_at`
    /// * `filename`
    /// * `purpose`
    ///
    /// # Returns
    ///
    /// A new `RetrieveFileOut` instance.
    pub fn new(
        id: uuid::Uuid,
        object: String,
        bytes: i32,
        created_at: i32,
        filename: String,
        purpose: Option<serde_json::Value>,
    ) -> RetrieveFileOut {
        RetrieveFileOut {
            id,
            object,
            bytes,
            created_at,
            filename,
            purpose,
        }
    }
}
