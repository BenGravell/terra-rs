# Setup

## Install

Install [rustup](https://rustup.rs/) and [mise](https://mise.jdx.dev/installing-mise.html) 2026.10.2 or newer.

Then run these commands from the repository root:

```sh
rustup show
mise trust
mise install
mise exec -- pre-commit install
```

Hook installation is local to each clone; repeat it after cloning the repository elsewhere.
Keep `mise` and Cargo available on the PATH used by Git, including when committing from an editor.
Verify the setup with `mise exec -- pre-commit run --all-files`.
The first run downloads the Markdown formatter and may compile Rust dependencies.
See [Development](DEVELOPMENT.md#build-and-checks) for hook behavior and retry instructions.

## Version configuration

| File                                                    | Purpose                                                                       |
| ------------------------------------------------------- | ----------------------------------------------------------------------------- |
| [mise.toml](../mise.toml)                               | Trunk, Python, uv, pre-commit, and cargo-machete versions; cached hook tasks. |
| [.pre-commit-config.yaml](../.pre-commit-config.yaml)   | Commit hooks and pinned Markdown formatter versions.                          |
| [rust-toolchain.toml](../rust-toolchain.toml)           | Rust channel, rustfmt/Clippy components, and WASM target.                     |
| [Cargo.toml](../Cargo.toml)                             | App and workspace Rust dependency configuration.                              |
| [tools/prepare/Cargo.toml](../tools/prepare/Cargo.toml) | Data preparation Rust dependencies.                                           |
| [Cargo.lock](../Cargo.lock)                             | Resolved Rust dependency versions.                                            |
| [pyproject.toml](../pyproject.toml)                     | Browser-test Python dependencies.                                             |
| [uv.lock](../uv.lock)                                   | Resolved Python dependency versions.                                          |

Local setup and CI read these files.
Python is used for hook helpers and browser automation; running the app, preparing data, and building WASM use Rust.

## Static hosting

Build the release site:

```sh
mise exec -- trunk build --release --cargo-profile web
```

The output is `dist/`.
[Trunk.toml](../Trunk.toml) sets the public URL to `/terra-rs/`; serve the output under that path over HTTP(S).

The [GitHub workflow](../.github/workflows/deploy.yml) runs checks and produces a `site-preview` artifact.
Successful pushes and manual runs on `main` also deploy to GitHub Pages.
Pull requests run checks without deploying.

## Setup for GitHub Pages publication

1. Set **Settings → Pages → Source** to **GitHub Actions**.
2. Push to `main` or run the workflow on `main`, then verify the published site.
