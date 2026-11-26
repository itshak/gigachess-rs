| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.379x |
| materialize | 0.996x | 0.995x | 1.258x |
| perft_bulk_d4 | 0.997x | 0.992x | 1.238x |
| perft_full_d3 | 1.001x | 0.996x | 1.052x |
| fen_parse | 1.003x | 0.999x | 1.211x |
| fen_write_native_ep | 1.017x | 1.012x | 1.580x |
| snapshot | 1.008x | 1.003x | 0.945x |
| san_parse_all | 0.998x | 0.996x | 1.815x |
| san_render_native | 1.004x | 0.993x | 2.576x |
| apply_children_full | 0.998x | 0.987x | 1.149x |
| make_unmake_full | 0.999x | 0.978x | 1.537x |
| startpos | 1.001x | 1.001x | 1.308x |
| chess960_materialize | 0.998x | 0.995x | 1.378x |
| chess960_perft_d3 | 1.003x | 0.990x | 1.351x |
| polyglot_key | 0.996x | 0.985x | 25.591x |
| polyglot_recompute | 1.004x | 1.002x | 1.088x |
| repetition_key | 1.012x | 1.004x | 19.118x |
| corpus_count | 1.002x | 0.999x | 1.191x |
| corpus_materialize | 1.000x | 1.000x | 1.176x |
| corpus_uci_sorted | 0.997x | 0.995x | 1.449x |
| replay_trusted_native | 0.991x | 0.983x | 0.604x |
| replay_checked_native | 1.055x | 1.038x | 1.105x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.04x).
