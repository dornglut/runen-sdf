# Status Map

## Current state

`PT-RUNENSDF-003 — Standalone Repository and Corrected Source Transfer` is complete.

The accepted standalone revision is:

```text
repository: Crystonix/runen-sdf
commit: d52badefc640d6dc6dcdd40268af3aea1bb8eefe
package: runen-sdf
crate: runen_sdf
source baseline: Crystonix/runenwerk@8de096259eab30f8d67672010df9190970d0bfc4
```

## Accepted baseline

```text
repository identity              complete
root public package              complete
licenses and security policy     complete
corrected source transfer        complete
all nine integration tests       complete
public downstream conformance    complete
framework documentation          complete
committed independent lockfile   complete
repository validation authority  complete
durable repository CI            configured
standalone parity review         complete
PT-RUNENSDF-003 merge             complete
```

Automatic command validation passed in public mirror runs `29845971330` and
`29846386222`. The native pull-request run `29846751864` failed before any runner
step was allocated and therefore produced no source-command evidence. This
repository retains the same `cargo validate` authority and requires a successful
native run before this closeout correction is merged.

## External integration state

Runenwerk clean cutover is not owned by this repository. Current Runenwerk `main`
still contains `domain/sdf`; its consumer audit, dependency decision, internal
package retirement, and duplicate-authority proof are `PT-RUNENSDF-004` work in
`Crystonix/runenwerk`.

RunenSDF must not add a compatibility package, forwarding namespace, source mirror,
or dependency on Runenwerk during that cutover.
