| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.999x | 1.292x |
| materialize | 1.006x | 1.001x | 1.217x |
| perft_bulk_d4 | 0.998x | 0.996x | 1.274x |
| perft_full_d3 | 0.998x | 0.984x | 0.751x |
| fen_parse | 0.994x | 0.983x | 1.227x |
| fen_write_native_ep | 1.511x | 1.413x | 1.586x |
| snapshot | 1.112x | 1.108x | 1.017x |
| san_parse_all | 0.999x | 0.991x | 1.796x |
| san_render_native | 1.006x | 0.998x | 2.422x |
| apply_children_full | 1.013x | 1.004x | 0.828x |
| make_unmake_full | 1.002x | 0.994x | 1.157x |
| startpos | 1.107x | 1.107x | 1.206x |
| chess960_materialize | 1.001x | 0.995x | 1.234x |
| chess960_perft_d3 | 1.003x | 1.001x | 1.224x |
| polyglot_key | 1.017x | 1.012x | 25.536x |
| polyglot_recompute | 1.004x | 0.997x | 1.072x |
| repetition_key | 1.031x | 1.031x | 19.057x |
| corpus_count | 1.006x | 0.997x | 1.401x |
| corpus_materialize | 1.005x | 1.000x | 1.210x |
| corpus_uci_sorted | 1.007x | 1.005x | 0.972x |
| replay_trusted_native | 0.984x | 0.976x | 0.407x |
| replay_checked_native | 0.992x | 0.992x | 0.897x |

Cold first legal move list: 0.013 ms -> 0.013 ms (1.05x).
