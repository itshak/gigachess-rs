| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.025x | 0.992x | 1.332x |
| materialize | 1.002x | 0.996x | 1.738x |
| perft_bulk_d4 | 1.033x | 1.004x | 1.230x |
| perft_full_d3 | 0.990x | 0.969x | 1.321x |
| fen_parse | 1.005x | 0.959x | 1.227x |
| fen_write_native_ep | 1.021x | 0.975x | 1.938x |
| snapshot | 1.000x | 0.995x | 1.001x |
| san_parse_all | 0.925x | 0.888x | 1.811x |
| san_render_native | 0.974x | 0.953x | 2.164x |
| apply_children_full | 1.000x | 0.997x | 1.345x |
| make_unmake_full | 1.058x | 0.991x | 1.276x |
| startpos | 0.999x | 0.999x | 1.426x |
| chess960_materialize | 0.909x | 0.891x | 1.358x |
| chess960_perft_d3 | 1.002x | 0.995x | 1.505x |
| polyglot_key | 1.150x | 1.001x | 42.538x |
| polyglot_recompute | 0.997x | 0.995x | 1.009x |
| repetition_key | 1.051x | 0.999x | 21.743x |
| corpus_count | 1.130x | 1.084x | 1.255x |
| corpus_materialize | 1.000x | 0.996x | 1.139x |
| corpus_uci_sorted | 0.998x | 0.995x | 1.553x |
| replay_trusted_native | 0.998x | 0.993x | 0.911x |
| replay_checked_native | 1.006x | 0.994x | 1.163x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.91x).
