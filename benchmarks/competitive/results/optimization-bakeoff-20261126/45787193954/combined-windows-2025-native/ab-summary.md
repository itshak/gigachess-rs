| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.899x | 1.350x |
| materialize | 1.005x | 0.931x | 1.053x |
| perft_bulk_d4 | 0.996x | 0.940x | 1.362x |
| perft_full_d3 | 0.996x | 0.933x | 1.094x |
| fen_parse | 1.201x | 1.100x | 1.463x |
| fen_write_native_ep | 0.951x | 0.875x | 1.578x |
| snapshot | 1.021x | 0.896x | 1.145x |
| san_parse_all | 0.942x | 0.693x | 1.904x |
| san_render_native | 1.008x | 0.908x | 3.379x |
| apply_children_full | 1.023x | 1.002x | 1.039x |
| make_unmake_full | 1.000x | 0.951x | 1.261x |
| startpos | 1.198x | 1.198x | 1.198x |
| chess960_materialize | 1.003x | 0.994x | 1.197x |
| chess960_perft_d3 | 1.007x | 0.997x | 1.421x |
| polyglot_key | 0.997x | 0.990x | 20.448x |
| polyglot_recompute | 0.989x | 0.980x | 0.999x |
| repetition_key | 0.940x | 0.886x | 15.843x |
| corpus_count | 0.993x | 0.988x | 1.320x |
| corpus_materialize | 1.004x | 0.997x | 1.175x |
| corpus_uci_sorted | 1.010x | 1.006x | 1.509x |
| replay_trusted_native | 0.998x | 0.995x | 0.733x |
| replay_checked_native | 1.014x | 1.008x | 0.938x |

Cold first legal move list: 0.024 ms -> 0.029 ms (0.83x).
