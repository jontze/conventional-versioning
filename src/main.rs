mod configuration;
mod conventional;
mod output;
mod repo;
mod variant;

use repo::Repo;

fn main() -> miette::Result<()> {
    let config = configuration::Config::new()?;

    let repo = Repo::open(config.repo_path())?;
    let (mut tag_version, tag_obj) = repo.latest_tag(config.semver_kind())?;
    let previous_version = tag_version.clone();

    let commits = repo.commits_since_tag(&tag_obj)?;
    let result = conventional::analyze(commits, config.prefixes())?;
    let next_version = conventional::suggest_next_version(&mut tag_version, &result);

    println!(
        "{}",
        output::stringify(
            config.output(),
            next_version,
            previous_version.to_string(),
            result
        )?
    );
    Ok(())
}
