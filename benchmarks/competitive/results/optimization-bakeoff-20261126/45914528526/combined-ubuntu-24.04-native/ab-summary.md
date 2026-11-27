| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.984x | 1.481x |
| materialize | 0.995x | 0.986x | 1.569x |
| perft_bulk_d4 | 0.991x | 0.983x | 1.154x |
| perft_full_d3 | 1.007x | 1.001x | 1.347x |
| fen_parse | 0.923x | 0.828x | 1.064x |
| fen_write_native_ep | 1.011x | 0.963x | 2.130x |
| snapshot | 0.998x | 0.994x | 1.153x |
| san_parse_all | 1.008x | 0.985x | 2.025x |
| san_render_native | 0.990x | 0.979x | 2.193x |
| apply_children_full | 1.011x | 1.000x | 1.355x |
| make_unmake_full | 0.948x | 0.918x | 1.130x |
| startpos | 0.894x | 0.894x | 1.117x |
| chess960_materialize | 0.998x | 0.989x | 1.555x |
| chess960_perft_d3 | 1.008x | 1.000x | 1.510x |
| polyglot_key | 0.921x | 0.849x | 36.378x |
| polyglot_recompute | 1.017x | 1.011x | 1.017x |
| repetition_key | 1.151x | 0.964x | 22.438x |
| corpus_count | 1.001x | 0.999x | 1.087x |
| corpus_materialize | 1.014x | 1.013x | 1.073x |
| corpus_uci_sorted | 0.990x | 0.989x | 1.562x |
| replay_trusted_native | 1.046x | 1.012x | 0.929x |
| replay_checked_native | 0.999x | 0.979x | 1.172x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.98x).
