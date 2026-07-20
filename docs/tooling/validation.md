# Validation

Use `cargo validate`. The alias delegates to `xtask`.

The target envelope covers locked metadata, formatting, workspace tests, workspace Clippy with denied warnings, documentation, Rust 1.93.0 compatibility, downstream conformance, dependency direction, metadata, licenses, document links, and diff hygiene.

CI calls the same validation authority instead of maintaining a separate command list.
