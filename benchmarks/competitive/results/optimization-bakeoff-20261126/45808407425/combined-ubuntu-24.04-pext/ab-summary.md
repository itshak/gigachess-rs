| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.963x | 1.432x |
| materialize | 1.000x | 0.953x | 1.198x |
| perft_bulk_d4 | 1.006x | 0.945x | 1.312x |
| perft_full_d3 | 1.012x | 0.929x | 0.988x |
| fen_parse | 0.754x | 0.724x | 0.937x |
| fen_write_native_ep | 1.005x | 0.985x | 1.809x |
| snapshot | 1.083x | 1.062x | 1.205x |
| san_parse_all | 0.881x | 0.862x | 1.881x |
| san_render_native | 0.954x | 0.878x | 2.013x |
| apply_children_full | 0.899x | 0.824x | 1.035x |
| make_unmake_full | 0.832x | 0.731x | 1.015x |
| startpos | 1.274x | 1.274x | 1.506x |
| chess960_materialize | 1.009x | 0.988x | 1.498x |
| chess960_perft_d3 | 0.934x | 0.925x | 1.404x |
| polyglot_key | 1.103x | 1.000x | 22.418x |
| polyglot_recompute | 0.943x | 0.910x | 0.962x |
| repetition_key | 0.951x | 0.913x | 17.950x |
| corpus_count | 1.014x | 1.005x | 1.481x |
| corpus_materialize | 1.006x | 1.004x | 1.264x |
| corpus_uci_sorted | 0.985x | 0.980x | 1.644x |
| replay_trusted_native | 0.940x | 0.860x | 0.765x |
| replay_checked_native | 0.854x | 0.835x | 0.818x |

Cold first legal move list: 0.012 ms -> 0.011 ms (1.11x).
