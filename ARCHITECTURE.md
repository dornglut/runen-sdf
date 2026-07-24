# Architecture

RunenSDF is a standalone, host-neutral Rust framework for validated signed-field mathematics and deterministic CPU reference queries.

## Ownership boundary

RunenSDF owns:

- finite signed-field samples and explicit capabilities;
- validated bounds, rays, primitives, composition, and transforms;
- gradients, normals, conservative tracing steps, and structured CPU query outcomes;
- conformance and provenance needed to keep the framework independently buildable.

RunenSDF does not own Runenwerk integration, ECS state, world streaming, materials, rendering, GPU resources, shaders, UI, native hosts, or product persistence.

## Dependency direction

The public package is the framework authority. The downstream conformance package may depend on it; production source must not depend on Runenwerk, copied source trees, external repository paths, or compatibility forwarding layers.

## Detailed authority

- [Detailed architecture](docs/architecture.md)
- [Numerical contract](docs/numerics.md)
- [Query model](docs/query-model.md)
- [Ownership rules](docs/ownership.md)
- [Extraction provenance](docs/provenance/runenwerk-extraction.md)
- [Current status](docs/status-map.md)
- [Roadmap](docs/roadmap.md)

Historical source identity is evidence only. Current architecture authority belongs to `dornglut/runen-sdf`.
