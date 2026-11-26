| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.866x | 1.385x |
| materialize | 1.005x | 0.859x | 1.483x |
| perft_bulk_d4 | 0.893x | 0.773x | 1.095x |
| perft_full_d3 | 0.973x | 0.940x | 1.448x |
| fen_parse | 0.935x | 0.866x | 1.064x |
| fen_write_native_ep | 1.017x | 1.002x | 2.042x |
| snapshot | 1.415x | 1.395x | 1.151x |
| san_parse_all | 0.991x | 0.970x | 1.910x |
| san_render_native | 0.995x | 0.985x | 2.176x |
| apply_children_full | 1.030x | 1.014x | 1.193x |
| make_unmake_full | 0.992x | 0.970x | 1.116x |
| startpos | 1.082x | 1.082x | 1.111x |
| chess960_materialize | 1.144x | 1.100x | 1.572x |
| chess960_perft_d3 | 1.054x | 0.981x | 1.448x |
| polyglot_key | 0.996x | 0.990x | 41.380x |
| polyglot_recompute | 1.042x | 1.027x | 1.053x |
| repetition_key | 0.948x | 0.728x | 18.838x |
| corpus_count | 0.659x | 0.552x | 0.868x |
| corpus_materialize | 0.986x | 0.983x | 1.077x |
| corpus_uci_sorted | 1.010x | 1.006x | 1.571x |
| replay_trusted_native | 1.003x | 0.997x | 0.812x |
| replay_checked_native | 1.006x | 0.995x | 1.152x |

Cold first legal move list: 0.009 ms -> 0.005 ms (1.70x).
