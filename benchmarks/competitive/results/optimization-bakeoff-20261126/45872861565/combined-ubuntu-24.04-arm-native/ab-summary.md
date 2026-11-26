| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.998x | 1.562x |
| materialize | 1.000x | 0.996x | 1.318x |
| perft_bulk_d4 | 1.004x | 0.994x | 1.409x |
| perft_full_d3 | 1.140x | 1.007x | 1.516x |
| fen_parse | 1.005x | 0.995x | 1.225x |
| fen_write_native_ep | 1.003x | 0.998x | 1.564x |
| snapshot | 0.987x | 0.982x | 0.959x |
| san_parse_all | 0.994x | 0.990x | 1.855x |
| san_render_native | 1.014x | 1.008x | 2.708x |
| apply_children_full | 0.996x | 0.980x | 1.153x |
| make_unmake_full | 1.017x | 1.004x | 1.588x |
| startpos | 1.002x | 1.002x | 1.334x |
| chess960_materialize | 1.003x | 0.991x | 1.494x |
| chess960_perft_d3 | 1.006x | 1.003x | 1.510x |
| polyglot_key | 1.000x | 0.989x | 25.606x |
| polyglot_recompute | 1.045x | 1.029x | 1.144x |
| repetition_key | 1.007x | 1.001x | 19.329x |
| corpus_count | 1.000x | 0.995x | 1.374x |
| corpus_materialize | 0.992x | 0.991x | 1.222x |
| corpus_uci_sorted | 0.992x | 0.985x | 1.448x |
| replay_trusted_native | 0.973x | 0.964x | 0.607x |
| replay_checked_native | 0.979x | 0.979x | 1.112x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.98x).
