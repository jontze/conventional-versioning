use anyhow::Context;
#[cfg(test)]
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

use crate::repo::Commit;
use crate::variant::AbstractVersion;

#[cfg_attr(test, automock)]
pub(crate) trait CommitExt {
    fn is_breaking(&self) -> bool;
    fn is_minor(&self) -> bool;
    fn is_patch(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub(crate) enum CommitType {
    /// Commit that has a ! after the type or a BREAKING CHANGE: in its body`
    Breaking,
    /// Commit that starts with `feat:`
    Feature,
    /// Commit that starts with `fix:`
    Fix,
    /// Commit that starts with `build:`
    Build,
    /// Commit that starts with `ci:`
    Ci,
    /// Commit that starts with `docs:`
    Docs,
    /// Commit that starts with `perf:`
    Perf,
    /// Commit that starts with `refactor:`
    Refactor,
    /// Commit that starts with `style:`
    Style,
    /// Commit that starts with `revert:`
    Revert,
    /// Any commit that doesn't fit in the above categories
    Other(String),
}

impl CommitExt for CommitType {
    fn is_breaking(&self) -> bool {
        matches!(self, Self::Breaking)
    }

    fn is_minor(&self) -> bool {
        matches!(self, Self::Feature)
    }

    fn is_patch(&self) -> bool {
        matches!(self, Self::Fix)
    }
}

impl TryFrom<&str> for CommitType {
    type Error = anyhow::Error;

    fn try_from(commit_string: &str) -> Result<Self, Self::Error> {
        let commit_prefix = commit_string
            .split(':')
            .next()
            .context("Unable to split commit message by ':'")?;
        if commit_prefix.contains('!') || commit_string.contains("BREAKING CHANGE:") {
            return Ok(CommitType::Breaking);
        }
        let commit_type_string = if commit_prefix.contains('(') && commit_prefix.contains(')') {
            commit_prefix
                .split('(')
                .next()
                .context("Unable to split commit message by scope")?
        } else {
            commit_prefix
        };

        match commit_type_string {
            "feat" => Ok(Self::Feature),
            "fix" => Ok(Self::Fix),
            "build" => Ok(Self::Build),
            "ci" => Ok(Self::Ci),
            "docs" => Ok(Self::Docs),
            "perf" => Ok(Self::Perf),
            "refactor" => Ok(Self::Refactor),
            "style" => Ok(Self::Style),
            "revert" => Ok(Self::Revert),
            _ => Ok(Self::Other(commit_string.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AnalyzeResult {
    pub(crate) breaking: Vec<Commit>,
    pub(crate) features: Vec<Commit>,
    pub(crate) fixes: Vec<Commit>,
    pub(crate) others: Vec<Commit>,
}

impl CommitExt for AnalyzeResult {
    fn is_breaking(&self) -> bool {
        !self.breaking.is_empty()
    }

    fn is_minor(&self) -> bool {
        !self.features.is_empty()
    }

    fn is_patch(&self) -> bool {
        !self.fixes.is_empty()
    }
}

pub(crate) fn analyze(commits: Vec<Commit>) -> anyhow::Result<AnalyzeResult> {
    let mut result = AnalyzeResult {
        breaking: Vec::new(),
        features: Vec::new(),
        fixes: Vec::new(),
        others: Vec::new(),
    };
    for commit in commits {
        match CommitType::try_from(commit.message.as_str())? {
            CommitType::Breaking => {
                result.breaking.push(commit);
            }
            CommitType::Feature => {
                result.features.push(commit);
            }
            CommitType::Fix => {
                result.fixes.push(commit);
            }
            _ => {
                result.others.push(commit);
            }
        }
    }
    Ok(result)
}

pub(crate) fn suggest_next_version<V>(
    version: &mut V,
    conventional_analyze: &dyn CommitExt,
) -> String
where
    V: AbstractVersion + Display + ?Sized,
{
    match version.major() {
        0 => suggest_for_unstable(version, conventional_analyze),
        _ => suggest_for_stable(version, conventional_analyze),
    }
}

fn suggest_for_stable<V>(version: &mut V, conventional_analyze: &dyn CommitExt) -> String
where
    V: AbstractVersion + Display + ?Sized,
{
    if conventional_analyze.is_breaking() {
        version.increment_major();
        version.reset_minor();
        version.reset_patch();
    } else if conventional_analyze.is_minor() {
        version.increment_minor();
        version.reset_patch();
    } else if conventional_analyze.is_patch() {
        version.increment_patch();
    }
    version.to_string()
}

fn suggest_for_unstable<V>(version: &mut V, conventional_analyze: &dyn CommitExt) -> String
where
    V: AbstractVersion + Display + ?Sized,
{
    if conventional_analyze.is_breaking() || conventional_analyze.is_minor() {
        version.increment_minor();
        version.reset_patch();
    } else if conventional_analyze.is_patch() {
        version.increment_patch();
    }
    version.to_string()
}

#[cfg(test)]
mod test_stable_version_bump {
    use super::*;

    use crate::variant::VersionVariant;
    use node_semver::Version;

    #[test]
    fn test_suggest_next_major_version() {
        let mut version = VersionVariant::Node(Version::parse("1.0.0").unwrap());
        let mut mock_commit_ext = MockCommitExt::new();
        mock_commit_ext.expect_is_breaking().returning(|| true);
        assert_eq!(
            suggest_next_version(&mut version, &mock_commit_ext),
            "2.0.0"
        );
    }

    #[test]
    fn test_suggest_next_minor_version() {
        let mut version = VersionVariant::Node(Version::parse("1.0.0").unwrap());
        let mut mock_commit_ext = MockCommitExt::new();
        mock_commit_ext.expect_is_breaking().returning(|| false);
        mock_commit_ext.expect_is_minor().returning(|| true);
        assert_eq!(
            suggest_next_version(&mut version, &mock_commit_ext),
            "1.1.0"
        );
    }

    #[test]
    fn test_suggest_next_patch_version() {
        let mut version = VersionVariant::Node(Version::parse("1.0.0").unwrap());
        let mut mock_commit_ext = MockCommitExt::new();
        mock_commit_ext.expect_is_breaking().returning(|| false);
        mock_commit_ext.expect_is_minor().returning(|| false);
        mock_commit_ext.expect_is_patch().returning(|| true);
        assert_eq!(
            suggest_next_version(&mut version, &mock_commit_ext),
            "1.0.1"
        );
    }
}

#[cfg(test)]
mod test_unstable_version_bup {
    use super::*;
    use crate::variant::VersionVariant;
    use node_semver::Version;

    #[test]
    fn test_suggest_next_major_version() {
        let mut version = VersionVariant::Node(Version::parse("0.1.0").unwrap());
        let mut mock_commit_ext = MockCommitExt::new();
        mock_commit_ext.expect_is_breaking().returning(|| true);
        assert_eq!(
            suggest_next_version(&mut version, &mock_commit_ext),
            "0.2.0",
        );
    }

    #[test]
    fn test_suggest_next_minor_version() {
        let mut version = VersionVariant::Node(Version::parse("0.1.0").unwrap());
        let mut mock_commit_ext = MockCommitExt::new();
        mock_commit_ext.expect_is_breaking().returning(|| false);
        mock_commit_ext.expect_is_minor().returning(|| true);
        assert_eq!(
            suggest_next_version(&mut version, &mock_commit_ext),
            "0.2.0"
        );
    }

    #[test]
    fn test_suggest_next_patch_version() {
        let mut version = VersionVariant::Node(Version::parse("0.1.0").unwrap());
        let mut mock_commit_ext = MockCommitExt::new();
        mock_commit_ext.expect_is_breaking().returning(|| false);
        mock_commit_ext.expect_is_minor().returning(|| false);
        mock_commit_ext.expect_is_patch().returning(|| true);
        assert_eq!(
            suggest_next_version(&mut version, &mock_commit_ext),
            "0.1.1"
        );
    }
}
#[cfg(test)]
mod test_commit_type {
    use super::*;

    #[test]
    fn test_feat_commit_detection() {
        let commit = CommitType::try_from("feat: add new feature").unwrap();
        assert_eq!(commit, CommitType::Feature);
    }

    #[test]
    fn test_feat_scoped_commit_detection() {
        let commit = CommitType::try_from("feat(scope): add new feature").unwrap();
        assert_eq!(commit, CommitType::Feature);
    }

    #[test]
    fn test_breaking_commit_detection() {
        let commit = CommitType::try_from("feat!: add new feature").unwrap();
        assert_eq!(commit, CommitType::Breaking);
    }

    #[test]
    fn test_breaking_scoped_commit_detection() {
        let commit = CommitType::try_from("feat(scope)!: add new feature").unwrap();
        assert_eq!(commit, CommitType::Breaking);
    }

    #[test]
    fn test_breaking_body_commit_detection() {
        let commit =
            CommitType::try_from("feat: add new feature\n\nBREAKING CHANGE: something").unwrap();
        assert_eq!(commit, CommitType::Breaking);
    }

    #[test]
    fn test_fix_commit_detection() {
        let commit = CommitType::try_from("fix: fix bug").unwrap();
        assert_eq!(commit, CommitType::Fix);
    }

    #[test]
    fn test_fix_scoped_commit_detection() {
        let commit = CommitType::try_from("fix(scope): fix bug").unwrap();
        assert_eq!(commit, CommitType::Fix);
    }

    #[test]
    fn test_build_commit_detection() {
        let commit = CommitType::try_from("build: build").unwrap();
        assert_eq!(commit, CommitType::Build);
    }

    #[test]
    fn test_build_scoped_commit_detection() {
        let commit = CommitType::try_from("build(scope): build").unwrap();
        assert_eq!(commit, CommitType::Build);
    }

    #[test]
    fn test_ci_commit_detection() {
        let commit = CommitType::try_from("ci: ci").unwrap();
        assert_eq!(commit, CommitType::Ci);
    }

    #[test]
    fn test_ci_scoped_commit_detection() {
        let commit = CommitType::try_from("ci(scope): ci").unwrap();
        assert_eq!(commit, CommitType::Ci);
    }

    #[test]
    fn test_docs_commit_detection() {
        let commit = CommitType::try_from("docs: docs").unwrap();
        assert_eq!(commit, CommitType::Docs);
    }

    #[test]
    fn test_docs_scoped_commit_detection() {
        let commit = CommitType::try_from("docs(scope): docs").unwrap();
        assert_eq!(commit, CommitType::Docs);
    }

    #[test]
    fn test_perf_commit_detection() {
        let commit = CommitType::try_from("perf: perf").unwrap();
        assert_eq!(commit, CommitType::Perf);
    }

    #[test]
    fn test_perf_scoped_commit_detection() {
        let commit = CommitType::try_from("perf(scope): perf").unwrap();
        assert_eq!(commit, CommitType::Perf);
    }

    #[test]
    fn test_refactor_commit_detection() {
        let commit = CommitType::try_from("refactor: refactor").unwrap();
        assert_eq!(commit, CommitType::Refactor);
    }

    #[test]
    fn test_refactor_scoped_commit_detection() {
        let commit = CommitType::try_from("refactor(scope): refactor").unwrap();
        assert_eq!(commit, CommitType::Refactor);
    }

    #[test]
    fn test_style_commit_detection() {
        let commit = CommitType::try_from("style: style").unwrap();
        assert_eq!(commit, CommitType::Style);
    }

    #[test]
    fn test_style_scoped_commit_detection() {
        let commit = CommitType::try_from("style(scope): style").unwrap();
        assert_eq!(commit, CommitType::Style);
    }

    #[test]
    fn test_revert_commit_detection() {
        let commit = CommitType::try_from("revert: revert").unwrap();
        assert_eq!(commit, CommitType::Revert);
    }

    #[test]
    fn test_revert_scoped_commit_detection() {
        let commit = CommitType::try_from("revert(scope): revert").unwrap();
        assert_eq!(commit, CommitType::Revert);
    }

    #[test]
    fn test_other_commit_detection() {
        let commit = CommitType::try_from("unknow: unknown").unwrap();
        assert_eq!(commit, CommitType::Other("unknow: unknown".to_string()));
    }

    #[test]
    fn test_other_scoped_commit_detection() {
        let commit = CommitType::try_from("unknow(scope): unknown").unwrap();
        assert_eq!(
            commit,
            CommitType::Other("unknow(scope): unknown".to_string())
        );
    }

    #[test]
    fn test_unconventional_commit_detection() {
        let commit = CommitType::try_from("Small change to the codebase").unwrap();
        assert_eq!(
            commit,
            CommitType::Other("Small change to the codebase".to_string())
        );
    }

    #[test]
    fn test_breaking_commit_types() {
        let breaking_commit = CommitType::Breaking;
        assert!(breaking_commit.is_breaking());
        assert!(!breaking_commit.is_minor());
        assert!(!breaking_commit.is_patch());

        let feature_commit = CommitType::Feature;
        assert!(!feature_commit.is_breaking());

        let fix_commit = CommitType::Fix;
        assert!(!fix_commit.is_breaking());

        let build_commit = CommitType::Build;
        assert!(!build_commit.is_breaking());

        let ci_commit = CommitType::Ci;
        assert!(!ci_commit.is_breaking());

        let docs_commit = CommitType::Docs;
        assert!(!docs_commit.is_breaking());

        let perf_commit = CommitType::Perf;
        assert!(!perf_commit.is_breaking());

        let refactor_commit = CommitType::Refactor;
        assert!(!refactor_commit.is_breaking());

        let style_commit = CommitType::Style;
        assert!(!style_commit.is_breaking());

        let revert_commit = CommitType::Revert;
        assert!(!revert_commit.is_breaking());

        let other_commit = CommitType::Other("unknown".to_string());
        assert!(!other_commit.is_breaking());
    }

    #[test]
    fn test_minor_commit_types() {
        let feature_commit = CommitType::Feature;
        assert!(feature_commit.is_minor());
        assert!(!feature_commit.is_breaking());
        assert!(!feature_commit.is_patch());

        let breaking_commit = CommitType::Breaking;
        assert!(!breaking_commit.is_minor());

        let fix_commit = CommitType::Fix;
        assert!(!fix_commit.is_minor());

        let build_commit = CommitType::Build;
        assert!(!build_commit.is_minor());

        let ci_commit = CommitType::Ci;
        assert!(!ci_commit.is_minor());

        let docs_commit = CommitType::Docs;
        assert!(!docs_commit.is_minor());

        let perf_commit = CommitType::Perf;
        assert!(!perf_commit.is_minor());

        let refactor_commit = CommitType::Refactor;
        assert!(!refactor_commit.is_minor());

        let style_commit = CommitType::Style;
        assert!(!style_commit.is_minor());

        let revert_commit = CommitType::Revert;
        assert!(!revert_commit.is_minor());

        let other_commit = CommitType::Other("unknown".to_string());
        assert!(!other_commit.is_minor());
    }

    #[test]
    fn test_patch_commit_types() {
        let fix_commit = CommitType::Fix;
        assert!(fix_commit.is_patch());
        assert!(!fix_commit.is_breaking());
        assert!(!fix_commit.is_minor());

        let breaking_commit = CommitType::Breaking;
        assert!(!breaking_commit.is_patch());

        let feature_commit = CommitType::Feature;
        assert!(!feature_commit.is_patch());

        let build_commit = CommitType::Build;
        assert!(!build_commit.is_patch());

        let ci_commit = CommitType::Ci;
        assert!(!ci_commit.is_patch());

        let docs_commit = CommitType::Docs;
        assert!(!docs_commit.is_patch());

        let perf_commit = CommitType::Perf;
        assert!(!perf_commit.is_patch());

        let refactor_commit = CommitType::Refactor;
        assert!(!refactor_commit.is_patch());

        let style_commit = CommitType::Style;
        assert!(!style_commit.is_patch());

        let revert_commit = CommitType::Revert;
        assert!(!revert_commit.is_patch());

        let other_commit = CommitType::Other("unknown".to_string());
        assert!(!other_commit.is_patch());
    }
}
