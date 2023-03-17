#![allow(dead_code)] // unused GlobalDialogue

// Internal imports
use crate::core::GlobalState;

// Re-exports
pub use crate::error::Error;

// Type aliases

// A convenient type alias for Result with a custom Error type.
pub type Result<T> = std::result::Result<T, Error>;

// A type alias for InMemStorage with the GlobalState type.
pub type GlobalStorage = teloxide::dispatching::dialogue::InMemStorage<GlobalState>;

// A type alias for teloxide Dialogue with the GlobalState type.
pub type GlobalDialogue = teloxide::prelude::Dialogue<GlobalState, GlobalStorage>;

// A type alias for teloxide UpdateHandler with a custom Error type.
pub type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;
