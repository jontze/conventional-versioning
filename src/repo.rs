use std::path::PathBuf;

use anyhow::Context;
use git2::{DescribeFormatOptions, Object, Repository};

use crate::{args::SemVerVariantArg, variant::VersionVariant};

pub(crate) fn open(path: Option<PathBuf>) -> anyhow::Result<Repository> {
    let repo_path = path.unwrap_or(std::path::Path::new(".").to_path_buf());
    let repo = Repository::open(repo_path)
        .context("Unable to open the repository at the given location")?;
    Ok(repo)
}

pub(crate) fn latest_tag(
    repo: &Repository,
    version_variant: SemVerVariantArg,
) -> anyhow::Result<(VersionVariant, Object)> {
    let latest_tag_name = repo
        .describe(git2::DescribeOptions::new().describe_tags())
        .context("There are no tags in the repository")?
        .format(Some(DescribeFormatOptions::default().abbreviated_size(0)))
        .context("Unable to format tag name")?;
    let latest_tag_object = repo
        .revparse_single(&latest_tag_name)
        .context("Unable to find latest tag by name {latest_tag_name}")?;
    let latest_tag = match version_variant {
        SemVerVariantArg::Node => VersionVariant::Node(
            node_semver::Version::parse(&latest_tag_name)
                .context("Unable to parse latest tag as a node semver version")?,
        ),
        SemVerVariantArg::Cargo => VersionVariant::Cargo(
            semver::Version::parse(
                latest_tag_name
                    .strip_prefix('v')
                    .unwrap_or(&latest_tag_name),
            )
            .context("Unable to parse latest tag as a cargo semver version")?,
        ),
    };
    Ok((latest_tag, latest_tag_object))
}

pub(crate) fn commits_since_tag<'a>(
    repo: &'a Repository,
    tag: &'a Object,
) -> anyhow::Result<Vec<git2::Commit<'a>>> {
    let mut revwalk = repo.revwalk().context("Unbale to create revwalk")?;
    revwalk
        .push_head()
        .context("Unable to push HEAD to revwalk")?;
    revwalk
        .set_sorting(git2::Sort::TOPOLOGICAL)
        .context("Unable to set sorting")?;

    let tag_commit_id = tag
        .peel_to_commit()
        .context("Tag can't be peeled to a commit")?
        .id();

    let mut commits = Vec::new();
    for commit_id in revwalk {
        let commit_id = commit_id.unwrap();
        if commit_id == tag_commit_id {
            break;
        }
        let commit = repo
            .find_commit(commit_id)
            .context("Unable to find commit of revwalk")?;
        commits.push(commit);
    }
    Ok(commits)
}
