| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.989x | 1.408x |
| materialize | 0.997x | 0.985x | 1.503x |
| perft_bulk_d4 | 1.005x | 0.994x | 1.205x |
| perft_full_d3 | 0.965x | 0.914x | 1.478x |
| fen_parse | 0.942x | 0.859x | 0.972x |
| fen_write_native_ep | 1.013x | 0.965x | 2.160x |
| snapshot | 0.704x | 0.693x | 1.144x |
| san_parse_all | 0.931x | 0.893x | 1.796x |
| san_render_native | 1.007x | 0.991x | 2.179x |
| apply_children_full | 0.933x | 0.887x | 1.048x |
| make_unmake_full | 0.948x | 0.915x | 1.060x |
| startpos | 1.016x | 1.016x | 1.292x |
| chess960_materialize | 0.916x | 0.890x | 1.318x |
| chess960_perft_d3 | 0.988x | 0.977x | 1.366x |
| polyglot_key | 0.999x | 0.998x | 41.500x |
| polyglot_recompute | 0.992x | 0.990x | 1.056x |
| repetition_key | 0.978x | 0.761x | 21.227x |
| corpus_count | 1.003x | 0.999x | 1.329x |
| corpus_materialize | 1.003x | 0.997x | 1.093x |
| corpus_uci_sorted | 1.015x | 1.010x | 1.569x |
| replay_trusted_native | 0.928x | 0.913x | 0.665x |
| replay_checked_native | 0.972x | 0.943x | 1.116x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.87x).
