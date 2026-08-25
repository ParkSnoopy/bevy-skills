# GitHub Copilot — Bevy 0.19

Copilot does not load Agent Skills directly. This file mirrors the key rules so Copilot users in this repo still benefit.

## When writing Bevy code

1. **Bevy 0.19 only.** Do not suggest older APIs except while explaining a migration. Specifically:
   - Events use `MessageWriter` / `MessageReader` (not the older
     `EventWriter` / `EventReader`; double-check the migration skills when
     reviewing stale code).
   - Resources are components; scene serialization uses `WorldAsset` / `WorldAssetRoot` and the `bevy_world_serialization` feature.
   - Text styling uses `FontSource` and `FontSize` components.
   - Required components use `#[require(...)]` on `Component` derives.

2. **Feature collections.** When configuring `Cargo.toml`, pick the right Bevy feature set: `2d`, `3d`, `ui` (high-level) or mid-level `2d_api`, `3d_api`, `ui_api`. See `skills/bevy-cargo-features/SKILL.md`.

3. **Schedule placement.** Frame-driven game/presentation logic goes in `Update`;
   fixed-step physics and deterministic simulation go in `FixedUpdate`.
   Render-world systems belong in Bevy's render schedules, not `Update`.

4. **No `unwrap()` in systems.** Bevy systems run every frame. Use `let ... else` or proper error propagation.

## When editing skills in this repo

Read `CLAUDE.md`. The frontmatter rules, lint script, and tester-crate workflow apply to all agents, not just Claude.

## Where to look

| Topic | Skill |
|---|---|
| ECS basics | `skills/bevy-core-concepts/` |
| Components & required components | `skills/bevy-ecs-components/` |
| Queries & filters | `skills/bevy-ecs-queries/` |
| Systems, sets, run conditions | `skills/bevy-ecs-systems/` |
| 0.18 → 0.19 breaks | `skills/bevy-migration-0-18-to-0-19/` |
| 0.17 → 0.18 breaks | `skills/bevy-migration-0-17-to-0-18/` |
| Assets & custom loaders | `skills/bevy-assets/`, `skills/bevy-custom-assets/` |
| WASM + WebGPU | `skills/bevy-wasm-webgpu/` |
| Cameras | `skills/bevy-cameras/` |
| PBR / materials | `skills/bevy-pbr-materials/` |
| Renderer selection / custom passes | `skills/bevy-rendering/` |
| Physics / Rapier / collision queries | `skills/bevy-physics/` |
| Game accessibility / adaptive controllers | `skills/bevy-a11y/` |
