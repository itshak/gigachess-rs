| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.996x | 1.477x |
| materialize | 0.997x | 0.989x | 1.549x |
| perft_bulk_d4 | 0.994x | 0.979x | 1.252x |
| perft_full_d3 | 0.993x | 0.984x | 1.479x |
| fen_parse | 1.117x | 1.086x | 1.161x |
| fen_write_native_ep | 0.997x | 0.990x | 1.304x |
| snapshot | 1.409x | 1.381x | 1.149x |
| san_parse_all | 0.992x | 0.963x | 1.992x |
| san_render_native | 0.995x | 0.987x | 2.139x |
| apply_children_full | 1.014x | 1.000x | 1.067x |
| make_unmake_full | 1.018x | 1.005x | 1.152x |
| startpos | 1.225x | 1.225x | 1.122x |
| chess960_materialize | 1.130x | 1.122x | 1.472x |
| chess960_perft_d3 | 1.002x | 0.994x | 1.401x |
| polyglot_key | 1.003x | 0.999x | 40.896x |
| polyglot_recompute | 0.997x | 0.995x | 1.054x |
| repetition_key | 0.861x | 0.746x | 20.148x |
| corpus_count | 1.001x | 0.996x | 1.409x |
| corpus_materialize | 1.003x | 1.002x | 1.122x |
| corpus_uci_sorted | 0.990x | 0.981x | 1.574x |
| replay_trusted_native | 0.990x | 0.980x | 0.709x |
| replay_checked_native | 1.003x | 0.991x | 1.119x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.93x).
