| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.022x | 0.953x | 1.251x |
| materialize | 1.031x | 0.968x | 1.142x |
| perft_bulk_d4 | 0.888x | 0.829x | 1.124x |
| perft_full_d3 | 1.053x | 0.916x | 0.621x |
| fen_parse | 0.830x | 0.764x | 1.153x |
| fen_write_native_ep | 0.962x | 0.914x | 1.237x |
| snapshot | 2.586x | 2.485x | 1.011x |
| san_parse_all | 0.951x | 0.939x | 1.771x |
| san_render_native | 0.896x | 0.868x | 3.285x |
| apply_children_full | 1.037x | 0.955x | 0.635x |
| make_unmake_full | 0.834x | 0.780x | 0.851x |
| startpos | 1.004x | 1.004x | 1.246x |
| chess960_materialize | 1.057x | 1.048x | 1.189x |
| chess960_perft_d3 | 0.910x | 0.865x | 1.184x |
| polyglot_key | 0.785x | 0.621x | 30.634x |
| polyglot_recompute | 1.010x | 0.987x | 0.979x |
| repetition_key | 0.559x | 0.545x | 15.161x |
| corpus_count | 1.012x | 1.006x | 1.217x |
| corpus_materialize | 1.038x | 0.951x | 1.086x |
| corpus_uci_sorted | 1.004x | 0.995x | 0.947x |
| replay_trusted_native | 1.052x | 0.899x | 0.509x |
| replay_checked_native | 0.712x | 0.589x | 0.701x |

Cold first legal move list: 0.021 ms -> 0.021 ms (0.97x).
