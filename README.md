# exploding-head-game

this is a prototype for a game that helps me process my psychotic state.

## build

either build with `cargo build` as standalone app, or use `trunk build` and `trunk serve` to build the wasm version.

## vscode

crates are separate in the `crates/` directory. to make it work in visual studio code, add the following to the file `/.vscode/settings.json`:

```json
{
    "rust-analyzer.linkedProjects": [
        "crates/*/Cargo.toml",
    ]
}
```

