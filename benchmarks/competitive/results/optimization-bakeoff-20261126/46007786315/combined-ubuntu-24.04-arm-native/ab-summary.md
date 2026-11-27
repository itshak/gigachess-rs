| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.968x | 1.537x |
| materialize | 1.008x | 0.997x | 1.440x |
| perft_bulk_d4 | 1.017x | 0.977x | 1.527x |
| perft_full_d3 | 1.156x | 1.017x | 1.404x |
| fen_parse | 1.001x | 0.985x | 1.088x |
| fen_write_native_ep | 1.007x | 0.997x | 1.607x |
| snapshot | 1.004x | 1.001x | 0.983x |
| san_parse_all | 1.001x | 0.997x | 1.995x |
| san_render_native | 1.000x | 0.998x | 2.073x |
| apply_children_full | 1.008x | 0.992x | 1.037x |
| make_unmake_full | 1.009x | 0.984x | 1.663x |
| startpos | 1.001x | 1.001x | 1.336x |
| chess960_materialize | 1.010x | 0.996x | 1.507x |
| chess960_perft_d3 | 1.008x | 0.998x | 1.726x |
| polyglot_key | 0.972x | 0.943x | 38.165x |
| polyglot_recompute | 1.028x | 1.002x | 1.081x |
| repetition_key | 0.998x | 0.991x | 30.360x |
| corpus_count | 1.067x | 1.032x | 1.496x |
| corpus_materialize | 0.938x | 0.880x | 1.290x |
| corpus_uci_sorted | 0.994x | 0.993x | 1.545x |
| replay_trusted_native | 1.034x | 1.029x | 0.682x |
| replay_checked_native | 1.024x | 1.013x | 1.281x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.07x).
