# Work Tracking

## Active objective

Maintain the accepted standalone RunenSDF baseline while `dornglut/runenwerk`
completes its separately owned clean cutover under issue `#133`.

## Completed

1. canonical package identity, licensing, security, provenance, and ownership;
2. exact corrected source transfer from Runenwerk commit
   `8de096259eab30f8d67672010df9190970d0bfc4`;
3. all nine integration-test modules with only the `sdf` to `runen_sdf` import
   migration;
4. downstream public trait, trait-object, invalid-construction,
   unsupported-capability, and successful-query proof;
5. framework API, numerical, query, and ownership documentation;
6. committed independent lockfile;
7. one maintained `cargo validate` authority;
8. durable repository CI invoking that authority;
9. automated mirror validation in runs `29845971330` and `29846386222`;
10. standalone parity review and source-transfer merge as
    `d52badefc640d6dc6dcdd40268af3aea1bb8eefe`;
11. native repository validation in run `29895816472`;
12. standalone authority closeout through PR `#2`;
13. shared organization validation adoption through PR `#4`.

## Current repository responsibility

- preserve one public root package and its downstream conformance proof;
- keep source provenance and validation authority accurate;
- reject compatibility packages, source mirrors, Runenwerk dependencies, and stale
  active owner paths;
- do not absorb Runenwerk cutover work into this repository.

## External work

Runenwerk owns the exact reverse-dependency census, dependency decision,
`domain/sdf` retirement, workspace and lockfile cleanup, active-authority cleanup,
and duplicate-authority proof.

Current evidence indicates the internal package may have no live product consumer.
If the complete census confirms that result, Runenwerk must delete the package
without adding an unused dependency on RunenSDF.

Module regrouping, publication, GPU work, rendering, ECS, UI, and persisted formats
remain excluded.
