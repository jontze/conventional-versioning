use clap::Parser;

mod args;
mod conventional;
mod output;
mod repo;
mod variant;

use args::{Args, SemVerKindArg};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let semver_kind = args.kind.unwrap_or(SemVerKindArg::Node);
    let cli_output_format = args.output.unwrap_or(output::OutputFormat::Human);

    let repo = repo::open(args.path)?;

    let (mut tag_version, tag_obj) = repo::latest_tag(&repo, semver_kind)?;
    let previous_version = tag_version.clone();

    let commits = repo::commits_since_tag(&repo, &tag_obj);
    let result = conventional::analyze(commits?)?;
    let next_version = conventional::suggest_next_version(&mut tag_version, &result);

    println!(
        "{}",
        output::stringify(
            cli_output_format,
            next_version,
            previous_version.to_string(),
            result
        )?
    );
    Ok(())
}
