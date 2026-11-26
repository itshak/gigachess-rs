| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.010x | 0.939x | 1.320x |
| materialize | 1.031x | 0.987x | 1.149x |
| perft_bulk_d4 | 0.940x | 0.879x | 1.235x |
| perft_full_d3 | 1.230x | 1.144x | 1.224x |
| fen_parse | 1.089x | 1.048x | 1.338x |
| fen_write_native_ep | 1.004x | 0.980x | 1.507x |
| snapshot | 1.443x | 1.397x | 1.500x |
| san_parse_all | 1.004x | 0.960x | 1.974x |
| san_render_native | 1.062x | 1.000x | 3.826x |
| apply_children_full | 1.205x | 1.149x | 1.364x |
| make_unmake_full | 1.016x | 0.996x | 1.250x |
| startpos | 1.017x | 1.017x | 1.250x |
| chess960_materialize | 1.170x | 1.114x | 1.284x |
| chess960_perft_d3 | 1.062x | 0.979x | 1.404x |
| polyglot_key | 0.889x | 0.861x | 33.769x |
| polyglot_recompute | 1.004x | 0.970x | 0.962x |
| repetition_key | 1.380x | 1.360x | 23.095x |
| corpus_count | 0.894x | 0.771x | 1.096x |
| corpus_materialize | 0.976x | 0.960x | 1.060x |
| corpus_uci_sorted | 1.011x | 1.008x | 1.379x |
| replay_trusted_native | 1.011x | 1.002x | 0.612x |
| replay_checked_native | 1.010x | 0.991x | 0.912x |

Cold first legal move list: 0.016 ms -> 0.009 ms (1.83x).
