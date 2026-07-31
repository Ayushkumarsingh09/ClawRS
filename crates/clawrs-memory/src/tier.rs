use serde::{Deserialize, Serialize};

/// Progressive memory layers (L0 working through L2 semantic).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryTier {
    Working,
    Episodic,
    Semantic,
}

impl MemoryTier {
    pub fn level(self) -> u8 {
        match self {
            Self::Working => 0,
            Self::Episodic => 1,
            Self::Semantic => 2,
        }
    }
}
