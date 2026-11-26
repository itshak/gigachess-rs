| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.984x | 1.347x |
| materialize | 1.046x | 1.030x | 1.122x |
| perft_bulk_d4 | 0.997x | 0.939x | 1.219x |
| perft_full_d3 | 1.006x | 0.965x | 1.036x |
| fen_parse | 0.979x | 0.956x | 1.185x |
| fen_write_native_ep | 1.003x | 1.000x | 1.924x |
| snapshot | 1.001x | 0.996x | 1.249x |
| san_parse_all | 0.997x | 0.984x | 2.256x |
| san_render_native | 0.998x | 0.958x | 2.102x |
| apply_children_full | 1.041x | 1.008x | 1.327x |
| make_unmake_full | 0.974x | 0.922x | 1.181x |
| startpos | 1.000x | 1.000x | 1.203x |
| chess960_materialize | 1.010x | 1.007x | 1.559x |
| chess960_perft_d3 | 0.998x | 0.996x | 1.491x |
| polyglot_key | 0.999x | 0.995x | 20.341x |
| polyglot_recompute | 0.998x | 0.993x | 1.017x |
| repetition_key | 0.998x | 0.985x | 18.952x |
| corpus_count | 1.007x | 1.006x | 1.203x |
| corpus_materialize | 1.001x | 1.000x | 1.204x |
| corpus_uci_sorted | 1.036x | 1.021x | 1.680x |
| replay_trusted_native | 0.950x | 0.934x | 1.110x |
| replay_checked_native | 0.998x | 0.993x | 1.015x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.00x).
