| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.022x | 1.000x | 1.576x |
| materialize | 1.011x | 0.991x | 1.156x |
| perft_bulk_d4 | 1.038x | 0.985x | 1.458x |
| perft_full_d3 | 1.313x | 1.071x | 1.690x |
| fen_parse | 1.088x | 1.055x | 1.474x |
| fen_write_native_ep | 0.996x | 0.989x | 1.597x |
| snapshot | 1.023x | 0.982x | 1.203x |
| san_parse_all | 1.019x | 0.995x | 1.952x |
| san_render_native | 1.019x | 1.000x | 3.349x |
| apply_children_full | 1.027x | 0.995x | 1.523x |
| make_unmake_full | 1.008x | 0.949x | 1.272x |
| startpos | 0.995x | 0.995x | 1.176x |
| chess960_materialize | 1.012x | 1.001x | 1.348x |
| chess960_perft_d3 | 1.003x | 1.000x | 1.601x |
| polyglot_key | 1.009x | 0.993x | 20.269x |
| polyglot_recompute | 1.024x | 0.999x | 0.997x |
| repetition_key | 1.004x | 0.992x | 19.019x |
| corpus_count | 1.063x | 1.048x | 1.391x |
| corpus_materialize | 1.008x | 1.001x | 1.192x |
| corpus_uci_sorted | 1.012x | 0.995x | 1.509x |
| replay_trusted_native | 0.943x | 0.863x | 1.031x |
| replay_checked_native | 1.023x | 1.016x | 1.107x |

Cold first legal move list: 0.017 ms -> 0.018 ms (0.93x).
