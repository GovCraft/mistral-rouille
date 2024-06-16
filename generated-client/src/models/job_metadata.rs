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
pub struct JobMetadata {
    /// The number of training steps to perform. A training step refers to a single update of the model weights during the fine-tuning process. This update is typically calculated using a batch of samples from the training dataset.
    #[serde(rename = "training_steps", skip_serializing_if = "Option::is_none")]
    pub training_steps: Option<i32>,
    /// The number of tokens consumed by one training step.
    #[serde(
        rename = "train_tokens_per_step",
        skip_serializing_if = "Option::is_none"
    )]
    pub train_tokens_per_step: Option<i32>,
    /// The total number of tokens in the training dataset.
    #[serde(rename = "data_tokens", skip_serializing_if = "Option::is_none")]
    pub data_tokens: Option<i32>,
    /// The total number of tokens used during the fine-tuning process.
    #[serde(rename = "train_tokens", skip_serializing_if = "Option::is_none")]
    pub train_tokens: Option<i32>,
    /// The number of complete passes through the entire training dataset.
    #[serde(rename = "epochs", skip_serializing_if = "Option::is_none")]
    pub epochs: Option<f32>,
    /// The approximated time (in seconds) for the fine-tuning process to complete.
    #[serde(
        rename = "expected_duration_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_duration_seconds: Option<i32>,
}

impl JobMetadata {
    /// Creates a new `JobMetadata` instance.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A new `JobMetadata` instance.
    pub fn new() -> JobMetadata {
        JobMetadata {
            training_steps: None,
            train_tokens_per_step: None,
            data_tokens: None,
            train_tokens: None,
            epochs: None,
            expected_duration_seconds: None,
        }
    }
}
