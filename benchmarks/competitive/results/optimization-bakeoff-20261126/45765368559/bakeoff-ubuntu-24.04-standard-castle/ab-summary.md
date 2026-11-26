| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.997x | 1.185x |
| materialize | 0.920x | 0.817x | 1.296x |
| perft_bulk_d4 | 1.007x | 0.992x | 0.881x |
| perft_full_d3 | 1.006x | 0.981x | 0.617x |
| fen_parse | 0.920x | 0.813x | 1.019x |
| fen_write_native_ep | 1.010x | 0.969x | 1.371x |
| snapshot | 0.809x | 0.802x | 0.855x |
| san_parse_all | 0.986x | 0.974x | 0.908x |
| san_render_native | 0.994x | 0.972x | 1.060x |
| apply_children_full | 1.116x | 1.093x | 0.637x |
| make_unmake_full | 0.998x | 0.958x | 0.753x |
| startpos | 0.992x | 0.992x | 0.888x |
| chess960_materialize | 0.993x | 0.991x | 1.208x |
| chess960_perft_d3 | 0.990x | 0.989x | 0.952x |
| polyglot_key | 0.949x | 0.900x | 36.719x |
| polyglot_recompute | 1.000x | 0.998x | 1.023x |
| repetition_key | 0.962x | 0.924x | 14.600x |
| corpus_count | 0.943x | 0.916x | 1.092x |
| corpus_materialize | 0.994x | 0.990x | 1.073x |
| corpus_uci_sorted | 0.989x | 0.988x | 0.627x |
| replay_trusted_native | 0.949x | 0.946x | 0.513x |
| replay_checked_native | 0.998x | 0.974x | 0.803x |

Cold first legal move list: 133.309 ms → 133.564 ms (1.00x).
