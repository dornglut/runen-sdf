# Roadmap

## PT-RUNENSDF-003 — Standalone repository

State: complete.

The corrected public package, package tests, downstream conformance, provenance,
validation tooling, independent lockfile, and repository CI were accepted at the
source-transfer revision `d52badefc640d6dc6dcdd40268af3aea1bb8eefe`.

The maintained repository is `dornglut/runen-sdf`. Later documentation, governance,
and CI commits do not change the transferred framework source baseline.

## PT-RUNENSDF-004 — Runenwerk clean cutover

State: external integration work in `dornglut/runenwerk`, issue `#133`.

Runenwerk must prove the complete reverse-dependency and source-reference census for
its internal `domain/sdf` package. Current manifest evidence indicates that the
package remains a workspace member but is not declared by the likely runtime
consumers. If the complete census confirms zero product consumers, the cutover is a
retirement-only change:

- remove `domain/sdf`;
- remove workspace and lockfile authority;
- remove stale active documentation authority;
- add a durable no-return repository guard;
- add no unused dependency on RunenSDF.

An exact standalone dependency is added only if the census discovers a real consumer
that must retain the public RunenSDF contract. No forwarding package, alias, source
include, branch dependency, or duplicate implementation may remain.

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
