use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum VersionVariant {
    Node(node_semver::Version),
    Cargo(semver::Version),
}

impl VersionVariant {
    pub(crate) fn major(&self) -> u64 {
        match self {
            VersionVariant::Node(node_version) => node_version.major,
            VersionVariant::Cargo(cargo_version) => cargo_version.major,
        }
    }

    pub(crate) fn minor(&self) -> u64 {
        match self {
            VersionVariant::Node(node_version) => node_version.minor,
            VersionVariant::Cargo(cargo_version) => cargo_version.minor,
        }
    }

    pub(crate) fn patch(&self) -> u64 {
        match self {
            VersionVariant::Node(node_version) => node_version.patch,
            VersionVariant::Cargo(cargo_version) => cargo_version.patch,
        }
    }

    pub(crate) fn increment_major(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.major += 1,
            VersionVariant::Cargo(cargo_version) => cargo_version.major += 1,
        }
    }

    pub(crate) fn increment_minor(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.minor += 1,
            VersionVariant::Cargo(cargo_version) => cargo_version.minor += 1,
        }
    }

    pub(crate) fn increment_patch(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.patch += 1,
            VersionVariant::Cargo(cargo_version) => cargo_version.patch += 1,
        }
    }

    pub(crate) fn reset_minor(&mut self) {
        match self {
            VersionVariant::Node(node_version) => node_version.minor = 0,
            VersionVariant::Cargo(cargo_version) => cargo_version.minor = 0,
        }
    }

    pub(crate) fn reset_patch(&mut self) {
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
