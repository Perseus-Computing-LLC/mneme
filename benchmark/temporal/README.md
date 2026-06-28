# Mimir bi-temporal benchmark

A **reproducible, fully offline** check that Mimir's bi-temporal facts behave
correctly — that *time-travel returns the version that was actually live at a
past instant*, and that current recall never resurfaces a superseded one. It is
the temporal companion to the [`../recall/`](../recall/) recall-quality
benchmark.

> **Why this exists.** "Stop storing, start *maintaining*" is only a real claim
> if you can show it. When a fact is overwritten, Mimir keeps the prior version
> in history (`entity_history`) and can answer *"what did we believe at time
> T?"* via `mimir_as_of` — while normal recall stays live-only. This harness
> drives the **real shipped binary over MCP stdio** through fact updates and
> verifies exactly that, with a signed, re-runnable report anyone can reproduce
> with **no API key, no network, no LLM**.

## Run it

```bash
cargo build --release
python benchmark/temporal/run.py        # writes report.json, prints a summary
```

Exit code is non-zero if any check fails, so it can gate. `--dataset other.json`
plugs in a different scenario set; `--gap-ms N` widens the spacing between the
two writes.

## How it works

For each fact-update scenario it writes **v1**, marks a timestamp `t_mid`, writes
**v2** under the same `(category, key)` (which supersedes v1 into history), then
issues five checks against the real binary:

| Check | Asserts |
|---|---|
| `as_of_mid_returns_v1` | `mimir_as_of(t_mid)` returns the version live *between* the writes |
| `as_of_now_returns_v2` | `mimir_as_of(now)` returns the current version |
| `as_of_before_not_found` | `mimir_as_of(before-it-existed)` reports `found=false` |
| `recall_excludes_superseded_v1` | live `mimir_recall` never returns the superseded content |
| `recall_includes_live_v2` | live `mimir_recall` still returns the current content |

Wall-clock timestamps separate the two writes, so absolute times differ each run;
the **PASS/FAIL verdicts are deterministic**, and the `signature_sha256` is taken
over the verdicts (not the clock), so a correct implementation re-runs to an
identical signature. The committed [`report.json`](./report.json) is the
reference.

## The dataset

[`dataset.json`](./dataset.json) — `mimir-temporal-mini`: a handful of everyday
fact updates (a capital city, an employer, a default model, a commute). Each
carries a `v1_token`/`v2_token` (substrings unique to each version) and a
`probe` keyword for the recall checks. Add entries of the same shape to extend
coverage; the harness is dataset-agnostic.

## Results (committed [`report.json`](./report.json))

`mimir-temporal-mini`: **20/20 checks, 100% accuracy** — `as_of` returns the
correct version at every probed instant, and recall is live-only across all
scenarios. *(Measured on the bundled MSVC build, Windows 11; the methodology and
verdicts are the point and are platform-independent.)*
