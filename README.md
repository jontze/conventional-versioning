# Conventional Versioning

[![Main](https://github.com/jontze/conventional-versioning/actions/workflows/main.yml/badge.svg?branch=main&event=push)](https://github.com/jontze/conventional-versioning/actions/workflows/main.yml)

Conventional Versioning is a tool for automatically generating version numbers based on the [Conventional Commits](https://www.conventionalcommits.org/) specification. It uses the commit messages in your Git repository and your latest tag to determine the next version number according to the [Semantic Versioning](https://semver.org/) specification.

## Installation

There are various ways to install Conventional Versioning. The easiest way is to install it using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) derictly from the git repository:

```sh
cargo install --git https://github.com/jontze/conventional-versioning
```

Alternatively, you can install it from the GitHub releases page (replace `VERSION` with the version you want to install):

```sh
curl -L https://github.com/jontze/conventional-versioning/releases/download/VERSION/conventional-versioning-VERSION-x86_64-unknown-linux-musl.tar.gz | tar -xz
```

## Usage

To use Conventional Versioning, you can run the following command in your Git repository:

```sh
conventional-versioning
```

This will determine the next version number based on the **commit messages** in your repository and the latest **git tag**, and output it to stdout. You can decide on various options to customize the output.

_For more information run call the `--help` argument._

## License

This project is licensed under the [MIT License](./LICENSE).
