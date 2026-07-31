//! Environment and file-backed configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub listen: String,
    pub database_url: String,
    pub static_dir: Option<PathBuf>,
    pub llm: LlmConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: LlmProviderKind,
    pub api_key: Option<String>,
    pub base_url: String,
    pub default_model: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmProviderKind {
    Mock,
    OpenAiCompatible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub api_key: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = std::env::var("CLAWRS_DATABASE_URL")
            .or_else(|_| std::env::var("DATABASE_URL"))
            .unwrap_or_else(|_| "sqlite://clawrs.db?mode=rwc".into());

        let listen = std::env::var("CLAWRS_LISTEN").unwrap_or_else(|_| "127.0.0.1:8787".into());

        let static_dir = std::env::var("CLAWRS_STATIC_DIR")
            .ok()
            .map(PathBuf::from);

        let api_key = std::env::var("CLAWRS_OPENAI_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .ok()
            .filter(|s| !s.is_empty());

        let base_url = std::env::var("CLAWRS_OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com".into());

        let default_model = std::env::var("CLAWRS_DEFAULT_MODEL")
            .unwrap_or_else(|_| "gpt-4o-mini".into());

        let provider = if api_key.is_some() {
            LlmProviderKind::OpenAiCompatible
        } else {
            LlmProviderKind::Mock
        };

        let gateway_key = std::env::var("CLAWRS_API_KEY").ok().filter(|s| !s.is_empty());

        Self {
            listen,
            database_url,
            static_dir,
            llm: LlmConfig {
                provider,
                api_key,
                base_url,
                default_model,
            },
            auth: AuthConfig {
                api_key: gateway_key,
            },
        }
    }
}
