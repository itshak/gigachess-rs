| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.965x | 1.391x |
| materialize | 0.997x | 0.981x | 1.368x |
| perft_bulk_d4 | 0.996x | 0.983x | 1.314x |
| perft_full_d3 | 1.013x | 1.005x | 1.265x |
| fen_parse | 0.859x | 0.827x | 1.063x |
| fen_write_native_ep | 1.160x | 1.006x | 1.663x |
| snapshot | 0.999x | 0.997x | 0.982x |
| san_parse_all | 1.009x | 0.993x | 2.006x |
| san_render_native | 1.036x | 0.987x | 2.320x |
| apply_children_full | 1.024x | 0.996x | 1.454x |
| make_unmake_full | 0.969x | 0.927x | 1.370x |
| startpos | 1.001x | 1.001x | 1.349x |
| chess960_materialize | 1.010x | 0.996x | 1.478x |
| chess960_perft_d3 | 1.003x | 1.000x | 1.583x |
| polyglot_key | 1.079x | 0.997x | 25.955x |
| polyglot_recompute | 1.012x | 1.007x | 1.043x |
| repetition_key | 1.031x | 0.995x | 19.024x |
| corpus_count | 1.002x | 0.994x | 1.292x |
| corpus_materialize | 1.005x | 1.001x | 1.219x |
| corpus_uci_sorted | 0.940x | 0.938x | 1.578x |
| replay_trusted_native | 0.999x | 0.995x | 0.925x |
| replay_checked_native | 0.978x | 0.972x | 1.139x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.10x).
