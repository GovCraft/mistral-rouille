use std::sync::Once;

use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use mistral_rouille::apis::configuration::Configuration;
use mistral_rouille::apis::default_api::{create_chat_completion, CreateChatCompletionParams};
use mistral_rouille::models::json_mode_messages::Role::User;
use mistral_rouille::models::{ChatRequest, JsonModeMessages};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_codestral_connectivity() -> anyhow::Result<()> {
    init_tracing();

    let mut config = Configuration::new();

    //will look for MISTRAL_API_KEY env variable for access token
    config.base_path = "https:/codestral.mistral.ai/v1".to_string();

    let mut user_prompt = JsonModeMessages::new();
    user_prompt.role = Some(User);
    user_prompt.content = Some(
        "Output a recursive implementation of fibonacci calculation with memoization.".to_string(),
    );

    let params = CreateChatCompletionParams {
        chat_request: ChatRequest {
            model: "codestral-latest".to_string(),
            messages: vec![user_prompt],
            temperature: Some(0.0),
            top_p: None,
            max_tokens: Some(100),
            stream: Some(false),
            safe_prompt: None,
            random_seed: None,
            tools: None,
            tool_choice: None,
            response_format: None,
        },
    };
    let chat = create_chat_completion(&config, params).await;
    assert!(chat.is_ok());
    tracing::info!(content = chat?.content);
    Ok(())
}

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        // Define an environment filter to suppress logs from specific functions
        let filter = EnvFilter::new("")
            .add_directive("mistral-rouille=trace".parse().unwrap())
            .add_directive("hyper_util=off".parse().unwrap())
            .add_directive("reqwest=off".parse().unwrap())
            .add_directive(Level::TRACE.into());
        // Set global log level to TRACE
        let subscriber = FmtSubscriber::builder()
            .with_span_events(FmtSpan::NONE)
            .with_max_level(Level::TRACE)
            .compact()
            .pretty()
            .with_line_number(true)
            //            .without_time()
            .with_env_filter(filter)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    });
}
