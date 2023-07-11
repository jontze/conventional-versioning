use clap::Parser;

mod conventional;
mod repo;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the repository
    #[arg(short = 'p', long, value_hint = clap::ValueHint::DirPath)]
    path: Option<std::path::PathBuf>,
    /// The initial start version of the repository. Useful if no tags are present yet.
    #[arg(short = 'i', long)]
    initial_version: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let repo = repo::open(&args)?;

    let (tag_name, tag_obj) = repo::latest_tag(&repo)?;

    println!("{tag_name}: {:?}", tag_obj.id());

    let commits = repo::commits_since_tag(&repo, &tag_obj);
    let _result = conventional::analyze(commits?)?;
    Ok(())
}
