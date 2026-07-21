# Runenwerk Extraction Provenance

## Source authority

```text
source repository: Crystonix/runenwerk
source commit: 8de096259eab30f8d67672010df9190970d0bfc4
source path: domain/sdf
source phase: PT-RUNENSDF-002
source pull request: Crystonix/runenwerk#116
transfer phase: PT-RUNENSDF-003
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
validation mirror repository: Crystonix/runen-ui
validation mirror pull request: Crystonix/runen-ui#16
full command validation run: 29845971330
maintained cargo validate run: 29846386222
maintained cargo validate conclusion: success
```

The private target repository's GitHub Actions service failed before runner
allocation and produced no source-command logs. To avoid substituting manual owner
validation, the complete standalone candidate source, all nine integration tests,
downstream conformance package, independent lockfile, and repository validation
tooling were mirrored into a temporary public validation branch. GitHub Actions
then executed the maintained `cargo validate` authority successfully.

The successful maintained-authority run covered repository policy, relative
Markdown links, locked metadata, direct and inverse dependency trees, formatting,
all workspace tests, downstream public conformance, all-target denied-warning
Clippy, denied-warning rustdoc, Rust 1.93.0 tests, diff hygiene, and clean tracked
state.

The temporary validation branch and pull request are evidence-only and must not be
merged into RunenUI.

## Remaining cutover fields

```text
accepted standalone revision: final PT-RUNENSDF-003 pull-request head
Runenwerk cutover commit: pending PT-RUNENSDF-004
Runenwerk source retirement: pending PT-RUNENSDF-004
```
