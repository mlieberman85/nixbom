# Nixbom

Nixbom is a tool intended to generate Software Bill of Materials (SBOM) based on Nix expressions and derivations.

Currently it supports SPDX.

## Installing
### With Nix
Assuming that you have enabled both the `flakes` and `nix-command` experimental features:
```
nix profile install github:mlieberman85/nixbom
```

### With Cargo
```
cargo install --path .
```
