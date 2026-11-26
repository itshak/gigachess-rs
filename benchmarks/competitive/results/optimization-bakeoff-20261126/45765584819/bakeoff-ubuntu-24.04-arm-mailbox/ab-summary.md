| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.997x | 1.083x |
| materialize | 0.998x | 0.995x | 1.170x |
| perft_bulk_d4 | 1.068x | 1.041x | 0.953x |
| perft_full_d3 | 1.027x | 0.962x | 0.624x |
| fen_parse | 0.990x | 0.985x | 1.183x |
| fen_write_native_ep | 0.989x | 0.979x | 1.058x |
| snapshot | 0.661x | 0.642x | 0.467x |
| san_parse_all | 1.012x | 0.995x | 0.939x |
| san_render_native | 0.988x | 0.946x | 1.138x |
| apply_children_full | 0.924x | 0.909x | 0.608x |
| make_unmake_full | 0.961x | 0.938x | 0.867x |
| startpos | 0.489x | 0.489x | 0.365x |
| chess960_materialize | 0.999x | 0.997x | 1.108x |
| chess960_perft_d3 | 1.035x | 1.013x | 0.867x |
| polyglot_key | 1.023x | 0.973x | 23.505x |
| polyglot_recompute | 0.982x | 0.970x | 1.109x |
| repetition_key | 1.028x | 1.000x | 14.374x |
| corpus_count | 0.958x | 0.940x | 1.148x |
| corpus_materialize | 0.983x | 0.973x | 1.095x |
| corpus_uci_sorted | 0.981x | 0.976x | 0.562x |
| replay_trusted_native | 1.085x | 1.047x | 0.349x |
| replay_checked_native | 0.996x | 0.993x | 0.667x |

Cold first legal move list: 154.147 ms → 153.884 ms (1.00x).
