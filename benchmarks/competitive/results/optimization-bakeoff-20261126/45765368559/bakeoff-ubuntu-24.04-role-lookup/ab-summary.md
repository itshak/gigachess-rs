| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.996x | 1.188x |
| materialize | 1.093x | 1.015x | 1.392x |
| perft_bulk_d4 | 0.912x | 0.892x | 0.798x |
| perft_full_d3 | 0.776x | 0.742x | 0.480x |
| fen_parse | 0.900x | 0.877x | 1.040x |
| fen_write_native_ep | 1.023x | 0.978x | 1.244x |
| snapshot | 1.180x | 1.170x | 0.613x |
| san_parse_all | 0.922x | 0.896x | 0.910x |
| san_render_native | 0.925x | 0.913x | 0.988x |
| apply_children_full | 0.983x | 0.958x | 0.645x |
| make_unmake_full | 0.832x | 0.773x | 0.628x |
| startpos | 0.939x | 0.939x | 0.588x |
| chess960_materialize | 1.000x | 0.988x | 1.219x |
| chess960_perft_d3 | 0.912x | 0.907x | 0.935x |
| polyglot_key | 1.059x | 0.890x | 36.916x |
| polyglot_recompute | 0.979x | 0.976x | 0.957x |
| repetition_key | 1.132x | 1.041x | 15.285x |
| corpus_count | 1.010x | 1.008x | 1.163x |
| corpus_materialize | 1.010x | 1.005x | 1.066x |
| corpus_uci_sorted | 0.991x | 0.990x | 0.638x |
| replay_trusted_native | 1.059x | 0.896x | 0.545x |
| replay_checked_native | 0.847x | 0.767x | 0.653x |

Cold first legal move list: 138.291 ms → 143.651 ms (0.96x).
