# Validation

`cargo validate` is the single maintained repository validation authority. GitHub Actions invokes this command; the workflow does not maintain a second command list.

The validation envelope covers:

- committed locked metadata and dependency trees;
- formatting;
- all workspace tests and downstream public conformance;
- Clippy for all targets with denied warnings;
- rustdoc with denied warnings;
- Rust 1.93.0 tests;
- exact manifest inventory and repository-local path dependencies;
- rejection of Runenwerk references, source includes, forwarding packages, submodules, stale package identities, and forbidden repository layouts;
- required licenses, security policy, provenance, and relative Markdown links;
- Git diff hygiene and clean tracked state after validation.

`Cargo.lock` is repository authority. The temporary bootstrap workflow may generate and commit the first lockfile on the active bootstrap branch. After that commit is accepted, durable CI must treat a missing or changed lockfile as a failure and must not generate it.
