| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.879x | 1.445x |
| materialize | 0.967x | 0.913x | 1.196x |
| perft_bulk_d4 | 0.988x | 0.912x | 1.289x |
| perft_full_d3 | 1.308x | 1.033x | 1.628x |
| fen_parse | 0.916x | 0.806x | 1.134x |
| fen_write_native_ep | 0.961x | 0.797x | 1.489x |
| snapshot | 0.993x | 0.974x | 1.494x |
| san_parse_all | 0.982x | 0.956x | 1.738x |
| san_render_native | 0.985x | 0.919x | 2.259x |
| apply_children_full | 0.996x | 0.982x | 1.355x |
| make_unmake_full | 1.003x | 0.935x | 1.207x |
| startpos | 0.971x | 0.971x | 1.249x |
| chess960_materialize | 0.994x | 0.988x | 1.458x |
| chess960_perft_d3 | 1.004x | 0.998x | 1.520x |
| polyglot_key | 1.002x | 0.978x | 34.576x |
| polyglot_recompute | 0.993x | 0.989x | 0.957x |
| repetition_key | 1.016x | 0.991x | 27.585x |
| corpus_count | 0.938x | 0.928x | 1.125x |
| corpus_materialize | 0.983x | 0.972x | 1.126x |
| corpus_uci_sorted | 0.987x | 0.963x | 1.874x |
| replay_trusted_native | 0.980x | 0.966x | 0.738x |
| replay_checked_native | 0.977x | 0.971x | 1.185x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.01x).
