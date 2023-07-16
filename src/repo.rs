use git2::{DescribeFormatOptions, Object, Repository};
use miette::{miette, Context};
#[cfg(test)]
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use crate::{args::SemVerKindArg, variant::VersionVariant};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub(crate) struct Commit {
    pub id: String,
    pub message: String,
}

impl TryFrom<git2::Commit<'_>> for Commit {
    type Error = miette::Error;

    fn try_from(commit: git2::Commit) -> Result<Self, Self::Error> {
        Ok(Self {
            id: commit.id().to_string(),
            message: commit
                .message()
                .ok_or_else(|| miette!("Commit message is not valid UTF-8: '{}'", commit.id()))?
                .to_string(),
        })
    }
}

pub(crate) fn open(path: Option<PathBuf>) -> miette::Result<Repository> {
    let repo_path = path.unwrap_or(std::path::Path::new(".").to_path_buf());
    let repo = Repository::open(repo_path)
        .map_err(|_| miette!("No repository found at the target location"))?;
    Ok(repo)
}

pub(crate) fn latest_tag(
    repo: &Repository,
    version_variant: SemVerKindArg,
) -> miette::Result<(VersionVariant, Object)> {
    let latest_tag_name = repo
        .describe(git2::DescribeOptions::new().describe_tags())
        .map_err(|_| miette!("No tags in the repository"))?
        .format(Some(DescribeFormatOptions::default().abbreviated_size(0)))
        .map_err(|_| miette!("Unable to format tag name"))?;
    let latest_tag_object = repo
        .revparse_single(&latest_tag_name)
        .map_err(|_| miette!("Unable to find latest tag object '{latest_tag_name}'"))?;
    let latest_tag = match version_variant {
        SemVerKindArg::Node => VersionVariant::Node(
            node_semver::Version::parse(&latest_tag_name)
                .context("Unable to parse latest tag as a node semver version")?,
        ),
        SemVerKindArg::Cargo => VersionVariant::Cargo(
            semver::Version::parse(
                latest_tag_name
                    .strip_prefix('v')
                    .unwrap_or(&latest_tag_name),
            )
            .map_err(|_| {
                miette!("Unable to parse tag '{latest_tag_name}' as cargo SemVer version. Is this a valid format?")
            })?,
        ),
    };
    Ok((latest_tag, latest_tag_object))
}

pub(crate) fn commits_since_tag<'a>(
    repo: &'a Repository,
    tag: &'a Object,
) -> miette::Result<Vec<Commit>> {
    let mut revwalk = repo
        .revwalk()
        .map_err(|_| miette!("Unable to start revwalk."))?;
    revwalk
        .push_head()
        .map_err(|_| miette!("Unable to push HEAD to revwalk"))?;
    revwalk
        .set_sorting(git2::Sort::TOPOLOGICAL)
        .map_err(|_| miette!("Unable to define revwalk sorting"))?;

    let tag_commit_id = tag
        .peel_to_commit()
        .map_err(|_| miette!("Tag can't be peeled to a commit"))?
        .id();

    let mut commits = Vec::new();
    for commit_id in revwalk {
        let commit_id = commit_id.unwrap();
        if commit_id == tag_commit_id {
            break;
        }
        let commit = repo
            .find_commit(commit_id)
            .map_err(|_| miette!("Unable to find commit of revwalk"))?;
        commits.push(commit.try_into()?);
    }
    Ok(commits)
}
