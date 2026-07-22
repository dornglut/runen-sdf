# Work Tracking

## Active objective

Maintain the accepted standalone baseline while Runenwerk completes its separately
owned clean cutover.

## Completed

1. canonical identity, licensing, security, provenance, and ownership;
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
10. standalone parity review and merge as
    `d52badefc640d6dc6dcdd40268af3aea1bb8eefe`;
11. native repository validation in run `29895816472` on candidate
    `2d02f66a7b88addf8d871c88e79489591c92e079`.

## Remaining repository action

Merge the authority correction after its final exact-head CI run passes.

## External work

Runenwerk consumer audit, dependency selection, `domain/sdf` retirement, lockfile
cleanup, and duplicate-authority proof belong to `PT-RUNENSDF-004` in
`Crystonix/runenwerk`.

Module regrouping, publication, GPU work, rendering, ECS, UI, and persisted formats
remain excluded.
