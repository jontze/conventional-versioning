use anyhow::Context;
#[cfg(test)]
use serde::Deserialize;
use serde::Serialize;

use crate::{args::OutputFormat, conventional::AnalyzeResult, repo::Commit};

pub(crate) fn stringify(
    format: OutputFormat,
    next_version: impl Into<String>,
    previous_version: impl Into<String>,
    analyze_result: AnalyzeResult,
) -> anyhow::Result<String> {
    let amount_breaking = analyze_result.breaking.len();
    let amount_features = analyze_result.features.len();
    let amount_fixes = analyze_result.fixes.len();
    let amount_unclassified = analyze_result.others.len();
    let commits: CommitOutput = analyze_result.into();

    let previous_version = previous_version.into();
    let output = match format {
        OutputFormat::Human => {
            format!(
                "Previous version: {}\nNext version: {}\nBreaking: {}\nFeatures: {}\nFixes: {}\nUnclassified: {}",
                previous_version,
                next_version.into(),
                amount_breaking,
                amount_features,
                amount_fixes,
                amount_unclassified,
            )
        }
        OutputFormat::Plain => next_version.into().to_string(),
        OutputFormat::Json => (serde_json::to_string_pretty(&Output {
            previous_version: previous_version.to_string(),
            next_version: next_version.into(),

            commits,
        })
        .map_err(|err| {
            eprintln!("Failed to serialize output: {}", err);
            err
        })
        .context("Failed to serialize output")?)
        .to_string(),
        OutputFormat::Yaml | OutputFormat::Yml => (serde_yaml::to_string(&Output {
            previous_version: previous_version.to_string(),
            next_version: next_version.into(),
            commits,
        })
        .context("Failed to serialize output")?)
        .to_string(),
        OutputFormat::Toml => (toml::to_string(&Output {
            previous_version: previous_version.to_string(),
            next_version: next_version.into(),
            commits,
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
    commits: CommitOutput,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize, Eq, PartialEq))]
struct CommitOutput {
    amount_breaking: usize,
    amount_features: usize,
    amount_fixes: usize,
    amount_unclassified: usize,
    breaking: Vec<Commit>,
    features: Vec<Commit>,
    fixes: Vec<Commit>,
    unclassified: Vec<Commit>,
}

impl From<AnalyzeResult> for CommitOutput {
    fn from(result: AnalyzeResult) -> Self {
        Self {
            amount_breaking: result.breaking.len(),
            amount_features: result.features.len(),
            amount_fixes: result.fixes.len(),
            amount_unclassified: result.others.len(),
            breaking: result.breaking,
            features: result.features,
            fixes: result.fixes,
            unclassified: result.others,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_json_output() {
        let mock_commit = Commit {
            id: "1234567890".to_string(),
            message: "Mock commit".to_string(),
        };
        let mock_result = AnalyzeResult {
            breaking: vec![mock_commit.clone()],
            features: vec![mock_commit.clone()],
            fixes: vec![mock_commit.clone()],
            others: vec![mock_commit.clone()],
        };
        let next_version = "1.1.0";
        let previous_version = "1.0.0";

        let output = stringify(
            OutputFormat::Json,
            next_version,
            previous_version,
            mock_result,
        )
        .unwrap();

        let parsed: Output = serde_json::from_str(&output).unwrap();
        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
                commits: CommitOutput {
                    amount_breaking: 1,
                    amount_features: 1,
                    amount_fixes: 1,
                    amount_unclassified: 1,
                    breaking: vec![mock_commit.clone()],
                    features: vec![mock_commit.clone()],
                    fixes: vec![mock_commit.clone()],
                    unclassified: vec![mock_commit.clone()]
                },
            }
        );
    }

    #[test]
    fn test_yaml_output() {
        let mock_commit = Commit {
            id: "1234567890".to_string(),
            message: "Mock commit".to_string(),
        };
        let mock_result = AnalyzeResult {
            breaking: vec![mock_commit.clone()],
            features: vec![mock_commit.clone()],
            fixes: vec![mock_commit.clone()],
            others: vec![mock_commit.clone()],
        };
        let next_version = "1.1.0";
        let previous_version = "1.0.0";

        let output = stringify(
            OutputFormat::Yaml,
            next_version,
            previous_version,
            mock_result,
        )
        .unwrap();
        let parsed: Output = serde_yaml::from_str(&output).unwrap();

        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
                commits: CommitOutput {
                    amount_breaking: 1,
                    amount_features: 1,
                    amount_fixes: 1,
                    amount_unclassified: 1,
                    breaking: vec![mock_commit.clone()],
                    features: vec![mock_commit.clone()],
                    fixes: vec![mock_commit.clone()],
                    unclassified: vec![mock_commit.clone()]
                },
            }
        );
    }

    #[test]
    fn test_yml_output() {
        let mock_commit = Commit {
            id: "1234567890".to_string(),
            message: "Mock commit".to_string(),
        };
        let mock_result = AnalyzeResult {
            breaking: vec![mock_commit.clone()],
            features: vec![mock_commit.clone()],
            fixes: vec![mock_commit.clone()],
            others: vec![mock_commit.clone()],
        };
        let next_version = "1.1.0";
        let previous_version = "1.0.0";

        let output = stringify(
            OutputFormat::Yml,
            next_version,
            previous_version,
            mock_result,
        )
        .unwrap();
        let parsed: Output = serde_yaml::from_str(&output).unwrap();

        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
                commits: CommitOutput {
                    amount_breaking: 1,
                    amount_features: 1,
                    amount_fixes: 1,
                    amount_unclassified: 1,
                    breaking: vec![mock_commit.clone()],
                    features: vec![mock_commit.clone()],
                    fixes: vec![mock_commit.clone()],
                    unclassified: vec![mock_commit.clone()]
                },
            }
        );
    }

    #[test]
    fn test_toml_output() {
        let mock_commit = Commit {
            id: "1234567890".to_string(),
            message: "Mock commit".to_string(),
        };
        let mock_result = AnalyzeResult {
            breaking: vec![mock_commit.clone()],
            features: vec![mock_commit.clone()],
            fixes: vec![mock_commit.clone()],
            others: vec![mock_commit.clone()],
        };
        let next_version = "1.1.0";
        let previous_version = "1.0.0";

        let output = stringify(
            OutputFormat::Toml,
            next_version,
            previous_version,
            mock_result,
        )
        .unwrap();
        let parsed: Output = toml::from_str(&output).unwrap();

        assert_eq!(
            parsed,
            Output {
                previous_version: previous_version.to_string(),
                next_version: next_version.to_string(),
                commits: CommitOutput {
                    amount_breaking: 1,
                    amount_features: 1,
                    amount_fixes: 1,
                    amount_unclassified: 1,
                    breaking: vec![mock_commit.clone()],
                    features: vec![mock_commit.clone()],
                    fixes: vec![mock_commit.clone()],
                    unclassified: vec![mock_commit.clone()]
                },
            }
        );
    }

    #[test]
    fn test_human_output() {
        let mock_commit = Commit {
            id: "1234567890".to_string(),
            message: "Mock commit".to_string(),
        };
        let mock_result = AnalyzeResult {
            breaking: vec![mock_commit.clone()],
            features: vec![mock_commit.clone()],
            fixes: vec![mock_commit.clone()],
            others: vec![mock_commit.clone()],
        };
        let next_version = "1.1.0";
        let previous_version = "1.0.0";

        let output = stringify(
            OutputFormat::Human,
            next_version,
            previous_version,
            mock_result,
        )
        .unwrap();

        assert_eq!(
            output,
            r"Previous version: 1.0.0
Next version: 1.1.0
Breaking: 1
Features: 1
Fixes: 1
Unclassified: 1"
        );
    }

    #[test]
    fn test_plain_output() {
        let mock_commit = Commit {
            id: "1234567890".to_string(),
            message: "Mock commit".to_string(),
        };
        let mock_result = AnalyzeResult {
            breaking: vec![mock_commit.clone()],
            features: vec![mock_commit.clone()],
            fixes: vec![mock_commit.clone()],
            others: vec![mock_commit.clone()],
        };
        let next_version = "1.1.0";
        let previous_version = "1.0.0";

        let output = stringify(
            OutputFormat::Plain,
            next_version,
            previous_version,
            mock_result,
        )
        .unwrap();
        assert_eq!(output, "1.1.0");
    }
}
