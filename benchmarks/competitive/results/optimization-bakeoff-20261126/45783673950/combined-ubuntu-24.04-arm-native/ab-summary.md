| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.355x |
| materialize | 1.012x | 1.009x | 1.244x |
| perft_bulk_d4 | 0.999x | 0.996x | 1.343x |
| perft_full_d3 | 0.954x | 0.908x | 1.004x |
| fen_parse | 0.996x | 0.990x | 1.245x |
| fen_write_native_ep | 1.005x | 1.001x | 1.056x |
| snapshot | 1.112x | 1.109x | 1.016x |
| san_parse_all | 1.002x | 1.000x | 1.822x |
| san_render_native | 1.004x | 0.998x | 2.546x |
| apply_children_full | 1.018x | 1.010x | 1.034x |
| make_unmake_full | 1.002x | 0.995x | 1.444x |
| startpos | 1.004x | 1.004x | 1.174x |
| chess960_materialize | 1.011x | 1.009x | 1.328x |
| chess960_perft_d3 | 0.999x | 0.995x | 1.318x |
| polyglot_key | 0.994x | 0.981x | 25.389x |
| polyglot_recompute | 0.996x | 0.993x | 1.071x |
| repetition_key | 1.002x | 1.000x | 19.022x |
| corpus_count | 0.992x | 0.990x | 1.470x |
| corpus_materialize | 1.008x | 1.003x | 1.209x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.457x |
| replay_trusted_native | 0.972x | 0.954x | 0.492x |
| replay_checked_native | 1.001x | 1.001x | 0.990x |

Cold first legal move list: 0.011 ms -> 0.011 ms (0.99x).
