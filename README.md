# Conventional Versioning

[![Main](https://github.com/jontze/conventional-versioning/actions/workflows/main.yml/badge.svg?branch=main&event=push)](https://github.com/jontze/conventional-versioning/actions/workflows/main.yml)

Conventional Versioning is a tool for automatically generating version numbers
based on the [Conventional Commits](https://www.conventionalcommits.org/)
specification. It uses the commit messages in your Git repository and your
latest tag to determine the next version number according to the
[Semantic Versioning](https://semver.org/) specification.

## Installation

There are various ways to install Conventional Versioning. The easiest way is to
install it using
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
directly from the git repository:

```sh
cargo install --git https://github.com/jontze/conventional-versioning
```

Alternatively, you can install it from the GitHub releases page (replace
`VERSION` with the version you want to install):

```sh
curl -L https://github.com/jontze/conventional-versioning/releases/download/VERSION/conventional-versioning-VERSION-x86_64-unknown-linux-musl.tar.gz | tar -xz
```

## Usage

To use Conventional Versioning, you can run the following command in your Git
repository:

```sh
conventional-versioning
```

This will determine the next version number based on the **commit messages** in
your repository and the latest **git tag**, and output it to stdout. You can
decide on various options to customize the output.

_Call the `--help` argument for more information._

```sh
Usage: conventional-versioning [OPTIONS]

Options:
  -r, --repo <REPO>                Path to the repository. Default is the current directory [env: CONVENTIONAL_VERSIONING_REPO=]
  -c, --config <CONFIG>            Path to the configuration file. By default, the OS specific user configuration directories are checked. WARNING: If you use the `--config` option, all other args will be ignored, besides `--repo` [env: CONVENTIONAL_VERSIONING_CONFIG=]
  -k, --kind <KIND>                SemVer kind. Default is the Node SemVer variant [env: CONVENTIONAL_VERSIONING_KIND=] [default: node] [possible values: node, cargo]
  -o, --out <OUT>                  Output format. Default is the human readable format [env: CONVENTIONAL_VERSIONING_OUTPUT=] [default: human] [possible values: human, plain, json, yaml, yml, toml]
  -p, --patch-scope <PATCH_SCOPE>  Commit scopes that cause a patch version bump [env: CONVENTIONAL_VERSIONING_PATCH=]
  -m, --minor-scope <MINOR_SCOPE>  Commit scopes that cause a minor version bump [env: CONVENTIONAL_VERSIONING_MINOR=]
  -M, --major-scope <MAJOR_SCOPE>  Commit scopes that cause a major version bump [env: CONVENTIONAL_VERSIONING_MAJOR=]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Configuration

You can also provide a configuration via yaml file with the `--config` option.
Be aware that all other CLI options beside of `--repo` will be ignored if you
use the `--config` option.

**The configuration file should look like this:**

```yaml
---
# Configuration for conventional-versioning
kind: Node # Node | Cargo
output: Plain # Human | Plain | Json | Yaml | Yml | Toml
prefixes:
  patch:
    - "fix" # e.g. fix: ... | fix(scope): ...
    - "patch"
    - "chore"
    - "..."
  minor:
    - "feat"
    - "..."
  major: 
    # Commits with a "!" or "BREAKING CHANGE:"
    # will always be considered  as a major change
    - "breaking"
    - "major"
    - "..."
```

## License

This project is licensed under the [MIT License](./LICENSE).
