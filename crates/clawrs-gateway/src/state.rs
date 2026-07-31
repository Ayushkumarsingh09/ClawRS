use clawrs_agent::AgentRunner;
use clawrs_config::AppConfig;
use clawrs_events::InMemoryEventBus;
use clawrs_llm::LlmFactory;
use clawrs_store::{SqliteMemoryStore, StorePool, StoreRepository, WorkspaceBootstrap};
use clawrs_tools::{EchoTool, ToolRegistry};
use std::sync::Arc;

pub type ClawRunner = AgentRunner<InMemoryEventBus, SqliteMemoryStore, clawrs_llm::DynProvider>;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<StorePool>,
    pub bootstrap: WorkspaceBootstrap,
    pub runner: Arc<ClawRunner>,
    pub default_model: String,
    pub api_key: Option<String>,
}

impl AppState {
    pub async fn bootstrap(config: &AppConfig) -> anyhow::Result<Self> {
        let pool = Arc::new(StorePool::connect(&config.database_url).await?);
        pool.migrate().await?;
        let repo = StoreRepository::new(pool.as_ref());
        let bootstrap = repo.ensure_bootstrap().await?;

        let memory = Arc::new(SqliteMemoryStore::new(pool.inner().clone()));
        let events = Arc::new(InMemoryEventBus::default());
        let provider = Arc::new(LlmFactory::from_config(
            config.llm.provider,
            config.llm.api_key.clone(),
            config.llm.base_url.clone(),
            config.llm.default_model.clone(),
        )?);

        let tools = Arc::new(ToolRegistry::new());
        tools.register(Arc::new(EchoTool));

        let runner = Arc::new(AgentRunner::new(events, memory, provider, tools));

        Ok(Self {
            pool,
            bootstrap,
            runner,
            default_model: config.llm.default_model.clone(),
            api_key: config.auth.api_key.clone(),
        })
    }

    pub fn repo(&self) -> StoreRepository<'_> {
        StoreRepository::new(self.pool.as_ref())
    }
}
