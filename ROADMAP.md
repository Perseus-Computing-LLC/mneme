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

## v0.1 — MVP

**Status:** ✅ Shipped (2026-05)

- SQLite + FTS5 keyword search with LIKE fallback
- MCP JSON-RPC 2.0 stdio server
- Three tools: `mimir_store`, `mimir_recall`, `mimir_health`
- Single static binary, bundled SQLite, zero runtime deps

---

## v0.2.0 — Structured Entity Model

**Status:** ✅ Shipped (2026-06-10)

### Three-table schema
- **entities** — idempotent by UNIQUE(category, key), FTS5-indexed
- **journal** — append-only event log with evaluated/acted/forward structure
- **state** — key-value with optional TTL and auto-expiration

### Entity tools
- `mimir_remember` — idempotent entity upsert by (category, key)
- `mimir_recall` — FTS5 search with category, type, topic, decay filters
- `mimir_forget` — soft-delete (archived=1) with reason
- `mimir_link` / `mimir_unlink` — entity relationship graph

### Journal tools
- `mimir_journal` — append structured events (decision/observation/action)
- `mimir_timeline` — time-range query with category/type/entity filters

### State tools
- `mimir_state_set` — key-value with optional TTL
- `mimir_state_get` — retrieve with auto-expiration check
- `mimir_state_delete` / `mimir_state_list` — management

### Management
- `mimir_stats` — full statistics across all three tables
- `mimir_compact` — archive entities below decay threshold
- `mimir_migrate` — CLI subcommand for v0.1.x → v0.2.0 migration
- `mimir_context` — pre-formatted markdown context block for session injection
- `mimir_workspace_list` — list all distinct categories

### Perseus integration
- Rewrote `mimir_connector.py` for entity model
- Removed Sibyl Memory dependency entirely
- Mimir is now the sole persistent memory backend for Perseus

---

## v1.0.0 — Intelligence & Distribution

**Status:** ✅ Shipped (2026-06-15)

This release transforms Mimir from a storage engine into an intelligent memory system.
Every v0.2.x and v0.5 goal was absorbed into this release (the intermediate version numbers
were skipped — v1.0.0 includes everything planned through v0.5 plus more).

### Confidence decay (was v0.2.1)
- Ebbinghaus decay algorithm: scores degrade over time, reset on recall
- Layer progression: buffer → working → core based on retrieval_count
- Near-duplicate detection via trigram similarity at store time
- `mimir_decay` tool for manual decay recalculation
- Auto-archive of stale entities via `mimir_compact`

### Semantic search (was v0.3)
- Hybrid search: FTS5 keyword + dense embeddings + RRF (Reciprocal Rank Fusion)
- Bundled embedding model via Ollama `/api/embed`
- Query expansion with Porter stemming for morphological variants
- `mimir_recall` with `search_mode`: fts5, dense, hybrid
- `mimir_embed` tool for explicit embedding generation

### Memory synthesis (was v0.5)
- Memory chain traversal via `mimir_traverse` (follow entity relationships)
- Quality scoring via `mimir_score` (agents rate memories 0-1)
- Conflict detection via `mimir_conflicts` (contradictory facts flagged)
- RAG via `mimir_ask` — NL Q&A with Ollama + cited sources

### Vault & portability
- `.md` vault export/import via `mimir_vault_export` / `mimir_vault_import`
- Human-readable, git-trackable, Obsidian-compatible markdown files
- SQLite remains the operational store; vault is the portable representation

### External connectors
- GitHub issues connector via `mimir_ingest`
- File watcher connector for watching directories
- Extensible connector framework for third-party data sources

### Security & operations
- AES-256-GCM encryption at rest for `body_json`
- `mimir migrate` subcommand for key generation
- Web dashboard (Axum HTTP server) with `--web --port` flags
- Entity graph visualization, search, stats in dashboard
- Smithery + Glama marketplace listings with full tool metadata

### Quality & polish
- Deep-dive code review (11 issues resolved)
- Second-pass review (10 issues resolved)
- Compiler warnings eliminated
- CI smoke-test workflow
- Claims audit against codebase
- Glama TDQS improvements (outputSchema + annotations)

**Total tools: 30 MCP tools**

---

## v1.1.0 — Distribution & Ecosystem (current)

**Target window:** Q3 2026 (Jul–Sep) · **Theme:** "Mimir everywhere."

### Integration guides (in progress)
- Claude Code integration guide
- Cursor integration guide
- LangGraph MimirStore adapter
- CrewAI MimirMemory provider
- AutoGen MimirContext plugin

### Transport expansion
- SSE/HTTP transport for non-stdio MCP hosts
- Docker image with pre-built binary (Alpine multi-stage)
- One-line install: `curl | bash` bootstrap verified on macOS, Linux, WSL

### Quality
- Glama TDQS score improvement (outputSchema + annotations on remaining tools)
- Smithery capability discovery fix (ensure all 30 tools appear)
- Windows CI in GitHub Actions
- Stress tests at 100K+ entity scale

### Discovery
- Submit to curated MCP server directories
- Appear in "awesome-mcp" lists
- Write comparison page vs Mem0, Sibyl, Holographic

---

## v1.2.0 — Multi-Agent & Federation

**Target window:** Q4 2026 (Oct–Dec) · **Theme:** "One memory engine, many agents, many workspaces."

- Workspace scoping with `workspace_hash`
- Agent identity tracking on stored memories
- Cross-workspace federation via vault sync
- Merge conflict resolution for concurrent writes
- Per-workspace access controls and visibility rules

---

## v1.3.0 — Offline Embeddings

**Target window:** Q1 2027 (Jan–Mar) · **Theme:** "Truly zero-dependency semantic search."

- Bundle all-MiniLM-L6-v2 via `ort` (ONNX Runtime) or `candle`
- Remove Ollama dependency for hybrid search
- Optional: still support external embedding endpoints
- 80MB binary size increase, zero network calls

---

## v2.0 — Platform

**Target window:** Q2 2027 (Apr–Jun) · **Theme:** "Mimir as infrastructure."

- gRPC transport alongside MCP
- Clustering with leader election
- Read replicas for high-availability deployments
- Audit log with cryptographic chaining
- Managed cloud option (Mimir Cloud)

---

## v2.1 — Federated Memory

**Target window:** Q3 2027 (Jul–Sep) · **Theme:** "Memory without borders."

- Cross-instance entity sync with configurable merge strategies (last-write-wins, CRDT-inspired, manual)
- Bidirectional `mimir_federate`: push and pull between any two Mimir instances
- Federation namespace scoping — sync only `category=X` or `workspace_hash=Y`
- Federation health dashboard: sync lag, conflict rate, entity drift
- Peer discovery: instances announce themselves on a local network or configured registry

---

## v2.2 — Memory Tiering

**Target window:** Q4 2027 (Oct–Dec) · **Theme:** "Infinite memory, finite resources."

- Hot/warm/cold storage tiers with automatic promotion/demotion
- Hot: SQLite (entities accessed in last 7 days, full-text indexed, <10ms recall)
- Warm: SQLite (entities 7-90 days, compressed body_json, <50ms recall)
- Cold: archive files on disk or object storage (>90 days, indexed metadata, <500ms recall)
- Transparent to agents — same MCP tools, documented latency per tier
- Configurable tier policies: per-category, per-workspace, per-entity-type
- `mimir_tier_stats` tool: size, entity count, access patterns per tier

---

## v2.3 — Memory Deduplication at Scale

**Target window:** Q1 2028 (Jan–Mar) · **Theme:** "One truth, many sources."

- Cross-workspace entity deduplication: collapse duplicate facts into canonical entities
- Dedup strategies: exact match, semantic near-match (embedding distance), key collision
- `mimir_dedup` tool with dry-run preview before merging
- Canonical entity carries highest-quality metadata (highest certainty, most recent update)
- Provenance links back to all source entities that were merged
- Dedup dashboard: duplicate rate, merge history, entity quality distribution

---

## v2.4 — Streaming Memory & Real-Time Sync

**Target window:** Q2 2028 (Apr–Jun) · **Theme:** "Memory that moves at the speed of thought."

- WebSocket and SSE transports for real-time memory updates
- Agents subscribe to entity changes: `mimir_subscribe(category="decision")`
- Push on write: Mimir notifies subscribers immediately, no polling
- CRDT-based sync between Mimir instances for offline-first multi-agent collaboration
- Sync topologies: mesh, hub-and-spoke, hierarchical
- Sync dashboard: per-instance lag, conflict rate, bandwidth usage

---

## v3.0 — Proactive Recall Engine

**Target window:** Q3 2028 (Jul–Sep) · **Theme:** "Mimir remembers so you don't have to."

- Mimir pushes relevant memories into context on session start — doesn't wait to be asked
- Semantic relevance scoring against task description with confidence calibration
- `recall_when` trigger system becomes the primary interface — context-aware, not keyword-dependent
- Pre-fetch on task start: query the task description, push top-N related entities before the first tool call
- Relevance feedback loop: agents signal whether pushed memories were useful; recall quality improves over time
- Configurable push budgets: max entities per session, per category, per confidence threshold

---

## v3.1 — Memory Synthesis Pipeline

**Target window:** Q4 2028 (Oct–Dec) · **Theme:** "Insight, not just recall."

- Pattern detection across entities: repeated decisions, recurring bugs, evolving approaches
- Temporal synthesis: "Over the last 3 months, your error handling has shifted from try/catch to Result types"
- `mimir_synthesize` tool: NL question → LLM-drafted insight → cited back to entity IDs
- Every synthesized claim links to its source entities — no uncited generation
- Synthesis cache: expensive synthesis runs once, reused across sessions with TTL
- Synthesis quality scoring: agents rate synthesized insights for accuracy and usefulness

---

## v3.2 — Forgetting Curves That Learn

**Target window:** Q1 2029 (Jan–Mar) · **Theme:** "Not all memories fade at the same rate."

- Ebbinghaus decay parameters self-tune per workspace, per agent, per entity type
- Some facts decay fast (yesterday's error message); some never decay (production password policy)
- Mimir learns which is which from retrieval patterns, agent corrections, and entity type
- `mimir_decay_policy` tool: inspect and override learned decay curves
- Decay dashboards: which entity types decay fastest? Which are sticky? Where is decay misconfigured?
- Adaptive retention: entities that keep getting retrieved slow their decay; entities that are ignored accelerate

---

## v3.3 — Causal Memory Graphs

**Target window:** Q2 2029 (Apr–Jun) · **Theme:** "Not just what happened, but why."

- Entities linked by causation: "Decision X caused bug Y which was fixed by PR Z"
- `mimir_traverse` follows causal chains in both directions: "what caused this?" and "what did this cause?"
- Automatic causal link detection: when entity B references entity A in the same session, suggest a causal link
- Causal graph visualization in the web dashboard
- Causal debug: "Why is this module structured this way? → follows chain back to the original architecture decision"
- Causal integrity: breaking a causal chain (deleting an entity) surfaces warnings about orphaned effects

---

## v4.0 — Mimir as a Protocol

**Target window:** Q3 2029 (Jul–Sep) · **Theme:** "Memory is bigger than one implementation."

- The Mimir entity model becomes an open, versioned standard with a formal specification
- Reference implementation (the Rust binary) + compliance test suite
- Anyone can implement a Mimir-compatible memory server in any language
- MCP tools remain the standard interface; storage format is documented and stable
- Protocol version negotiation: a v4.0 client talks to a v4.2 server
- Compliance certification: "Mimir Compatible" badge for third-party implementations

---

## v4.1 — Multi-Modal Memory

**Target window:** Q4 2029 (Oct–Dec) · **Theme:** "Not everything worth remembering is text."

- Store image embeddings, audio transcripts, code diffs — same entity model, same MCP tools
- `mimir_remember` accepts binary payloads with automatic embedding generation
- Cross-modal recall: "find the diagram about the auth flow" matches an image entity
- Modality-specific preview: thumbnails for images, playback metadata for audio, syntax highlighting for code
- Multi-modal embedding pipeline: ONNX for text, CLIP for images, Whisper for audio
- Modality filtering: `mimir_recall(modality="image")` scopes results

---

## v4.2 — Memory Compaction Pipelines

**Target window:** Q1 2030 (Jan–Mar) · **Theme:** "Millions of memories, zero clutter."

- Long-running projects accumulate millions of entities; compaction pipeline keeps the working set small
- Automatic summarization: N related entities → 1 synthesis entity (cited, reversible)
- Configurable compaction policies: by age, category, retrieval frequency
- Compaction is reversible — archives are queryable, just slower
- Working set target: ~10K entities in hot storage regardless of total stored
- `mimir_compact --strategy aggressive` for disk-constrained deployments
- Compaction preview: "This policy would archive 45,000 entities and keep 8,200 hot. Proceed?"

---

## v4.3 — Real-Time Memory Sync (Production)

**Target window:** Q2 2030 (Apr–Jun) · **Theme:** "Memory without borders, without latency."

- Production-grade CRDT sync across WAN (builds on v2.4 foundations)
- Vector clocks for causal ordering; automatic conflict resolution with manual override
- Sync topologies: mesh, hub-and-spoke, hierarchical — all production-hardened
- Bandwidth-aware: differential sync sends only changed entities
- Offline-first: agents work disconnected; sync automatically on reconnect with conflict resolution
- Sync SLAs: <1s propagation within region, <5s cross-region

---

## v5.0 — Mimir Global

**Target window:** Q3 2030 (Jul–Sep) · **Theme:** "A memory fabric for the planet."

- Global distributed memory fabric: agents anywhere remember facts anywhere
- Namespaced, encrypted, permissioned — universally accessible with the right keys
- Edge nodes: Mimir instances close to where agents run, syncing to regional hubs
- Latency SLAs: <10ms recall for hot entities, <100ms for warm, <1s for cold
- Global deduplication: a fact remembered in Tokyo is deduped against a fact in London
- Multi-region deployment: run your own regional hubs or use Mimir Cloud's

---

## v5.1 — Organizational Memory

**Target window:** Q4 2030 (Oct–Dec) · **Theme:** "What your company knows."

- Companies run Mimir clusters as organizational infrastructure
- Every employee's agent sessions contribute to a shared, permissioned memory
- Organizational memory graph: who knows what, which teams make which decisions
- Role-based access: some memories are public, some are team-private, some are individual
- "What did we learn from the last incident?" is a query, not a meeting
- Knowledge silo detection: "Team A and Team B have made conflicting decisions about API auth"

---

## v5.2 — Memory Analytics

**Target window:** Q1 2031 (Jan–Mar) · **Theme:** "Your memory has a story to tell."

- Dashboards over organizational memory: trends, conflicts, revisitations
- "Your team has made 47 decisions about API design in the last year. 12 conflict."
- Predictive analytics: "Based on memory patterns, you'll revisit auth architecture within 2 weeks"
- Knowledge coverage maps: which topics are well-documented in memory? Which are sparse?
- Memory health scoring: recall quality, entity freshness, conflict rate, synthesis accuracy
- Export to BI tools: Mimir as a data source for organizational intelligence

---

## v5.3 — Memory Interop Standard

**Target window:** Q2 2031 (Apr–Jun) · **Theme:** "The SQL of agent memory."

- Mimir's entity model published as an IETF RFC or equivalent standards body document
- Memory portability between vendors: move your agent's memories from Mimir to any compliant server
- Compliance test suite: any server can prove it's Mimir-compatible
- Reference implementation remains the Rust binary; protocol is the standard
- "Mimir Compatible" certification program for third-party implementations
- The standard is the moat — Mimir the product is the best implementation of the standard it created

---

## Design Principles

1. **Zero runtime dependencies.** The binary is self-contained.
2. **Offline-first.** All core operations work without internet.
3. **MCP-native.** Every feature ships as an MCP tool.
4. **Agent-first, not human-first.** Tools are designed for AI agents.
5. **Compose, don't integrate.** Mimir does persistent memory; composes with Perseus, Obsidian, Git.
6. **Local-first, cloud-optional.** Run it anywhere; cloud features are additive.
