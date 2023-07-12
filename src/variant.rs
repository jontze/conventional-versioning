use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum VersionVariant {
    Node(node_semver::Version),
    Cargo(semver::Version),
}

pub(crate) trait AbstractVersion {
    fn major(&self) -> u64;
    fn minor(&self) -> u64;
    fn patch(&self) -> u64;
    fn increment_major(&mut self);
    fn increment_minor(&mut self);
    fn increment_patch(&mut self);
    fn reset_minor(&mut self);
    fn reset_patch(&mut self);
}

impl AbstractVersion for VersionVariant {
    fn major(&self) -> u64 {
        match self {
            VersionVariant::Node(node_version) => node_version.major,
            VersionVariant::Cargo(cargo_version) => cargo_version.major,
        }
    }

    fn minor(&self) -> u64 {
        match self {
            VersionVariant::Node(node_version) => node_version.minor,
            VersionVariant::Cargo(cargo_version) => cargo_version.minor,
        }
    }

    fn patch(&self) -> u64 {
        match self {
            VersionVariant::Node(node_version) => node_version.patch,
            VersionVariant::Cargo(cargo_version) => cargo_version.patch,
        }
    }

    fn increment_major(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.major += 1,
            VersionVariant::Cargo(cargo_version) => cargo_version.major += 1,
        }
    }

    fn increment_minor(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.minor += 1,
            VersionVariant::Cargo(cargo_version) => cargo_version.minor += 1,
        }
    }

    fn increment_patch(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.patch += 1,
            VersionVariant::Cargo(cargo_version) => cargo_version.patch += 1,
        }
    }

    fn reset_minor(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.minor = 0,
            VersionVariant::Cargo(cargo_version) => cargo_version.minor = 0,
        }
    }

    fn reset_patch(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.patch = 0,
            VersionVariant::Cargo(cargo_version) => cargo_version.patch = 0,
        }
    }
}

impl Display for VersionVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionVariant::Node(node_version) => write!(f, "{}", node_version),
            VersionVariant::Cargo(cargo_version) => write!(f, "{}", cargo_version),
        }
    }
}
