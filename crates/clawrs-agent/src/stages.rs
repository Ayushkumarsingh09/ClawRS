use crate::pipeline::PipelineState;
use async_trait::async_trait;
use clawrs_core::ClawrsResult;
use clawrs_llm::{ChatMessage, MessageRole};

#[async_trait]
pub trait PipelineStage: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run(&self, state: &mut PipelineState) -> ClawrsResult<()>;
}

pub struct ContextStage;

#[async_trait]
impl PipelineStage for ContextStage {
    fn name(&self) -> &'static str {
        "context"
    }

    async fn run(&self, state: &mut PipelineState) -> ClawrsResult<()> {
        state.context_notes.push(format!(
            "agent_kind={:?}, prompt_mode={:?}",
            state.input.kind, state.input.prompt_mode
        ));
        Ok(())
    }
}

pub struct HistoryStage;

#[async_trait]
impl PipelineStage for HistoryStage {
    fn name(&self) -> &'static str {
        "history"
    }

    async fn run(&self, state: &mut PipelineState) -> ClawrsResult<()> {
        state.messages.extend(state.input.prior_messages.clone());
        state
            .messages
            .push(ChatMessage::user(&state.input.user_message));
        Ok(())
    }
}

pub struct PromptStage {
    preamble: String,
}

impl PromptStage {
    pub fn new(preamble: String) -> Self {
        Self { preamble }
    }
}

#[async_trait]
impl PipelineStage for PromptStage {
    fn name(&self) -> &'static str {
        "prompt"
    }

    async fn run(&self, state: &mut PipelineState) -> ClawrsResult<()> {
        if state.input.prompt_mode.includes_identity() {
            let base = state
                .input
                .system_prompt
                .as_deref()
                .unwrap_or(self.preamble.as_str());
            let system = format!("{base}\n\nContext:\n{}", state.context_notes.join("\n"));
            state.messages.insert(0, ChatMessage::system(system));
        }
        Ok(())
    }
}

pub struct ThinkStage;

#[async_trait]
impl PipelineStage for ThinkStage {
    fn name(&self) -> &'static str {
        "think"
    }

    async fn run(&self, _state: &mut PipelineState) -> ClawrsResult<()> {
        Ok(())
    }
}

pub struct ActStage;

#[async_trait]
impl PipelineStage for ActStage {
    fn name(&self) -> &'static str {
        "act"
    }

    async fn run(&self, _state: &mut PipelineState) -> ClawrsResult<()> {
        Ok(())
    }
}

pub struct ObserveStage;

#[async_trait]
impl PipelineStage for ObserveStage {
    fn name(&self) -> &'static str {
        "observe"
    }

    async fn run(&self, _state: &mut PipelineState) -> ClawrsResult<()> {
        Ok(())
    }
}

pub struct MemoryStage;

#[async_trait]
impl PipelineStage for MemoryStage {
    fn name(&self) -> &'static str {
        "memory"
    }

    async fn run(&self, state: &mut PipelineState) -> ClawrsResult<()> {
        state.context_notes.push("memory_stage=post_run".into());
        Ok(())
    }
}

pub struct SummarizeStage;

#[async_trait]
impl PipelineStage for SummarizeStage {
    fn name(&self) -> &'static str {
        "summarize"
    }

    async fn run(&self, state: &mut PipelineState) -> ClawrsResult<()> {
        let user = state.input.user_message.chars().take(120).collect::<String>();
        let assistant = state.assistant_message.chars().take(120).collect::<String>();
        state.session_summary = format!("User: {user} | Assistant: {assistant}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::AgentKind;
    use crate::prompt::PromptMode;
    use crate::run::AgentRunInput;
    use clawrs_core::{AgentId, SessionId, TenantContext, TenantId, WorkspaceId};

    #[tokio::test]
    async fn prompt_stage_inserts_system_message() {
        let mut state = PipelineState::new(AgentRunInput {
            tenant: TenantContext::system(TenantId::new_v4(), WorkspaceId::new_v4()),
            agent_id: AgentId::new_v4(),
            session_id: SessionId::new_v4(),
            kind: AgentKind::General,
            prompt_mode: PromptMode::Full,
            model: "m".into(),
            user_message: "hi".into(),
            max_tool_rounds: 1,
            system_prompt: None,
            prior_messages: vec![],
        });
        PromptStage::new("test".into()).run(&mut state).await.unwrap();
        assert!(matches!(state.messages[0].role, MessageRole::System));
    }
}
