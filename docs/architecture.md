# RunenSDF Architecture

RunenSDF is a host-neutral Rust framework for validated signed-field mathematics and CPU reference queries.

It owns field samples and capabilities, conservative tracing steps, validated bounds and rays, primitives, operators, transforms, differential helpers, CPU queries, and structured errors.

It does not own Runenwerk adapters, ECS, world storage or streaming, scene state, materials, rendering, GPU or shader code, native windows, UI integration, or persisted product formats.

Dependency direction:

```text
runen-sdf -> glam, thiserror
Runenwerk adapters -> runen-sdf
runen-sdf -/-> Runenwerk
```

The repository contains one public root package. Supporting workspace packages exist only for downstream conformance and repository validation.
