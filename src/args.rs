use crate::output;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub(crate) enum SemVerKindArg {
    Node,
    Cargo,
}

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub(crate) struct Args {
    /// Path to the repository
    #[arg(short = 'p', long, value_hint = clap::ValueHint::DirPath)]
    pub path: Option<std::path::PathBuf>,
    /// SemVer kind. Default is the Node SemVer variant
    #[arg(short = 'k', long)]
    pub kind: Option<SemVerKindArg>,
    /// Output format. Default is the human readable format
    #[arg(short = 'o', long)]
    pub output: Option<output::OutputFormat>,
}
