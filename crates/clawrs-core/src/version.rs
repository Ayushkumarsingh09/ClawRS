/// Semantic version of the ClawRS workspace (see root `Cargo.toml`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Cargo profile used at build time.
pub const BUILD_PROFILE: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};
