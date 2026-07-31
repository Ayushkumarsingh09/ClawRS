use serde::{Deserialize, Serialize};

/// Prompt assembly density (inspired by GoClaw's 4-mode system, extended for ClawRS).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptMode {
    #[default]
    Full,
    Task,
    Minimal,
    None,
}

impl PromptMode {
    pub fn includes_identity(self) -> bool {
        matches!(self, Self::Full | Self::Task)
    }

    pub fn includes_tools(self) -> bool {
        !matches!(self, Self::None)
    }
}
