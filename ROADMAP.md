# Mimir Roadmap

## What Mimir Is

A local-first persistent memory engine for AI agents. MCP-native. Single static binary.
Zero runtime dependencies. Structured entity model with journal events and state management.

## What Mimir Is Not

- Not a knowledge graph or entity extraction engine
- Not a cloud service or SaaS
- Not a replacement for a vector database
- Not dependent on any specific AI assistant or framework

---

## Status — 2026-06-24

- **Latest release:** `v2.0.1`
- **`main`:** `2.1.0` (unreleased) — adds trust-aware recall ranking and top-level `--db`
- **MCP tools:** **40**, spanning entities, search/RAG, journal, state, graph, lifecycle, multi-agent/federation, and vault
- **In one line:** everything originally planned from v0.1 through the v2.0 "Platform" milestone has shipped. This document is being corrected to reflect that, and the forward section is deliberately short and honest.

> **Doc hygiene note:** prior revisions of this file listed shipped capabilities
> (federation, multi-agent scoping, gRPC, offline embeddings) as "future," and
> carried fabricated quarterly milestones through 2031 — several describing tools
> that already exist (`mimir_federate`, `mimir_synthesize`). That has been removed.
> Forward-looking work that is not committed now lives under **Exploratory** with
> no dates. The canonical roadmap is this file; `docs/ROADMAP.md` points here.

---

## Shipped

### v0.1 — MVP ✅ (2026-05)
- SQLite + FTS5 keyword search with LIKE fallback
- MCP JSON-RPC 2.0 stdio server; single static binary, bundled SQLite, zero runtime deps

### v0.2.0 — Structured entity model ✅ (2026-06-10)
- Three-table schema: **entities** (idempotent by `UNIQUE(category, key)`, FTS5-indexed),
  **journal** (append-only `evaluated/acted/forward` events), **state** (key-value + TTL)
- Entity tools (`remember`, `recall`, `forget`, `link`/`unlink`), journal (`journal`, `timeline`),
  state (`state_set/get/delete/list`), management (`stats`, `compact`, `migrate`, `context`, `workspace_list`)
- Became the sole persistent-memory backend for Perseus (Sibyl dependency removed)

### v1.0.0 — Intelligence & distribution ✅ (2026-06-15)
- **Confidence decay:** Ebbinghaus decay, `buffer → working → core` layering, trigram near-dup detection, `mimir_decay`
- **Hybrid search:** FTS5 + dense embeddings + Reciprocal Rank Fusion; Porter-stemming query expansion; `mimir_embed`
- **Synthesis:** chain traversal (`mimir_traverse`), quality scoring (`mimir_score`), conflict detection (`mimir_conflicts`), RAG (`mimir_ask`)
- **Vault & portability:** `.md` export/import (`mimir_vault_export`/`import`) — human-readable, git-trackable, Obsidian-compatible
- **Connectors:** GitHub issues + file watcher via `mimir_ingest`
- **Security & ops:** AES-256-GCM encryption at rest, web dashboard, Smithery/Glama listings

### v1.1 – v2.0 — Ecosystem, multi-agent, platform ✅ (2026-06)
- **Ecosystem:** framework adapters for **LangGraph, CrewAI, AutoGen** (`integrations/`), an **Obsidian plugin**,
  SSE/HTTP transport for non-stdio hosts, Docker image, and a one-line installer (`curl -sSf … | sh`, `v2.0.1`)
- **Multi-agent & federation:** workspace scoping (`workspace_hash`), agent identity (`agent_id`),
  per-entity `visibility`, and cross-instance sync via `mimir_federate`
- **Local/offline embeddings:** ONNX path via `ort` — hybrid search without an external embedding service
- **Platform (`v2.0.0`):** gRPC transport alongside MCP, and a cryptographically-chained audit log
- **Additional tools since the docs last counted:** `autocohere`, `bench`, `correct`, `supersede`,
  `synthesize`, `share`, `purge`, `maintenance`, `recall_when`, `get_entity` — **40 tools total**

### Unreleased on `main` (`2.1.0`)
- **Trust-aware recall:** `mimir_recall` ranks verified sources above unverified drafts
  (uses `verified`/`source`/`certainty`; on by default at a low weight). Consistent with `mimir_conflicts`.
- **CLI:** top-level `--db` accepted when running the server directly (`mimir --db <path>`),
  matching the documented MCP host config.

---

## Now — Foundation

**Theme: "what we ship matches what we say."** Stabilize the base before adding capability.

- **Single source of version truth:** `Cargo.toml`, the README badge, git tags, and this doc agree.
  (`Cargo.toml` had drifted *behind* the latest release tag.)
- **Doc accuracy:** correct the tool count (40), audit README capability claims against code,
  and collapse the duplicate `docs/ROADMAP.md`.
- **Cross-platform CI:** Windows in the matrix; resolve the gnu/mingw build gap so all three
  platforms are first-class (today the gnu toolchain can't build locally without mingw).
- **Release discipline:** start a `CHANGELOG.md`, commit to semver, and cut a clean `2.1.0`.

## Next — Remaining platform hardening

The genuinely-unshipped pieces of the "Mimir as infrastructure" goal:

- **Clustering / HA:** leader election and read replicas for high-availability deployments
  (the one part of the v2.0 platform theme not yet built).
- **Bundled-by-default offline embeddings:** ship the model in the binary rather than the current
  opt-in `ort` download — truly zero-network semantic search out of the box.
- **Scale:** 100K+ entity stress tests with documented recall latency budgets.
- **Federation maturation:** sync health/observability (lag, conflict rate, entity drift) for `mimir_federate`.

## Later — Gated & cross-product

- **Managed "Mimir Cloud":** a hosted/multi-region option — only after the platform hardening above.
- **Billing for hosted tiers via Plutus:** explicitly **gated on Plutus reaching 1.0** (stable, frozen
  API + DB schema). No integration code before then, to avoid churn against a moving contract.

## Exploratory — directional, not committed (no dates)

Ideas we like and may pursue. Listed to capture intent, **not** to promise delivery or timing:

- Memory tiering (hot/warm/cold storage with automatic promotion/demotion)
- Proactive recall — pre-fetch relevant entities on task start instead of waiting to be asked
- Learned forgetting curves — decay parameters that self-tune per workspace/agent/type
- Causal memory graphs — entities linked by causation, traversable in both directions
- Multi-modal memory — image/audio/code entities with cross-modal recall
- Production CRDT sync across WAN with conflict resolution
- An open, versioned "Mimir-compatible" memory standard + compliance suite

---

## Design Principles

1. **Zero runtime dependencies.** The binary is self-contained.
2. **Offline-first.** All core operations work without internet.
3. **MCP-native.** Every feature ships as an MCP tool.
4. **Agent-first, not human-first.** Tools are designed for AI agents.
5. **Compose, don't integrate.** Mimir does persistent memory; composes with Perseus, Obsidian, Git.
6. **Local-first, cloud-optional.** Run it anywhere; cloud features are additive.
