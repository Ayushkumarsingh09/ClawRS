use clap::{Parser, Subcommand};
use clawrs_config::AppConfig;
use clawrs_core::VERSION;
use clawrs_gateway::{attach_static, build_router};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "clawrs", about = "ClawRS — Rust-native AI agent platform", version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the HTTP gateway and web console
    Serve {
        #[arg(long, env = "CLAWRS_LISTEN")]
        listen: Option<SocketAddr>,
        #[arg(long, env = "CLAWRS_STATIC_DIR")]
        static_dir: Option<PathBuf>,
    },
    /// Print build and environment diagnostics
    Doctor,
    /// Run a single agent turn locally
    Chat {
        message: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("clawrs=info".parse()?))
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { listen, static_dir } => serve(listen, static_dir).await?,
        Commands::Doctor => doctor().await?,
        Commands::Chat { message } => chat_once(message).await?,
    }
    Ok(())
}

async fn serve(listen: Option<SocketAddr>, static_dir: Option<PathBuf>) -> anyhow::Result<()> {
    let mut config = AppConfig::from_env();
    if let Some(dir) = static_dir {
        config.static_dir = Some(dir);
    }
    let listen: SocketAddr = listen
        .or_else(|| config.listen.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:8787".parse().unwrap());

    let mut router = build_router(&config).await?;
    if let Some(dir) = config.static_dir.clone() {
        router = attach_static(router, &dir);
        tracing::info!(path = %dir.display(), "serving web console");
    } else {
        let default = PathBuf::from("web/dist");
        if default.join("index.html").exists() {
            router = attach_static(router, &default);
            tracing::info!("serving web console from web/dist");
        }
    }

    router = router.layer(TraceLayer::new_for_http());

    match config.llm.provider {
        clawrs_config::LlmProviderKind::Mock => {
            tracing::warn!(
                "no LLM API key configured — using mock provider (set OPENAI_API_KEY or CLAWRS_OPENAI_API_KEY)"
            );
        }
        clawrs_config::LlmProviderKind::OpenAiCompatible => {
            tracing::info!(model = %config.llm.default_model, "LLM provider ready");
        }
    }

    tracing::info!(%listen, "ClawRS gateway listening");
    let listener = tokio::net::TcpListener::bind(listen).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn doctor() -> anyhow::Result<()> {
    let config = AppConfig::from_env();
    println!("ClawRS {}", VERSION);
    println!("profile: {}", clawrs_core::BUILD_PROFILE);
    println!("database: {}", config.database_url);
    println!("llm provider: {:?}", config.llm.provider);
    println!("default model: {}", config.llm.default_model);
    println!("api key auth: {}", if config.auth.api_key.is_some() { "enabled" } else { "disabled" });
    Ok(())
}

async fn chat_once(message: String) -> anyhow::Result<()> {
    use clawrs_agent::{AgentKind, AgentRunInput, AgentRunner, PromptMode};
    use clawrs_core::{SessionId, TenantContext};
    use clawrs_events::InMemoryEventBus;
    use clawrs_llm::LlmFactory;
    use clawrs_memory::InMemoryMemoryStore;
    use clawrs_tools::{EchoTool, ToolRegistry};
    use std::sync::Arc;

    let config = AppConfig::from_env();
    let pool = clawrs_store::StorePool::connect(&config.database_url).await?;
    pool.migrate().await?;
    let bootstrap = clawrs_store::StoreRepository::new(&pool).ensure_bootstrap().await?;

    let events = Arc::new(InMemoryEventBus::default());
    let memory = Arc::new(InMemoryMemoryStore::default());
    let provider = Arc::new(LlmFactory::from_config(
        config.llm.provider,
        config.llm.api_key,
        config.llm.base_url,
        config.llm.default_model,
    )?);
    let tools = Arc::new(ToolRegistry::new());
    tools.register(Arc::new(EchoTool));

    let runner = AgentRunner::new(events, memory, provider, tools);
    let out = runner
        .run(AgentRunInput {
            tenant: TenantContext::system(bootstrap.tenant_id, bootstrap.workspace_id),
            agent_id: bootstrap.default_agent_id,
            session_id: SessionId::new_v4(),
            kind: AgentKind::General,
            prompt_mode: PromptMode::Full,
            model: config.llm.default_model,
            user_message: message,
            max_tool_rounds: 3,
            system_prompt: None,
            prior_messages: vec![],
        })
        .await?;

    println!("{}", out.assistant_message);
    Ok(())
}
