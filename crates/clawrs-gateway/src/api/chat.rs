use axum::{extract::State, Json};
use clawrs_agent::{AgentRunInput, PromptMode};
use clawrs_core::{SessionId, TenantContext};
use clawrs_llm::MessageRole;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
    pub tool_rounds: u32,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

pub async fn chat(
    State(state): State<AppState>,
    axum::extract::Path(session_id): axum::extract::Path<SessionId>,
    Json(body): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, ApiError> {
    let session_row = state
        .repo()
        .get_session(session_id)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?
        .ok_or_else(|| clawrs_core::ClawrsError::NotFound {
            resource: "session",
            id: session_id.to_string(),
        })?;

    let agent = state
        .repo()
        .get_agent(session_row.agent_id)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?
        .ok_or_else(|| clawrs_core::ClawrsError::NotFound {
            resource: "agent",
            id: session_row.agent_id.to_string(),
        })?;

    let prior = state
        .repo()
        .messages_as_chat(session_id)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;

    let history_len = prior.len();

    let tenant = TenantContext::system(
        state.bootstrap.tenant_id,
        state.bootstrap.workspace_id,
    );

    let output = state
        .runner
        .run(AgentRunInput {
            tenant,
            agent_id: agent.id,
            session_id,
            kind: agent.kind,
            prompt_mode: PromptMode::Full,
            model: agent.model.clone(),
            user_message: body.message.clone(),
            max_tool_rounds: 8,
            system_prompt: Some(agent.system_prompt.clone()),
            prior_messages: prior,
        })
        .await?;

    state
        .repo()
        .append_message(session_id, MessageRole::User, &body.message)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;

    state
        .repo()
        .append_message(session_id, MessageRole::Assistant, &output.assistant_message)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;

    if history_len == 0 {
        let title: String = body.message.chars().take(48).collect();
        let _ = state
            .repo()
            .update_session_title(session_id, &title)
            .await;
    }

    Ok(Json(ChatResponse {
        reply: output.assistant_message,
        tool_rounds: output.tool_rounds,
        prompt_tokens: output.prompt_tokens,
        completion_tokens: output.completion_tokens,
    }))
}
