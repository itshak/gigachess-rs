| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.053x | 1.001x | 1.365x |
| materialize | 1.012x | 0.998x | 1.779x |
| perft_bulk_d4 | 1.070x | 1.041x | 1.266x |
| perft_full_d3 | 1.149x | 1.013x | 1.538x |
| fen_parse | 1.027x | 0.927x | 1.186x |
| fen_write_native_ep | 1.001x | 0.975x | 2.069x |
| snapshot | 1.001x | 0.988x | 1.003x |
| san_parse_all | 1.002x | 0.985x | 1.908x |
| san_render_native | 0.978x | 0.971x | 2.130x |
| apply_children_full | 1.007x | 1.002x | 1.380x |
| make_unmake_full | 0.996x | 0.948x | 1.290x |
| startpos | 1.006x | 1.006x | 1.355x |
| chess960_materialize | 1.051x | 1.043x | 1.543x |
| chess960_perft_d3 | 1.016x | 0.995x | 1.520x |
| polyglot_key | 1.029x | 0.997x | 41.643x |
| polyglot_recompute | 1.610x | 1.281x | 1.525x |
| repetition_key | 0.979x | 0.975x | 22.467x |
| corpus_count | 1.137x | 1.107x | 1.342x |
| corpus_materialize | 1.010x | 1.007x | 1.139x |
| corpus_uci_sorted | 0.996x | 0.990x | 1.550x |
| replay_trusted_native | 1.007x | 0.983x | 0.903x |
| replay_checked_native | 1.026x | 1.014x | 1.203x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.90x).
