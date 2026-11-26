| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.940x | 0.903x | 1.198x |
| materialize | 0.945x | 0.925x | 1.678x |
| perft_bulk_d4 | 0.870x | 0.796x | 1.030x |
| perft_full_d3 | 1.052x | 0.946x | 1.402x |
| fen_parse | 0.966x | 0.906x | 1.058x |
| fen_write_native_ep | 0.915x | 0.865x | 1.706x |
| snapshot | 1.001x | 0.997x | 1.000x |
| san_parse_all | 0.953x | 0.928x | 1.799x |
| san_render_native | 0.940x | 0.870x | 2.034x |
| apply_children_full | 0.921x | 0.813x | 1.295x |
| make_unmake_full | 0.779x | 0.686x | 0.967x |
| startpos | 1.402x | 1.402x | 1.999x |
| chess960_materialize | 0.935x | 0.890x | 1.449x |
| chess960_perft_d3 | 0.833x | 0.804x | 1.258x |
| polyglot_key | 0.950x | 0.903x | 38.005x |
| polyglot_recompute | 0.990x | 0.984x | 0.946x |
| repetition_key | 1.082x | 0.926x | 21.902x |
| corpus_count | 0.965x | 0.962x | 1.095x |
| corpus_materialize | 0.976x | 0.961x | 1.105x |
| corpus_uci_sorted | 1.000x | 0.999x | 1.553x |
| replay_trusted_native | 0.969x | 0.893x | 0.838x |
| replay_checked_native | 0.920x | 0.902x | 1.067x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.87x).
