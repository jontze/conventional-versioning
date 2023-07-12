use crate::output;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub(crate) enum SemVerVariantArg {
    Node,
    Cargo,
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// Path to the repository
    #[arg(short = 'p', long, value_hint = clap::ValueHint::DirPath)]
    pub path: Option<std::path::PathBuf>,
    /// SemVer variant. Default is the Node SemVer variant
    #[arg(short = 'v', long)]
    pub variant: Option<SemVerVariantArg>,
    /// Output format. Default is the human readable format
    #[arg(short = 'o', long)]
    pub output: Option<output::OutputFormat>,
}
