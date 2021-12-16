#![doc = include_str!("../README.md")]

mod event;
mod polling;
mod query;

pub use event::EventInputHandler;
pub use polling::PollingInputHandler;
pub use query::QueryInputHandler;
