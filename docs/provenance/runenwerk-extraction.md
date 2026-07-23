# Runenwerk Extraction Provenance

## Current repository authority

```text
repository: dornglut/runen-sdf
package: runen-sdf
crate: runen_sdf
source-transfer revision: d52badefc640d6dc6dcdd40268af3aea1bb8eefe
maintained main includes later authority and CI commits
```

## Historical source authority

```text
source repository at extraction: Crystonix/runenwerk
current repository identity: dornglut/runenwerk
source commit: 8de096259eab30f8d67672010df9190970d0bfc4
source path: domain/sdf
source phase: PT-RUNENSDF-002
source pull request: dornglut/runenwerk#116
transfer phase: PT-RUNENSDF-003
standalone pull request: dornglut/runen-sdf#1
historical owner at extraction: Crystonix
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
accepted source-transfer merge: d52badefc640d6dc6dcdd40268af3aea1bb8eefe
historical validation mirror repository: Crystonix/runen-ui
current mirror repository identity: dornglut/runen-ui
historical validation mirror pull request: dornglut/runen-ui#16
full command validation run: 29845971330
maintained mirror cargo validate run: 29846386222
maintained mirror cargo validate conclusion: success
native pre-runner failure: 29846751864
native authority-correction candidate: 2d02f66a7b88addf8d871c88e79489591c92e079
native cargo validate run: 29895816472
native cargo validate conclusion: success
```

The original native pull-request run produced no source-command logs because
execution failed before runner allocation. To avoid substituting manual owner
validation, the complete standalone candidate source, all nine integration tests,
downstream conformance package, independent lockfile, and repository validation
tooling were mirrored into a temporary public validation branch. GitHub Actions
then executed the maintained `cargo validate` authority successfully.

The successful maintained-authority runs covered repository policy, relative
Markdown links, locked metadata, direct and inverse dependency trees, formatting,
all workspace tests, downstream public conformance, all-target denied-warning
Clippy, denied-warning rustdoc, Rust 1.93.0 tests, diff hygiene, and clean tracked
state. Native run `29895816472` confirms the same authority executes successfully
inside the standalone repository.

The temporary validation branch and pull request were evidence-only and were not
merged into RunenUI.

## Remaining integration fields

```text
Runenwerk cutover issue: dornglut/runenwerk#133
Runenwerk cutover commit: pending PT-RUNENSDF-004
Runenwerk source retirement: pending PT-RUNENSDF-004
Runenwerk dependency decision: pending exact consumer census
```

Runenwerk must not retain a forwarding package, compatibility alias, source include,
branch dependency, or duplicate implementation after the clean cutover. If its
complete census proves zero consumers, the correct cutover adds no standalone
dependency and retires the internal package directly.
