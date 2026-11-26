| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.084x |
| materialize | 1.002x | 0.999x | 1.172x |
| perft_bulk_d4 | 1.011x | 1.002x | 0.914x |
| perft_full_d3 | 1.052x | 1.039x | 0.641x |
| fen_parse | 1.012x | 1.004x | 1.212x |
| fen_write_native_ep | 1.003x | 0.996x | 1.082x |
| snapshot | 1.002x | 0.998x | 0.803x |
| san_parse_all | 0.998x | 0.988x | 0.916x |
| san_render_native | 1.001x | 0.996x | 1.143x |
| apply_children_full | 1.000x | 0.986x | 0.654x |
| make_unmake_full | 1.005x | 1.001x | 0.901x |
| startpos | 0.992x | 0.992x | 0.831x |
| chess960_materialize | 1.001x | 1.001x | 1.109x |
| chess960_perft_d3 | 1.054x | 1.045x | 0.888x |
| polyglot_key | 1.019x | 0.996x | 23.216x |
| polyglot_recompute | 0.998x | 0.996x | 1.129x |
| repetition_key | 1.022x | 1.002x | 14.030x |
| corpus_count | 0.997x | 0.993x | 1.189x |
| corpus_materialize | 0.997x | 0.993x | 1.117x |
| corpus_uci_sorted | 0.956x | 0.955x | 0.541x |
| replay_trusted_native | 0.989x | 0.979x | 0.317x |
| replay_checked_native | 1.003x | 1.003x | 0.661x |

Cold first legal move list: 154.139 ms → 153.815 ms (1.00x).
