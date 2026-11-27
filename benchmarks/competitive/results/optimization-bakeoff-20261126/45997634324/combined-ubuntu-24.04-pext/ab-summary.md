| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 1.003x | 1.628x |
| materialize | 0.942x | 0.910x | 1.574x |
| perft_bulk_d4 | 1.058x | 1.039x | 1.251x |
| perft_full_d3 | 1.250x | 1.059x | 1.675x |
| fen_parse | 0.992x | 0.987x | 1.066x |
| fen_write_native_ep | 1.016x | 0.995x | 2.139x |
| snapshot | 0.997x | 0.990x | 1.152x |
| san_parse_all | 1.011x | 0.998x | 2.021x |
| san_render_native | 1.016x | 1.011x | 2.126x |
| apply_children_full | 0.990x | 0.953x | 1.320x |
| make_unmake_full | 0.982x | 0.939x | 1.163x |
| startpos | 1.108x | 1.108x | 1.103x |
| chess960_materialize | 0.928x | 0.909x | 1.389x |
| chess960_perft_d3 | 1.017x | 0.993x | 1.499x |
| polyglot_key | 1.017x | 1.008x | 38.726x |
| polyglot_recompute | 0.978x | 0.978x | 0.972x |
| repetition_key | 0.825x | 0.698x | 19.048x |
| corpus_count | 1.147x | 1.116x | 1.287x |
| corpus_materialize | 1.010x | 1.010x | 1.076x |
| corpus_uci_sorted | 1.022x | 1.020x | 1.587x |
| replay_trusted_native | 1.002x | 0.959x | 0.910x |
| replay_checked_native | 0.980x | 0.976x | 1.181x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.01x).
