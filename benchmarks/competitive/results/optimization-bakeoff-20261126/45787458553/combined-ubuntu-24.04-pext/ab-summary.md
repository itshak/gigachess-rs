| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.999x | 1.449x |
| materialize | 0.958x | 0.932x | 1.233x |
| perft_bulk_d4 | 0.953x | 0.930x | 1.258x |
| perft_full_d3 | 0.916x | 0.853x | 0.879x |
| fen_parse | 0.899x | 0.863x | 1.059x |
| fen_write_native_ep | 1.003x | 0.994x | 1.838x |
| snapshot | 1.235x | 1.225x | 1.251x |
| san_parse_all | 0.943x | 0.910x | 2.010x |
| san_render_native | 0.961x | 0.931x | 2.032x |
| apply_children_full | 0.988x | 0.948x | 0.972x |
| make_unmake_full | 0.904x | 0.867x | 1.083x |
| startpos | 0.915x | 0.915x | 1.001x |
| chess960_materialize | 1.008x | 1.006x | 1.529x |
| chess960_perft_d3 | 0.980x | 0.953x | 1.450x |
| polyglot_key | 1.001x | 0.999x | 20.624x |
| polyglot_recompute | 1.046x | 1.033x | 1.035x |
| repetition_key | 1.063x | 1.003x | 17.949x |
| corpus_count | 0.970x | 0.945x | 1.422x |
| corpus_materialize | 1.016x | 0.995x | 1.304x |
| corpus_uci_sorted | 1.022x | 1.016x | 1.691x |
| replay_trusted_native | 1.050x | 0.976x | 0.815x |
| replay_checked_native | 0.944x | 0.937x | 0.924x |

Cold first legal move list: 0.012 ms -> 0.012 ms (1.02x).
