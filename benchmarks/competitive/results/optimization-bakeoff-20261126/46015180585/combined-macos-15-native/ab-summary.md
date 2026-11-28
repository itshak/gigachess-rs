| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.112x | 0.758x | 1.619x |
| materialize | 1.033x | 0.926x | 1.059x |
| perft_bulk_d4 | 1.082x | 0.928x | 1.336x |
| perft_full_d3 | 1.099x | 0.915x | 1.443x |
| fen_parse | 1.060x | 0.815x | 1.207x |
| fen_write_native_ep | 1.020x | 0.866x | 1.657x |
| snapshot | 1.169x | 1.010x | 0.986x |
| san_parse_all | 0.978x | 0.780x | 1.806x |
| san_render_native | 1.061x | 0.834x | 2.822x |
| apply_children_full | 0.969x | 0.752x | 1.085x |
| make_unmake_full | 0.951x | 0.832x | 1.311x |
| startpos | 0.632x | 0.632x | 0.789x |
| chess960_materialize | 1.052x | 1.032x | 1.506x |
| chess960_perft_d3 | 0.936x | 0.882x | 1.454x |
| polyglot_key | 1.017x | 0.941x | 39.601x |
| polyglot_recompute | 1.486x | 1.408x | 1.638x |
| repetition_key | 0.861x | 0.812x | 18.943x |
| corpus_count | 0.926x | 0.852x | 1.212x |
| corpus_materialize | 1.027x | 1.012x | 1.259x |
| corpus_uci_sorted | 0.971x | 0.887x | 1.521x |
| replay_trusted_native | 0.943x | 0.787x | 0.646x |
| replay_checked_native | 0.846x | 0.801x | 1.220x |

Cold first legal move list: 0.006 ms -> 0.003 ms (1.67x).
