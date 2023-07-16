use anyhow::Context;
use clap::ValueEnum;
#[cfg(test)]
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub(crate) enum OutputFormat {
    Human,
    Plain,
    Json,
    Yaml,
    Yml,
    Toml,
}

pub(crate) fn stringify(
    format: OutputFormat,
    next_version: impl Into<String>,
    previous_version: impl Into<String>,
) -> anyhow::Result<String> {
    let previous_version = previous_version.into();
    let output = match format {
        OutputFormat::Human => {
            format!(
                "Previous version: {}\nNext version: {}",
                previous_version,
                next_version.into()
            )
        }
        OutputFormat::Plain => next_version.into().to_string(),
        OutputFormat::Json => (serde_json::to_string_pretty(&Output {
            previous_version: previous_version.to_string(),
            next_version: next_version.into(),
        })
        .context("Failed to serialize output")?)
        .to_string(),
        OutputFormat::Yaml | OutputFormat::Yml => (serde_yaml::to_string(&Output {
            previous_version: previous_version.to_string(),
            next_version: next_version.into(),
        })
        .context("Failed to serialize output")?)
        .to_string(),
        OutputFormat::Toml => (toml::to_string(&Output {
            previous_version: previous_version.to_string(),
            next_version: next_version.into(),
        })
        .context("Failed to serialize output")?)
        .to_string(),
    };
    Ok(output)
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, Eq, PartialEq))]
struct Output {
    previous_version: String,
    next_version: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_json_output() {
        let next_version = "1.1.0";
        let previous_version = "1.0.0";
        let output = stringify(OutputFormat::Json, next_version, previous_version).unwrap();
        let parsed: Output = serde_json::from_str(&output).unwrap();
        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
            }
        );
    }

    #[test]
    fn test_yaml_output() {
        let next_version = "1.1.0";
        let previous_version = "1.0.0";
        let output = stringify(OutputFormat::Yaml, next_version, previous_version).unwrap();
        let parsed: Output = serde_yaml::from_str(&output).unwrap();
        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
            }
        );
    }

    #[test]
    fn test_yml_output() {
        let next_version = "1.1.0";
        let previous_version = "1.0.0";
        let output = stringify(OutputFormat::Yml, next_version, previous_version).unwrap();
        let parsed: Output = serde_yaml::from_str(&output).unwrap();
        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
            }
        );
    }

    #[test]
    fn test_toml_output() {
        let next_version = "1.1.0";
        let previous_version = "1.0.0";
        let output = stringify(OutputFormat::Toml, next_version, previous_version).unwrap();
        let parsed: Output = toml::from_str(&output).unwrap();
        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
            }
        );
    }

    #[test]
    fn test_human_output() {
        let next_version = "1.1.0";
        let previous_version = "1.0.0";
        let output = stringify(OutputFormat::Human, next_version, previous_version).unwrap();
        assert_eq!(
            output,
            r"Previous version: 1.0.0
Next version: 1.1.0"
        );
    }

    #[test]
    fn test_plain_output() {
        let next_version = "1.1.0";
        let previous_version = "1.0.0";
        let output = stringify(OutputFormat::Plain, next_version, previous_version).unwrap();
        assert_eq!(output, "1.1.0");
    }
}
