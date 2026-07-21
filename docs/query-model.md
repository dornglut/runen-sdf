# Query Model

RunenSDF queries distinguish successful hits, valid terminal misses, and errors.

```text
Hit(value)
Miss(OutsideBounds | SurfaceRuledOut | MaxDistanceReached |
     StepBudgetExhausted | ConvergenceBudgetExhausted |
     InsufficientProgress)
Error(InvalidInput | Sample | Gradient | UnsupportedCapability)
```

Raymarching requires conservative-step capability. Projection, closest-point, and sphere sweep require exact-distance capability. Unsupported capability is an error rather than an ordinary miss.

Every sample is validated. Non-finite evaluation, unusable gradients, invalid settings, bounds exits, distance limits, and budget exhaustion remain distinguishable.

Primary normal estimation never fabricates a fallback direction.
