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
pub struct CheckpointOut {
    #[serde(rename = "metrics")]
    pub metrics: models::MetricOut,
    /// The step number that the checkpoint was created at.
    #[serde(rename = "step_number")]
    pub step_number: i32,
    /// The UNIX timestamp (in seconds) for when the checkpoint was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
}

impl CheckpointOut {
    /// Creates a new `CheckpointOut` instance.
    ///
    /// # Arguments
    /// * `metrics`
    /// * `step_number`
    /// * `created_at`
    ///
    /// # Returns
    ///
    /// A new `CheckpointOut` instance.
    pub fn new(metrics: models::MetricOut, step_number: i32, created_at: i32) -> CheckpointOut {
        CheckpointOut {
            metrics,
            step_number,
            created_at,
        }
    }
}
