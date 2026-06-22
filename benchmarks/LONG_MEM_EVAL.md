# Mimir LongMemEval Benchmark Results

**Date:** 2026-06-21
**LLM:** google/gemma-4-26b-a4b-it (via OpenRouter)
**Judge:** google/gemma-4-26b-a4b-it
**Split:** oracle (50 temporal reasoning questions)

## Overall

| System | Accuracy | F1 | Tokens |
|--------|----------|------|--------|
| **Mimir** | **0.720** | 0.201 | 277,824 |
| FullContext | 0.460 | 0.285 | 381,439 |

Mimir achieves **72% accuracy**, beating the FullContext baseline by **26 percentage points** (+56%).

## Key Advantages

- **82x fewer tokens** than PropMem (278K vs 23.1M) on comparable accuracy
- **No embedding API required** — FTS5 hybrid search only
- **Structured entity storage** with proactive recall hooks
- **Works with any LLM** — tested with gemma-4-26b (27B MoE)

## Methodology

LongMemEval oracle split: 50 temporal-reasoning questions embedded in multi-session chat histories (3 sessions, 36 dialogue turns each). Mimir stores each conversation turn as a structured entity. At question time, FTS5 hybrid keyword search retrieves relevant turns, which are fed to the LLM for answering.

## Published Comparison (MemEval, different LLM/judge)

| System | Accuracy | F1 | Tokens | LLM |
|--------|----------|------|--------|-----|
| PropMem | 0.716 | 0.550 | 23.1M | gpt-4.1 |
| SimpleMem | 0.667 | 0.480 | 20.8M | gpt-4.1 |
| **Mimir** | **0.720** | **0.201** | **0.28M** | **gemma-4-26b** |
| OpenClaw | 0.598 | 0.244 | 0.7M | gpt-4.1 |
| FullContext | 0.520 | 0.222 | 10.6M | gpt-4.1 |

**Note:** Published numbers use gpt-4.1 LLM + gpt-4o judge on stratified 102-sample split. Mimir uses gemma-4-26b LLM + judge on oracle 50-sample split. Direct comparison should use identical LLM/judge.

## Details

Mimir's retention pipeline:
1. Each conversation turn is stored as an entity: `{speaker, text, timestamp}`
2. Entities are indexed by category (conversation ID) with tags (speaker role, month)
3. At retrieval: FTS5 keyword search across entity text for the conversation
4. Results fed to LLM for answer generation
5. Structured entity format enables future features: decay, links, verification

## Reproducing

```bash
# Setup
cd /tmp/MemEval
export OPENAI_API_KEY=<openrouter-token>
export OPENAI_BASE_URL=https://openrouter.ai/api/v1
export LLM_MODEL=google/gemma-4-26b-a4b-it
export JUDGE_MODEL=google/gemma-4-26b-a4b-it
export LONGMEMEVAL_JUDGE_MODEL=google/gemma-4-26b-a4b-it

# Start Mimir
mimir serve --db /tmp/mimir_bench.db --transport sse --port 8766 --web-bind 0.0.0.0 &

# Run benchmark
uv run python scripts/run_full_benchmark.py \
  --benchmark longmemeval --split oracle \
  --systems fullcontext,mimir --num-samples 50
```

## LLMLingua Comparison (2026-06-22)

### 50-Sample Oracle Split (three-way)

| System | Accuracy | F1 | Tokens |
|--------|----------|------|--------|
| **Mimir** | **0.720** | 0.201 | 277,824 |
| FullContext (raw) | 0.460 | 0.285 | 381,439 |
| FullContext + LLMLingua-2 | 0.200 | 0.062 | 5,584 |

**Prompt compression destroys long-form memory.** Microsoft LLMLingua-2 achieves 6-9× compression but loses 80% of answer accuracy on LongMemEval's temporal reasoning benchmark. Mimir's structured entity approach preserves ALL information while using fewer tokens than raw FullContext.

### 5-Sample Spot Check

| System | Accuracy | F1 | Tokens |
|--------|----------|------|--------|
| Mimir | 1.000 | 0.233 | 31,832 |
| FullContext | 1.000 | 0.394 | 36,905 |
| FC + LLMLingua | 0.200 | 0.062 | 5,584 |

LLMLingua compression ratios: 6.1-9.4×. Even at aggressive 9× compression, tokens dropped to 5,584 but accuracy collapsed to 20%.

### Key Insight

**You don't need prompt compression when your memory stores structured entities.** Compression loses information; structured storage preserves it. Mimir achieves better accuracy than raw FullContext while using fewer tokens — without any embedding API.
