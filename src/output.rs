use anyhow::Context;
use clap::ValueEnum;
use serde::Serialize;
use std::fmt::Display;

use crate::variant::AbstractVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub(crate) enum OutputFormat {
    Human,
    Plain,
    Json,
    Yaml,
}

pub(crate) fn print<V>(
    format: OutputFormat,
    next_version: impl Into<String>,
    previous_version: &V,
) -> anyhow::Result<()>
where
    V: AbstractVersion + Display,
{
    match format {
        OutputFormat::Human => {
            println!("Previous version: {}", previous_version);
            println!("Next version: {}", next_version.into());
        }
        OutputFormat::Plain => {
            println!("{}", next_version.into());
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&Output {
                    previous_version: previous_version.to_string(),
                    next_version: next_version.into(),
                })
                .context("Failed to serialize output")?
            );
        }
        OutputFormat::Yaml => {
            println!(
                "{}",
                serde_yaml::to_string(&Output {
                    previous_version: previous_version.to_string(),
                    next_version: next_version.into(),
                })
                .context("Failed to serialize output")?
            );
        }
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct Output {
    previous_version: String,
    next_version: String,
}
