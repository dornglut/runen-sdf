# RunenSDF Architecture

RunenSDF is one independently useful public package with two non-public support packages: `conformance/downstream` and `xtask`.

```text
runen-sdf -> glam, thiserror
conformance/downstream -> runen-sdf public API
xtask -> standard library and repository tools
Runenwerk adapters -> runen-sdf
runen-sdf -/-> Runenwerk
```

The public package owns signed-field samples and capabilities, validated bounds and rays, primitives, operations, domain composition, transforms, gradients, normals, and deterministic CPU reference queries.

It does not own ECS, scheduling, world storage or streaming, scene state, materials, rendering, GPU execution, shaders, windows, UI, networking, or persisted product formats.

No façade, compatibility crate, private source inclusion, submodule, external path dependency, or speculative package decomposition is permitted.

See the [numerical contract](numerics.md), [query model](query-model.md), and [ownership boundary](ownership.md).
