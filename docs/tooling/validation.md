# Validation

`cargo validate` is the single maintained repository validation authority. GitHub
Actions invokes this command; the workflow does not maintain a second command
list.

The validation envelope covers:

- committed locked metadata and dependency trees;
- formatting;
- all workspace tests and downstream public conformance;
- Clippy for all targets with denied warnings;
- rustdoc with denied warnings;
- Rust 1.93.0 tests;
- exact manifest inventory and repository-local path dependencies;
- rejection of Runenwerk references, source includes, forwarding packages,
  submodules, stale package identities, and forbidden repository layouts;
- required licenses, security policy, provenance, and relative Markdown links;
- Git diff hygiene and clean tracked state after validation.

`Cargo.lock` is committed repository authority. CI treats a missing, outdated, or
modified lockfile as a failure and never generates or commits it.

The durable workflow is `.github/workflows/validation.yml`. It checks the exact
revision and runs only `cargo validate` after installing stable Rust and the
accepted Rust 1.93.0 minimum toolchain.

During PT-RUNENSDF-003, the private target repository's Actions service failed
before runner allocation. Automatic command evidence was therefore obtained from
a temporary public mirror containing the transferred source, all nine tests,
downstream conformance, lockfile, and validation tooling. GitHub Actions run
`29846386222` passed the maintained `cargo validate` authority. No owner-operated
local validation was substituted.
