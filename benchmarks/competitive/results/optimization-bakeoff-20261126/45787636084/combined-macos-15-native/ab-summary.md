| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.910x | 0.764x | 1.294x |
| materialize | 0.960x | 0.774x | 1.036x |
| perft_bulk_d4 | 0.666x | 0.601x | 1.006x |
| perft_full_d3 | 0.900x | 0.673x | 1.289x |
| fen_parse | 0.902x | 0.620x | 1.198x |
| fen_write_native_ep | 0.924x | 0.828x | 1.753x |
| snapshot | 0.883x | 0.696x | 0.920x |
| san_parse_all | 0.851x | 0.785x | 1.853x |
| san_render_native | 0.895x | 0.674x | 2.603x |
| apply_children_full | 0.909x | 0.704x | 0.978x |
| make_unmake_full | 0.942x | 0.756x | 1.237x |
| startpos | 0.942x | 0.942x | 1.374x |
| chess960_materialize | 0.827x | 0.783x | 1.161x |
| chess960_perft_d3 | 0.703x | 0.652x | 1.233x |
| polyglot_key | 0.909x | 0.900x | 39.860x |
| polyglot_recompute | 0.994x | 0.979x | 1.009x |
| repetition_key | 1.006x | 0.984x | 20.101x |
| corpus_count | 0.754x | 0.653x | 0.964x |
| corpus_materialize | 0.990x | 0.944x | 1.288x |
| corpus_uci_sorted | 0.802x | 0.759x | 1.365x |
| replay_trusted_native | 0.942x | 0.763x | 0.611x |
| replay_checked_native | 0.921x | 0.856x | 0.926x |

Cold first legal move list: 0.014 ms -> 0.011 ms (1.33x).
