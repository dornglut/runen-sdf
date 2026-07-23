# RunenSDF

RunenSDF is a host-neutral Rust library for validated signed-field mathematics and deterministic CPU reference queries.

```text
repository: dornglut/runen-sdf
package: runen-sdf
crate: runen_sdf
version: 0.1.0
MSRV: Rust 1.93.0
publication: disabled
```

The framework owns finite signed-field samples, explicit capabilities, conservative tracing steps, validated bounds and rays, analytic primitives, composition, transforms, gradients, normals, and structured CPU query outcomes.

It does not own Runenwerk integration, ECS state, world streaming, materials, rendering, GPU resources, shaders, UI, or persisted product formats.

## Public contract

```rust
use glam::Vec3;
use runen_sdf::primitives::SdfSphere;
use runen_sdf::queries::raymarch::{RaymarchSettings, raymarch_first_hit};
use runen_sdf::queries::QueryOutcome;
use runen_sdf::Ray3;

let sphere = SdfSphere::new(Vec3::ZERO, 1.0)?;
let ray = Ray3::try_new(Vec3::new(-3.0, 0.0, 0.0), Vec3::X)?;
let outcome = raymarch_first_hit(&sphere, &ray, RaymarchSettings::default())?;
assert!(matches!(outcome, QueryOutcome::Hit(_)));
# Ok::<(), Box<dyn std::error::Error>>(())
```

A sample's `signed_value` preserves inside/surface/outside sign but is not universally exact Euclidean distance. `safe_step` is absent or a proven finite non-negative conservative tracing step. Sphere tracing consumes only `safe_step`; metric queries require exact-distance capability.

## Validation

`cargo validate` is the single maintained validation authority. CI invokes the same command and checks locked metadata, dependency direction, formatting, all tests, downstream conformance, Clippy, rustdoc, Rust 1.93.0, licenses, provenance, links, and clean repository state.

## Authority

- [Architecture](docs/architecture.md)
- [Numerical contract](docs/numerics.md)
- [Query model](docs/query-model.md)
- [Ownership](docs/ownership.md)
- [Status map](docs/status-map.md)
- [Roadmap](docs/roadmap.md)
- [Work tracking](docs/work-tracking.md)
- [Extraction provenance](docs/provenance/runenwerk-extraction.md)
- [Validation](docs/tooling/validation.md)

The transferred source baseline is Runenwerk commit `8de096259eab30f8d67672010df9190970d0bfc4`, path `domain/sdf`. Historical owner paths are retained only where provenance requires them; current authority uses the `dornglut/*` namespace.
