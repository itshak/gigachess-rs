| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.988x | 1.571x |
| materialize | 1.016x | 1.009x | 1.339x |
| perft_bulk_d4 | 1.039x | 1.018x | 1.454x |
| perft_full_d3 | 1.139x | 1.009x | 1.520x |
| fen_parse | 1.013x | 0.943x | 1.227x |
| fen_write_native_ep | 0.994x | 0.992x | 1.568x |
| snapshot | 0.998x | 0.995x | 0.936x |
| san_parse_all | 1.001x | 0.995x | 1.821x |
| san_render_native | 1.004x | 0.992x | 2.682x |
| apply_children_full | 1.000x | 0.980x | 1.160x |
| make_unmake_full | 1.012x | 0.984x | 1.566x |
| startpos | 1.000x | 1.000x | 1.278x |
| chess960_materialize | 1.005x | 1.002x | 1.504x |
| chess960_perft_d3 | 1.018x | 1.008x | 1.519x |
| polyglot_key | 1.019x | 1.009x | 25.892x |
| polyglot_recompute | 1.293x | 1.212x | 1.401x |
| repetition_key | 1.010x | 0.999x | 19.433x |
| corpus_count | 1.098x | 1.078x | 1.505x |
| corpus_materialize | 1.011x | 1.006x | 1.232x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.454x |
| replay_trusted_native | 0.996x | 0.993x | 0.622x |
| replay_checked_native | 1.019x | 1.018x | 1.145x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.84x).
