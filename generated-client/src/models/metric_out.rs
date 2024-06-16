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

/// MetricOut : Metrics at the step number during the fine-tuning job. Use these metrics to assess if the training is going smoothly (loss should decrease, token accuracy should increase).

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricOut {
    #[serde(rename = "train_loss", skip_serializing_if = "Option::is_none")]
    pub train_loss: Option<f64>,
    #[serde(rename = "valid_loss", skip_serializing_if = "Option::is_none")]
    pub valid_loss: Option<f64>,
    #[serde(
        rename = "valid_mean_token_accuracy",
        skip_serializing_if = "Option::is_none"
    )]
    pub valid_mean_token_accuracy: Option<f64>,
}

impl MetricOut {
    /// Creates a new `MetricOut` instance.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A new `MetricOut` instance.
    /// Metrics at the step number during the fine-tuning job. Use these metrics to assess if the training is going smoothly (loss should decrease, token accuracy should increase).
    pub fn new() -> MetricOut {
        MetricOut {
            train_loss: None,
            valid_loss: None,
            valid_mean_token_accuracy: None,
        }
    }
}
