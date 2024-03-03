use clap::Parser;
use miette::miette;
use std::path::{Path, PathBuf};

use file::{read_config_at_path, Settings};

mod args;
mod file;
mod shared;

pub(crate) use shared::OutputFormat;
pub(crate) use shared::Prefixes;
pub(crate) use shared::SemVerKindArg as SemVerKind;

const DEFAULT_REPO_PATH: &str = ".";

#[derive(Debug)]
pub(crate) struct Config {
    prefixes: Prefixes,
    output: OutputFormat,
    kind: SemVerKind,
    repo_path: PathBuf,
}

impl Config {
    pub(crate) fn new() -> miette::Result<Self> {
        let args = args::Args::parse();

        let settings: Option<miette::Result<Settings>> = args.config.map(read_config_at_path);

        match settings {
            // If config path provided, read from file
            Some(settings) => {
                if let Ok(settings) = settings {
                    Ok(Self {
                        kind: settings.kind,
                        output: settings.output,
                        prefixes: settings.prefixes,
                        repo_path: args
                            .repo
                            .map_or(Path::new(DEFAULT_REPO_PATH).to_path_buf(), |p| p),
                    })
                } else {
                    Err(miette!(
                        "Failed to read configuration file at provided path."
                    ))
                }
            }
            // Otherwise, use command line args
            None => Ok(Self {
                kind: args.kind.unwrap_or(SemVerKind::Node),
                output: args.out.unwrap_or(OutputFormat::Human),
                prefixes: Prefixes {
                    patch: args.patch_scope.unwrap_or_else(Vec::new),
                    minor: args.minor_scope.unwrap_or_else(Vec::new),
                    major: args.major_scope.unwrap_or_else(Vec::new),
                },
                repo_path: args
                    .repo
                    .map_or(Path::new(DEFAULT_REPO_PATH).to_path_buf(), |p| p),
            }),
        }
    }

    pub(crate) fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub(crate) fn semver_kind(&self) -> SemVerKind {
        self.kind
    }

    pub(crate) fn output(&self) -> OutputFormat {
        self.output
    }

    pub(crate) fn prefixes(&self) -> &Prefixes {
        &self.prefixes
    }
}
