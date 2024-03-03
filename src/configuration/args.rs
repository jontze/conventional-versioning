use super::shared::{OutputFormat, SemVerKindArg};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub(crate) struct Args {
    /// Path to the repository. Default is the current directory.
    #[arg(short = 'r', long, value_hint = clap::ValueHint::DirPath, env = "CONVENTIONAL_VERSIONING_REPO")]
    pub repo: Option<std::path::PathBuf>,
    /// Path to the configuration file. By default, the OS specific
    /// user configuration directories are checked.    
    /// WARNING: If you use the `--config` option, all other args will be ignored, besides `--repo`.
    #[arg(short = 'c', long, value_hint = clap::ValueHint::FilePath, env = "CONVENTIONAL_VERSIONING_CONFIG")]
    pub config: Option<std::path::PathBuf>,
    /// SemVer kind. Default is the Node SemVer variant.
    #[arg(
        short = 'k',
        long,
        default_value = "node",
        env = "CONVENTIONAL_VERSIONING_KIND"
    )]
    pub kind: Option<SemVerKindArg>,
    /// Output format. Default is the human readable format
    #[arg(
        short = 'o',
        long,
        default_value = "human",
        env = "CONVENTIONAL_VERSIONING_OUTPUT"
    )]
    pub out: Option<OutputFormat>,
    /// Commit scopes that cause a patch version bump.
    #[arg(short = 'p', long, env = "CONVENTIONAL_VERSIONING_PATCH")]
    pub patch_scope: Option<Vec<String>>,
    /// Commit scopes that cause a minor version bump.
    #[arg(short = 'm', long, env = "CONVENTIONAL_VERSIONING_MINOR")]
    pub minor_scope: Option<Vec<String>>,
    /// Commit scopes that cause a major version bump.
    #[arg(short = 'M', long, env = "CONVENTIONAL_VERSIONING_MAJOR")]
    pub major_scope: Option<Vec<String>>,
}
