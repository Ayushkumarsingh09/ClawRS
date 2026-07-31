use crate::run::AgentRunInput;
use clawrs_core::ClawrsResult;
use clawrs_llm::ChatMessage;

pub struct PipelineConfig {
    pub system_preamble: String,
    pub max_history_messages: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            system_preamble: "You are a ClawRS agent. Be precise, safe, and tool-aware.".into(),
            max_history_messages: 32,
        }
    }
}

pub struct PipelineState {
    pub input: AgentRunInput,
    pub messages: Vec<ChatMessage>,
    pub assistant_message: String,
    pub session_summary: String,
    pub context_notes: Vec<String>,
}

impl PipelineState {
    pub fn new(input: AgentRunInput) -> Self {
        Self {
            input,
            messages: Vec::new(),
            assistant_message: String::new(),
            session_summary: String::new(),
            context_notes: Vec::new(),
        }
    }
}

pub struct AgentPipeline {
    config: PipelineConfig,
    stages: Vec<Box<dyn crate::stages::PipelineStage>>,
}

impl Default for AgentPipeline {
    fn default() -> Self {
        Self::standard(PipelineConfig::default())
    }
}

impl AgentPipeline {
    pub fn standard(config: PipelineConfig) -> Self {
        use crate::stages::{
            ActStage, ContextStage, HistoryStage, MemoryStage, ObserveStage, PromptStage,
            SummarizeStage, ThinkStage,
        };
        let stages: Vec<Box<dyn crate::stages::PipelineStage>> = vec![
            Box::new(ContextStage),
            Box::new(HistoryStage),
            Box::new(PromptStage::new(config.system_preamble.clone())),
            Box::new(ThinkStage),
            Box::new(ActStage),
            Box::new(ObserveStage),
            Box::new(MemoryStage),
            Box::new(SummarizeStage),
        ];
        Self { config, stages }
    }

    pub async fn execute(&self, state: &mut PipelineState) -> ClawrsResult<()> {
        for stage in &self.stages[..5] {
            stage.run(state).await?;
        }
        Ok(())
    }

    pub async fn finalize(&self, state: &mut PipelineState) -> ClawrsResult<()> {
        for stage in &self.stages[5..] {
            stage.run(state).await?;
        }
        Ok(())
    }

    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }
}
