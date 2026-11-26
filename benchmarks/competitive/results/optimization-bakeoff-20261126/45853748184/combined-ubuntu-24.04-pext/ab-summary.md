| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.022x | 0.998x | 1.548x |
| materialize | 1.004x | 1.000x | 1.527x |
| perft_bulk_d4 | 1.013x | 0.999x | 1.187x |
| perft_full_d3 | 1.002x | 0.996x | 1.363x |
| fen_parse | 0.900x | 0.880x | 0.972x |
| fen_write_native_ep | 0.948x | 0.895x | 2.028x |
| snapshot | 1.000x | 0.995x | 1.163x |
| san_parse_all | 1.050x | 1.011x | 1.780x |
| san_render_native | 1.004x | 0.998x | 2.128x |
| apply_children_full | 0.970x | 0.954x | 1.265x |
| make_unmake_full | 0.936x | 0.860x | 1.109x |
| startpos | 0.997x | 0.997x | 1.115x |
| chess960_materialize | 0.998x | 0.998x | 1.419x |
| chess960_perft_d3 | 0.992x | 0.991x | 1.467x |
| polyglot_key | 1.003x | 0.998x | 40.743x |
| polyglot_recompute | 1.014x | 1.000x | 1.020x |
| repetition_key | 0.805x | 0.790x | 25.181x |
| corpus_count | 1.007x | 1.001x | 0.952x |
| corpus_materialize | 1.002x | 0.999x | 1.092x |
| corpus_uci_sorted | 1.011x | 1.010x | 1.563x |
| replay_trusted_native | 0.962x | 0.938x | 0.846x |
| replay_checked_native | 0.976x | 0.962x | 1.040x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.24x).
