# Roadmap

## PT-RUNENSDF-003 — Standalone repository

State: complete.

The corrected public package, package tests, downstream conformance, provenance,
validation tooling, independent lockfile, and repository CI were accepted at
`d52badefc640d6dc6dcdd40268af3aea1bb8eefe`.

## PT-RUNENSDF-004 — Runenwerk clean cutover

State: external integration work in `Crystonix/runenwerk`.

Runenwerk must audit every current consumer, add the accepted standalone dependency
only where a real consumer exists, remove `domain/sdf` and its workspace and lockfile
authority, and prove that no forwarding package, alias, source include, branch
dependency, or duplicate implementation remains.

If Runenwerk has no product consumer, it must remove the internal package without
adding an unused dependency on RunenSDF.

## PT-RUNENSDF-005 — Adoption closeout and release readiness

State: blocked by completion of the Runenwerk clean cutover.

Close integration provenance, adoption evidence, compatibility policy, release
policy, and obsolete branch cleanup. Publication remains disabled until separately
authorized.

## Later framework work

Property-test dependencies, benchmark regression policy, serialization, GPU
backends, shader authoring, module regrouping, and multi-package decomposition
require separate evidence and authorization. They are not part of the extraction
or clean-cutover phases.
