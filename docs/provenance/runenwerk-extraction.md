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

The source commit completed validated local bounds and rays, finite signed values, optional conservative steps, exact-distance capability, invariant-preserving construction, structured query outcomes, explicit gradient failures, and all nine integration-test modules.

## Transfer mapping

```text
domain/sdf/src/**   -> src/**
domain/sdf/tests/** -> tests/**
package sdf         -> package runen-sdf
crate imports sdf   -> crate imports runen_sdf
```

The implementation and tests are transferred without behavioral redesign. Repository identity, crate imports, framework documentation, conformance, and validation are the only intentional transfer-layer changes.

This record does not claim preserved per-file Git history. The exact source commit, path, and parity review are the durable provenance authority.

## Completion fields

```text
standalone transfer commit: pending branch commit
standalone validation run: pending automatic GitHub Actions allocation
Runenwerk cutover commit: pending PT-RUNENSDF-004
Runenwerk source retirement: pending PT-RUNENSDF-004
```
