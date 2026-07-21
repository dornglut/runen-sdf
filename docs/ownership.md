# Ownership Boundary

RunenSDF owns reusable signed-field mathematics and CPU query policy.

RunenSDF owns:

- field samples, capabilities, and errors;
- validated bounds and rays;
- analytic primitives;
- boolean and smooth composition;
- repeat, mirror, clamp, and warp wrappers;
- translation, rotation, uniform scale, and affine transforms;
- gradients and normals;
- classification, projection, closest-point, raymarch, and sphere-sweep queries.

RunenSDF does not own:

- Runenwerk adapters or product policy;
- ECS components, resources, mutation, or scheduling;
- world chunks, streaming, scene state, or procgen orchestration;
- material semantics, render planning, GPU resources, WGPU, or shaders;
- UI, windowing, networking, or persisted field/program formats.

Cross-domain conversion belongs to the consumer or an explicit consumer-owned adapter. The framework never depends back on an adapter.
