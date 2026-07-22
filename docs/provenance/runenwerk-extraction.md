# Runenwerk Extraction Provenance

## Source authority

```text
source repository: Crystonix/runenwerk
source commit: 8de096259eab30f8d67672010df9190970d0bfc4
source path: domain/sdf
source phase: PT-RUNENSDF-002
source pull request: Crystonix/runenwerk#116
transfer phase: PT-RUNENSDF-003
accepted standalone revision: d52badefc640d6dc6dcdd40268af3aea1bb8eefe
standalone pull request: Crystonix/runen-sdf#1
```

The source commit completed validated local bounds and rays, finite signed values,
optional conservative steps, exact-distance capability, invariant-preserving
construction, structured query outcomes, explicit gradient failures, and all nine
integration-test modules.

## Transfer mapping

```text
domain/sdf/src/**   -> src/**
domain/sdf/tests/** -> tests/**
package sdf         -> package runen-sdf
crate imports sdf   -> crate imports runen_sdf
```

The implementation and tests were transferred without behavioral redesign.
Repository identity, crate imports, framework documentation, downstream
conformance, licensing, security policy, and validation integration are the only
intentional transfer-layer changes.

This record does not claim preserved per-file Git history. The exact source commit,
path, transferred module inventory, test inventory, and automated parity evidence
are the durable provenance authority.

## Standalone delivery evidence

```text
source-and-test transfer commit: 2d1f5ea4bd739d5c6358e9348d757c2b54bcde1b
validated formatted candidate: 4f20b187d134e102193d5eb1dcd126c558accadd
accepted standalone merge: d52badefc640d6dc6dcdd40268af3aea1bb8eefe
validation mirror repository: Crystonix/runen-ui
validation mirror pull request: Crystonix/runen-ui#16
full command validation run: 29845971330
maintained cargo validate run: 29846386222
maintained cargo validate conclusion: success
native pull-request run: 29846751864
native pull-request conclusion: platform failure before runner steps
```

The native pull-request run produced no source-command logs because execution
failed before runner allocation. To avoid substituting manual owner validation,
the complete standalone candidate source, all nine integration tests, downstream
conformance package, independent lockfile, and repository validation tooling were
mirrored into a temporary public validation branch. GitHub Actions then executed
the maintained `cargo validate` authority successfully.

The successful maintained-authority run covered repository policy, relative
Markdown links, locked metadata, direct and inverse dependency trees, formatting,
all workspace tests, downstream public conformance, all-target denied-warning
Clippy, denied-warning rustdoc, Rust 1.93.0 tests, diff hygiene, and clean tracked
state.

The temporary validation branch and pull request were evidence-only and were not
merged into RunenUI. A successful native `runen-sdf` validation run remains the
final repository-authority correction gate.

## Remaining integration fields

```text
Runenwerk cutover commit: pending PT-RUNENSDF-004
Runenwerk source retirement: pending PT-RUNENSDF-004
Runenwerk dependency decision: pending exact consumer audit
```

Runenwerk must not retain a forwarding package, compatibility alias, source include,
branch dependency, or duplicate implementation after the clean cutover.
