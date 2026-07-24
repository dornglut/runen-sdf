# Testing and Validation

`cargo validate` is the single maintained local and CI validation authority for RunenSDF.

It verifies:

- repository policy, package identity, manifest inventory, and dependency direction;
- formatting, tests, downstream conformance, Clippy, rustdoc, and Rust 1.93.0 MSRV compatibility;
- licenses, extraction provenance, Markdown links, and clean repository state;
- independence from Runenwerk source, path dependencies outside the repository, gitlinks, and duplicate source authority.

## Required command

```text
cargo validate
```

Run focused tests while editing, but do not substitute them for the complete baseline before review or merge. GitHub Actions invokes the same repository-owned command through the immutable shared Rust validation workflow.

Detailed command order, expected toolchains, and failure interpretation remain in [the validation contract](docs/tooling/validation.md).
