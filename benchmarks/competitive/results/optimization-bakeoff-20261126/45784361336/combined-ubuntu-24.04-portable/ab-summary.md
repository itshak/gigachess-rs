| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.197x |
| materialize | 0.996x | 0.980x | 1.652x |
| perft_bulk_d4 | 1.000x | 0.997x | 1.162x |
| perft_full_d3 | 0.996x | 0.994x | 1.482x |
| fen_parse | 1.132x | 1.010x | 1.201x |
| fen_write_native_ep | 0.997x | 0.993x | 2.044x |
| snapshot | 0.998x | 0.996x | 0.998x |
| san_parse_all | 1.103x | 1.069x | 1.929x |
| san_render_native | 0.997x | 0.992x | 2.125x |
| apply_children_full | 1.000x | 0.998x | 1.129x |
| make_unmake_full | 0.998x | 0.980x | 1.238x |
| startpos | 1.002x | 1.002x | 1.355x |
| chess960_materialize | 1.008x | 1.007x | 1.425x |
| chess960_perft_d3 | 0.998x | 0.995x | 1.335x |
| polyglot_key | 1.145x | 1.016x | 43.128x |
| polyglot_recompute | 1.039x | 1.037x | 1.037x |
| repetition_key | 1.134x | 0.950x | 24.182x |
| corpus_count | 1.002x | 1.000x | 1.267x |
| corpus_materialize | 0.986x | 0.985x | 1.156x |
| corpus_uci_sorted | 1.009x | 1.009x | 1.563x |
| replay_trusted_native | 0.995x | 0.995x | 0.680x |
| replay_checked_native | 1.003x | 1.002x | 1.070x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.05x).
