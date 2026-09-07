//! Domain model, version ledger semantics, and retrieval fusion for PaminMemory.
//!
//! This crate holds the parts that must stay stable regardless of which store or
//! index sits behind them, and it deliberately carries no heavy dependencies:
//! it is the crate edited most often, so its rebuild cost sets the development
//! loop.

pub mod filter;
pub mod id;
pub mod ledger;
pub mod version;

pub use filter::{Rejection, SensoryFilter, Verdict};
pub use id::{
    IndexJobId, ProjectId, SourceId, SourceSpanId, SourceVersionId, TopicId, TopicStateId,
};
pub use ledger::{
    FilterDecision, Project, RetrievalSignals, Source, SourceKind, SourceSpan, SourceVersion,
    Topic, TopicState,
};
pub use version::{ResolvedVersion, VersionOffset, resolve};
