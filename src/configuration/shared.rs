use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum, Serialize, Deserialize)]
pub(crate) enum SemVerKindArg {
    Node,
    Cargo,
}

impl Default for SemVerKindArg {
    fn default() -> Self {
        Self::Cargo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum, Serialize, Deserialize)]
pub(crate) enum OutputFormat {
    Human,
    Plain,
    Json,
    Yaml,
    Yml,
    Toml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Plain
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct Prefixes {
    pub patch: Vec<String>,
    pub minor: Vec<String>,
    pub major: Vec<String>,
}

impl Prefixes {
    pub(crate) fn is_empty(&self) -> bool {
        self.patch.is_empty() && self.minor.is_empty() && self.major.is_empty()
    }

    pub(crate) fn is_patch(&self, commit_prefix: impl ToString) -> bool {
        let commit_prefix = commit_prefix.to_string();
        self.patch
            .iter()
            .any(|patch_prefix| patch_prefix.eq(&commit_prefix))
    }

    pub(crate) fn is_minor(&self, commit_prefix: impl ToString) -> bool {
        let commit_prefix = commit_prefix.to_string();
        self.minor
            .iter()
            .any(|minor_prefix| minor_prefix.eq(&commit_prefix))
    }

    pub(crate) fn is_major(&self, commit_prefix: impl ToString) -> bool {
        let commit_prefix = commit_prefix.to_string();
        self.major
            .iter()
            .any(|major_prefix| major_prefix.eq(&commit_prefix))
    }
}
