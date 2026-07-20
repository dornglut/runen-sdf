# Runenwerk Extraction Provenance

## Source authority

```text
source repository: Crystonix/runenwerk
source commit: 8de096259eab30f8d67672010df9190970d0bfc4
source path: domain/sdf
source phase: PT-RUNENSDF-002
source pull request: Crystonix/runenwerk#116
```

The source commit completed the validated bounds and ray boundary, signed-value versus conservative-step model, exact-distance capability, structured query outcomes, fallible gradient and normal behavior, and all nine package test modules.

## Transfer policy

The initial standalone transfer preserves the proven implementation and tests before optional module reorganization. Package identity changes from `sdf` to `runen-sdf`; crate imports change from `sdf` to `runen_sdf`.

Runenwerk retains integration adapters and all ECS, world, scene, material, renderer, GPU, UI, and product-specific behavior.

This document does not claim preserved per-file Git history. The exact source commit and path are the provenance authority.

## Completion fields

```text
standalone parity commit: pending
standalone validation run: pending
Runenwerk cutover commit: pending PT-RUNENSDF-004
Runenwerk source retirement: pending PT-RUNENSDF-004
```
