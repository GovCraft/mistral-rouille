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
pub struct FileSchema {
    /// The file identifier, which can be referenced in the API endpoints
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The object type, which is always `file`.
    #[serde(rename = "object")]
    pub object: String,
    /// The size of the file, in bytes.
    #[serde(rename = "bytes")]
    pub bytes: i32,
    /// The UNIX timestamp (in seconds) for when the file was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The name of the file
    #[serde(rename = "filename")]
    pub filename: String,
    /// The intended purpose of the file. Only supports `fine-tune` for now.
    #[serde(rename = "purpose")]
    pub purpose: String,
}

impl FileSchema {
    /// Creates a new `FileSchema` instance.
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
    /// A new `FileSchema` instance.
    pub fn new(
        id: uuid::Uuid,
        object: String,
        bytes: i32,
        created_at: i32,
        filename: String,
        purpose: String,
    ) -> FileSchema {
        FileSchema {
            id,
            object,
            bytes,
            created_at,
            filename,
            purpose,
        }
    }
}
