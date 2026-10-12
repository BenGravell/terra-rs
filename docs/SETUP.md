# Setup

## Install

Install [rustup](https://rustup.rs/) and [mise](https://mise.jdx.dev/installing-mise.html).

Then run these commands from the repository root:

```sh
rustup show
mise trust
mise install
```

## Run

```sh
mise exec -- trunk serve --open
```

Open `http://127.0.0.1:8080/terra/`. Trunk rebuilds the app as source files change.

For the optional native development window:

```sh
cargo run --locked
```

Browser settings persist across reloads; the native window starts with defaults.

## Version configuration

| File                                                    | Purpose                                                   |
| ------------------------------------------------------- | --------------------------------------------------------- |
| [mise.toml](../mise.toml)                               | Trunk, Python, and uv tool versions.                      |
| [rust-toolchain.toml](../rust-toolchain.toml)           | Rust channel, rustfmt/Clippy components, and WASM target. |
| [Cargo.toml](../Cargo.toml)                             | App and workspace Rust dependency configuration.          |
| [tools/prepare/Cargo.toml](../tools/prepare/Cargo.toml) | Data preparation Rust dependencies.                       |
| [Cargo.lock](../Cargo.lock)                             | Resolved Rust dependency versions.                        |
| [pyproject.toml](../pyproject.toml)                     | Browser-test Python dependencies.                         |
| [uv.lock](../uv.lock)                                   | Resolved Python dependency versions.                      |

Local setup and CI read these files. Python is used for browser automation; running the app, preparing data, and building WASM use Rust.

## Static hosting

Build the release site:

```sh
mise exec -- trunk build --release --cargo-profile web
```

The output is `dist/`. [Trunk.toml](../Trunk.toml) sets the public URL to `/terra/`; serve the output under that path over HTTP(S).

The [GitHub workflow](../.github/workflows/deploy.yml) runs checks and produces a `site-preview` artifact.

## Setup for GitHub Pages publication

1. Set **Settings → Pages → Source** to **GitHub Actions**.
2. Add the Actions repository variable **DEPLOY_PAGES=true**.
3. Push to `main` or run the workflow on `main`, then verify the published site.
