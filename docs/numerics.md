# Numerical Contract

## Signed value

Every successful sample contains a finite `signed_value`:

```text
negative  inside
zero      on the zero set
positive  outside
```

The magnitude is not universally exact Euclidean distance.

## Conservative tracing step

`safe_step` is either absent or a finite non-negative lower bound to the nearest zero-set crossing. It must never exceed the distance justified by the field implementation.

Sphere tracing uses only `safe_step`. It never substitutes `abs(signed_value)` when a proof is absent.

## Capabilities

Exact primitives expose exact-distance capability. Translation, validated rotation, and uniform scale preserve it. Affine transforms retain a conservative step and remove exactness. Hard booleans retain only conservative step information. Smooth operations, clamp, warp, repeat, and mirror remove unproven metric capability.

## Validated state

Public construction rejects non-finite values, invalid dimensions, zero plane normals, invalid rays, zero scales, singular affine transforms, invalid ranges, non-positive epsilons, and invalid query budgets.

`FieldBounds::Empty` and `FieldBounds::Unbounded` are distinct. Disjoint finite intersection is `Empty`.
