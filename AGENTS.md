# RunenSDF Agent Guide

Start with `README.md`, `docs/architecture.md`, `docs/status-map.md`, and `docs/work-tracking.md`.

## Repository mission

RunenSDF owns host-neutral signed-field mathematics and CPU queries. It must not depend on Runenwerk or acquire ECS, world, renderer, material, GPU, UI, or product responsibilities.

## Required workflow

1. Read the current status and work-tracking documents.
2. Keep one public root package. Add a package only for a proven ownership and dependency boundary.
3. Preserve validated construction, finite values, conservative safe-step semantics, explicit capabilities, and structured query outcomes.
4. Do not introduce compatibility aliases or duplicate source authority.
5. Run `cargo validate` before declaring a change ready.
6. Update provenance and status when extraction or ownership changes.

## Public API rules

- Prefer explicit validated constructors.
- Keep invalid state unrepresentable where practical.
- Do not treat signed-value magnitude as a tracing step without proof.
- Do not fabricate normals or convert errors into ordinary misses.
- Avoid macro or derive magic unless it removes repeated syntax without hiding semantics.
