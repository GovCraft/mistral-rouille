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

/// FineTuneableModel : The name of the model to fine-tune.
/// The name of the model to fine-tune.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FineTuneableModel {
    #[serde(rename = "open-mistral-7b")]
    OpenMistral7b,
    #[serde(rename = "mistral-small-latest")]
    MistralSmallLatest,
}

impl std::fmt::Display for FineTuneableModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OpenMistral7b => write!(f, "open-mistral-7b"),
            Self::MistralSmallLatest => write!(f, "mistral-small-latest"),
        }
    }
}

impl Default for FineTuneableModel {
    fn default() -> FineTuneableModel {
        Self::OpenMistral7b
    }
}
