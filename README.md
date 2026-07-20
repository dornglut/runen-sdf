# RunenSDF

RunenSDF is a host-neutral Rust library for validated signed-field mathematics and CPU queries.

Canonical identity:

```text
repository: Crystonix/runen-sdf
package: runen-sdf
crate: runen_sdf
```

The framework owns field samples, conservative tracing steps, capabilities, bounds, rays, primitives, operators, transforms, differential helpers, and CPU queries. Runenwerk-specific geometry, ECS, world, renderer, material, GPU, UI, and product integration remain outside this repository.

The initial extraction source is Runenwerk commit `8de096259eab30f8d67672010df9190970d0bfc4`, path `domain/sdf`.

Current authority:

- [Architecture](docs/architecture.md)
- [Status map](docs/status-map.md)
- [Roadmap](docs/roadmap.md)
- [Work tracking](docs/work-tracking.md)
- [Extraction provenance](docs/provenance/runenwerk-extraction.md)
- [Validation](docs/tooling/validation.md)

This repository is not yet published to crates.io.
