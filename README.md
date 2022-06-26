# exploding-head-game

this is a prototype for a game that helps me process my psychotic state.

## build

either build with `cargo build` as standalone app, or use `trunk build` and `trunk serve` to build the wasm version.

## vscode

crates are separate in the `crates/` directory. to make it work in visual studio code, add the following to the file `/.vscode/settings.json`:

```json
{
    "rust-analyzer.linkedProjects": [
        "crates/assets/Cargo.toml",
        "crates/build-tools/Cargo.toml",
        "crates/game/Cargo.toml",
        "crates/utils/Cargo.toml",
    ]
}

```

## crates

- `assets`: output of assets processing pipeline with structs.
- `build-tools`: tools used by build scripts, e.g. the asset processing pipeline.
- `game`: the game itself.
- `utils`: common utilities.
