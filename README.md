# mydylint

A portable collection of personal Rust lints, loaded by [Dylint](https://github.com/trailofbits/dylint).

## Install

```toml
[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/fxyoge/mydylint.git", rev = "<tag>", pattern = "lints/*" },
]
```

Then run:

```sh
cargo dylint --all -- --all-targets --locked
```
