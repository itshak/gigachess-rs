| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.985x | 1.397x |
| materialize | 1.002x | 0.998x | 1.507x |
| perft_bulk_d4 | 1.001x | 0.996x | 1.221x |
| perft_full_d3 | 0.998x | 0.991x | 1.473x |
| fen_parse | 1.071x | 1.031x | 1.014x |
| fen_write_native_ep | 0.987x | 0.971x | 1.367x |
| snapshot | 1.002x | 0.997x | 1.153x |
| san_parse_all | 0.934x | 0.925x | 1.889x |
| san_render_native | 1.000x | 0.998x | 2.098x |
| apply_children_full | 1.000x | 0.995x | 1.095x |
| make_unmake_full | 0.997x | 0.985x | 1.151x |
| startpos | 1.001x | 1.001x | 1.215x |
| chess960_materialize | 0.991x | 0.983x | 1.431x |
| chess960_perft_d3 | 0.995x | 0.992x | 1.378x |
| polyglot_key | 1.004x | 1.000x | 41.305x |
| polyglot_recompute | 1.001x | 0.929x | 0.995x |
| repetition_key | 1.083x | 0.883x | 21.985x |
| corpus_count | 0.993x | 0.991x | 1.326x |
| corpus_materialize | 1.011x | 1.010x | 1.081x |
| corpus_uci_sorted | 0.998x | 0.994x | 1.573x |
| replay_trusted_native | 0.998x | 0.997x | 0.715x |
| replay_checked_native | 1.017x | 1.007x | 1.092x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.11x).
