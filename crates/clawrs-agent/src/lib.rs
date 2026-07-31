//! Agent orchestration: specs, prompt modes, and the 8-stage pipeline.

pub mod kind;
pub mod pipeline;
pub mod prompt;
pub mod run;
pub mod stages;

pub use kind::AgentKind;
pub use pipeline::{AgentPipeline, PipelineConfig};
pub use prompt::PromptMode;
pub use run::{AgentRunInput, AgentRunOutput, AgentRunner};
