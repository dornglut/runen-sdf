# Work Tracking

## Active objective

Complete standalone parity with the corrected Runenwerk SDF package and make GitHub Actions the only merge-validation gate.

## Ordered work

1. complete repository identity, licensing, security, provenance, and validation authority;
2. let the self-hosted bootstrap workflow generate and commit the initial independent lockfile;
3. replace bootstrap mutation with durable read-only CI invoking `cargo validate`;
4. transfer the corrected source from Runenwerk commit `8de096259eab30f8d67672010df9190970d0bfc4` without behavioral redesign;
5. migrate crate imports and all nine integration-test modules;
6. replace the downstream stub with public trait, trait-object, unsupported-capability, and query proof;
7. migrate framework-owned API, numerical, query, and ownership documentation;
8. pass stable and Rust 1.93.0 validation through GitHub Actions;
9. review exact source parity and record the accepted standalone revision;
10. prepare the separately authorized Runenwerk cutover phase.

Module-vocabulary cleanup remains deferred until standalone parity is independently green. No compatibility aliases are introduced.
