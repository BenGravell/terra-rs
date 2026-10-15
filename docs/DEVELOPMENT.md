# Development

Complete [Setup](SETUP.md) first.
Run commands below from the repository root.

## Architecture

Bevy hosts the application and egui renders settings, ranking, contribution bars, and a fixed 2D map.
Scoring and filtering are ordinary Rust functions independent of the UI.

| Location                                                        | Responsibility                                                                                                 |
| --------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| [src/model](../src/model)                                       | Named domain values, countries, settings, scores, map geometry, validation, and weight redistribution.         |
| [src/scoring.rs](../src/scoring.rs)                             | Score calculation, contributions, constraint evaluation, and ranking.                                          |
| [src/ui](../src/ui)                                             | User interface: app orchestration, responsive layout, settings controls, result views, theme, and persistence. |
| [tools/prepare](../tools/prepare)                               | Data preparation.                                                                                              |
| [data/catalog.toml](../data/catalog.toml)                       | Source metadata, checksums, mappings, units, and transformation rules.                                         |
| [data/raw](../data/raw)                                         | Pinned input files.                                                                                            |
| [assets](../assets)                                             | Prepared country data, triangulated map geometry, and fonts.                                                   |
| Adjacent `*_test.rs` files                                      | Unit tests beside the source files they exercise.                                                              |
| [index.html](../index.html), [web](../web)                      | WASM shell, loading/error handling, and loading-screen fonts.                                                  |
| [.github/workflows/deploy.yml](../.github/workflows/deploy.yml) | Checks, preview artifact, and Pages publication from `main`.                                                   |

Model fields name culture dimensions, quality metrics, weights, minimums, range bounds, and coordinates.
`serde_tuple` derives serialize and deserialize these named fields as JSON arrays in declaration order.
`Arraygen` derives `values()` from all declared fields for numeric iteration, keeping field order and array length
automatic.

Results are recomputed when settings change.
The table, bars, and map use the same scores and selection.
Prepared JSON and cached geometry are embedded in the application; raw data and Polars stay out of the WASM dependency
graph.
The app runs entirely in the browser.

Within `src/ui/`, [app.rs](../src/ui/app.rs) owns initialization, recomputation, selection reconciliation, and
persistence calls.
[layout.rs](../src/ui/layout.rs) owns panels, scrolling, and responsive placement.
Components receive the settings, data, or selection they need: settings combines culture, weights, constraints, and
reference-country controls; ranking, bars, and map render results.
Theme and browser persistence have separate modules.

Tests use `#[cfg(test)]` modules pointing to an adjacent file with the source file's name plus `_test`: for example,
`src/scoring_test.rs`, `src/model/settings_test.rs`, and `src/ui/layout_test.rs`.
Shared test data and the egui harness live in test-only `test_support.rs` modules.
The preparation package follows the same convention with `tools/prepare/source_test.rs`.

## Build and checks

Install the Git hooks once per clone with `mise exec -- pre-commit install`.
The hooks in [.pre-commit-config.yaml](../.pre-commit-config.yaml) run in this order:

1. Format Markdown with mdformat, excluding `.agents/`.
2. Format Rust with `cargo fmt --all`.
3. Apply native workspace Clippy fixes, then reject remaining native and WASM warnings with `-D warnings`.
4. Run `cargo test --locked --workspace`.
5. Check for unused dependencies with `cargo machete`.

Hooks stop at the first failure.
If a hook changes files, review and stage those changes before retrying the commit.
Warnings without automatic fixes must be corrected manually.
During a commit, pre-commit temporarily sets aside unstaged changes to tracked files and restores them afterward.
The Rust hooks run even for documentation-only commits, but successful Clippy and test results are cached by mise.
Cache inputs include workspace sources, assets, data, configuration, toolchain versions, and relevant environment
variables.
Markdown-only changes reuse successful Rust results when the other inputs are unchanged.

Run all hooks manually:

```sh
mise exec -- pre-commit run --all-files
```

Run either cached task directly, or force fresh results:

```sh
mise run hook-clippy-fix
mise run hook-test
mise run --force hook-clippy-fix
mise run --force hook-test
```

The task definitions live in [mise.toml](../mise.toml).
[tools/clippy_fix.py](../tools/clippy_fix.py) prevents caching a Clippy run that changed files;
[tools/cargo_hook_context.py](../tools/cargo_hook_context.py) includes inherited configuration and environment in cache
keys.

Run the individual Rust checks and build with:

```sh
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo clippy --locked --target wasm32-unknown-unknown --lib --bin terra -- -D warnings
cargo test --locked --workspace
cargo run --locked -p terra-prepare -- --check
mise exec -- trunk build --release --cargo-profile web
```

For core checks without compiling Bevy:

```sh
cargo test --locked --no-default-features
```

CI runs Rust checks, deterministic asset regeneration checks, a release WASM build, and Chromium smoke tests.
Asset regeneration, the release build, and browser smoke tests remain separate from the commit hooks.
Build and deployment details are in [Setup](SETUP.md#static-hosting).

## Maintain the data

```sh
cargo run --locked -p terra-prepare
```

The preparation package reads checksummed local inputs, applies catalog mappings and normalization, and generates
[assets/countries.json](../assets/countries.json), [assets/map.json](../assets/map.json), and
[data/coverage.json](../data/coverage.json).
It does not download sources.
Frontend builds use the checked-in generated assets.

For a source update:

1. Replace the relevant raw file.
   Record its source, year, units, rights, and SHA-256 in the catalog after inspecting it (`sha256sum data/raw/FILE`).
   Record unknown metadata explicitly.
2. Update column mappings as needed.
   Country names must match those in `country_codes_alpha_3.csv`.
3. Run preparation.
   Missing columns, bad numeric values, duplicate countries, invalid values, checksum changes, and unmapped eligible
   countries fail with an error.
4. Review the coverage report and generated diff.
   Dataset maxima affect normalization, so changing one input can change other countries' scores.
   Missing values remain missing, with exclusion reasons recorded in the coverage report.
5. Run the checks above.
   Review expected scoring behavior whenever datasets or formulas change.

The map uses a fixed equirectangular projection and offline `earcutr` triangulation for polygons, holes, and
multipolygons.
The source geometry splits antimeridian islands; location markers cover small countries such as Malta and Singapore.

See [data provenance](../data/README.md) for source years, Natural Earth version, ISO exceptions, missing-data
limitations, and licenses.

## Scoring

Weights are normalized within each group for scoring:

```text
culture_fit = 1 - sum(abs(preference_i - country_i)) / (6 * 100)
quality_of_life = w_hp * happy_planet + w_sp * social_progress + w_hf * human_freedom
overall = w_cf * culture_fit + w_ql * quality_of_life
```

Overall chart segments are `w_cf * culture_fit` and `w_ql * quality_of_life`.
The Quality of Life breakdown shows its three internal weighted terms; applying those directly to the overall score
would also require multiplying by `w_ql`.
Display rounding never affects calculations.

Weight controls maintain a 100% total by redistributing changes equally across the remaining weights, respecting
zero/100% bounds.
Validation checks the actual sum with floating-point tolerance.
Regression cases live in [src/model/weights_test.rs](../src/model/weights_test.rs).

Semantics:

- Happy Planet and Social Progress are normalized by their dataset maxima.
  Human Freedom is normalized against the entire raw dataset maximum **before** averaging 2015–2020.
- Eligibility requires complete culture, quality-of-life, temperature, sunshine, and continent data even when the
  corresponding constraint is inactive.
  English data is required only when its minimum is active.
- A score/language minimum is active above `EPS = 0.001`; active comparisons use strict `>`/`<`.
  Climate constraints activate when either bound moves inward from the default temperature `[-10, 30]` or sunshine
  `[3, 10]` range.
- Sequential ranks (`1, 2, 3, 4`) are assigned only to matching countries after constraints.
  Equal scores sort deterministically by country ID.
  Infeasible countries retain rank 0 internally and have no displayed rank.

## Testing

Rust tests exercise scoring formulas and contributions with hand-calculated examples, strict filter boundaries, missing
data, sequential ranks after filtering, weight balancing, settings serialization, and validation.
Geometry tests cover holes, antimeridian islands, corrupt indices, and marker coverage.
UI tests cover keyboard changes, weight balancing, reset, invalid settings, empty results, selection, and simultaneous
settings and live results on narrow screens.

After building the release site, run browser checks:

```sh
mise exec -- uv run --locked --group browser playwright install chromium firefox webkit
mise exec -- uv run --locked --group browser tools/browser_smoke.py
mise exec -- uv run --locked --group browser tools/browser_smoke.py --browser firefox
mise exec -- uv run --locked --group browser tools/browser_smoke.py --browser webkit
```

The script starts its own local server under `/terra-rs/`.
It exercises canvas controls, weight balancing, persistence/reset, corrupt and blocked storage, and desktop/narrow
layouts.
Chrome is used if installed; otherwise the downloaded Chromium is used.
Linux may also need Playwright's system dependencies; CI installs them with `playwright install --with-deps chromium` in
the configured browser dependency group.

Screenshots and timing/size measurements are written to ignored `artifacts/`.
