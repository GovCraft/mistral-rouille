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

/// TrainingParameters : The fine-tuning hyperparameter settings used in a fine-tune job.

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrainingParameters {
    /// The number of training steps to perform. A training step refers to a single update of the model weights during the fine-tuning process. This update is typically calculated using a batch of samples from the training dataset.
    #[serde(rename = "training_steps")]
    pub training_steps: u32,
    /// A parameter describing how much to adjust the pre-trained model's weights in response to the estimated error each time the weights are updated during the fine-tuning process.
    #[serde(rename = "learning_rate", skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f64>,
}

impl TrainingParameters {
    /// Creates a new `TrainingParameters` instance.
    ///
    /// # Arguments
    /// * `training_steps`
    ///
    /// # Returns
    ///
    /// A new `TrainingParameters` instance.
    /// The fine-tuning hyperparameter settings used in a fine-tune job.
    pub fn new(training_steps: u32) -> TrainingParameters {
        TrainingParameters {
            training_steps,
            learning_rate: None,
        }
    }
}
