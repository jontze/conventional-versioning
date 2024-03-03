use miette::miette;

use super::shared::{OutputFormat, Prefixes, SemVerKindArg};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct Settings {
    pub kind: SemVerKindArg,
    pub output: OutputFormat,
    pub prefixes: Prefixes,
}

pub(crate) fn read_config_at_path<P, TConfig>(path: P) -> miette::Result<TConfig>
where
    P: AsRef<std::path::Path>,
    TConfig: for<'de> serde::Deserialize<'de>,
{
    let file = std::fs::File::open(path).map_err(|err| miette!(err.to_string()))?;
    let reader = std::io::BufReader::new(file);
    let settings = serde_yaml::from_reader(reader).map_err(|err| miette!(err.to_string()))?;
    Ok(settings)
}
