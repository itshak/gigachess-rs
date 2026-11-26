| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.992x | 1.153x |
| materialize | 1.090x | 1.063x | 1.632x |
| perft_bulk_d4 | 0.998x | 0.996x | 1.140x |
| perft_full_d3 | 1.004x | 1.001x | 0.995x |
| fen_parse | 0.954x | 0.930x | 1.106x |
| fen_write_native_ep | 1.509x | 1.386x | 1.876x |
| snapshot | 0.895x | 0.893x | 1.000x |
| san_parse_all | 1.051x | 1.022x | 1.768x |
| san_render_native | 0.998x | 0.989x | 2.084x |
| apply_children_full | 0.998x | 0.979x | 1.009x |
| make_unmake_full | 1.001x | 0.995x | 0.971x |
| startpos | 1.002x | 1.002x | 1.357x |
| chess960_materialize | 0.999x | 0.992x | 1.365x |
| chess960_perft_d3 | 1.003x | 0.999x | 1.291x |
| polyglot_key | 1.012x | 1.000x | 41.225x |
| polyglot_recompute | 0.981x | 0.974x | 0.962x |
| repetition_key | 1.073x | 0.996x | 23.760x |
| corpus_count | 1.002x | 1.002x | 1.193x |
| corpus_materialize | 1.009x | 1.001x | 1.160x |
| corpus_uci_sorted | 0.996x | 0.993x | 0.969x |
| replay_trusted_native | 1.017x | 1.002x | 0.593x |
| replay_checked_native | 1.012x | 0.997x | 0.974x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.94x).
