| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.998x | 1.084x |
| materialize | 0.998x | 0.997x | 1.172x |
| perft_bulk_d4 | 1.018x | 0.995x | 0.902x |
| perft_full_d3 | 1.001x | 0.998x | 0.601x |
| fen_parse | 1.011x | 1.001x | 1.207x |
| fen_write_native_ep | 1.012x | 1.007x | 1.058x |
| snapshot | 1.016x | 1.003x | 0.752x |
| san_parse_all | 0.990x | 0.984x | 0.911x |
| san_render_native | 1.011x | 1.000x | 1.160x |
| apply_children_full | 0.971x | 0.964x | 0.642x |
| make_unmake_full | 1.008x | 0.996x | 0.903x |
| startpos | 1.073x | 1.073x | 0.728x |
| chess960_materialize | 0.999x | 0.998x | 1.109x |
| chess960_perft_d3 | 0.995x | 0.986x | 0.836x |
| polyglot_key | 0.995x | 0.991x | 23.646x |
| polyglot_recompute | 0.995x | 0.992x | 1.123x |
| repetition_key | 0.996x | 0.992x | 14.417x |
| corpus_count | 1.017x | 1.001x | 1.217x |
| corpus_materialize | 1.007x | 1.006x | 1.117x |
| corpus_uci_sorted | 1.011x | 1.010x | 0.571x |
| replay_trusted_native | 1.000x | 0.993x | 0.323x |
| replay_checked_native | 1.010x | 1.009x | 0.662x |

Cold first legal move list: 153.975 ms → 154.059 ms (1.00x).
