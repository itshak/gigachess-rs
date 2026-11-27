| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.028x | 0.935x | 1.492x |
| materialize | 0.990x | 0.912x | 1.286x |
| perft_bulk_d4 | 1.029x | 0.832x | 1.318x |
| perft_full_d3 | 1.010x | 0.990x | 1.314x |
| fen_parse | 1.090x | 1.025x | 1.349x |
| fen_write_native_ep | 0.987x | 0.950x | 1.619x |
| snapshot | 1.063x | 0.994x | 0.988x |
| san_parse_all | 1.018x | 0.873x | 2.028x |
| san_render_native | 1.060x | 0.996x | 3.910x |
| apply_children_full | 1.045x | 0.970x | 1.450x |
| make_unmake_full | 1.016x | 0.964x | 1.368x |
| startpos | 1.000x | 1.000x | 1.255x |
| chess960_materialize | 1.090x | 1.034x | 1.365x |
| chess960_perft_d3 | 1.043x | 1.022x | 1.580x |
| polyglot_key | 0.849x | 0.766x | 23.621x |
| polyglot_recompute | 1.031x | 0.997x | 1.014x |
| repetition_key | 1.104x | 1.056x | 21.048x |
| corpus_count | 1.006x | 0.995x | 1.441x |
| corpus_materialize | 0.906x | 0.812x | 1.048x |
| corpus_uci_sorted | 0.985x | 0.979x | 1.411x |
| replay_trusted_native | 1.007x | 0.988x | 0.890x |
| replay_checked_native | 0.963x | 0.946x | 1.068x |

Cold first legal move list: 0.018 ms -> 0.020 ms (0.92x).
