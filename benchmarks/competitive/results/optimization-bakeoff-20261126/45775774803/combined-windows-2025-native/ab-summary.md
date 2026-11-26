| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.988x | 1.305x |
| materialize | 0.998x | 0.988x | 1.119x |
| perft_bulk_d4 | 1.010x | 1.002x | 1.348x |
| perft_full_d3 | 1.012x | 0.999x | 0.630x |
| fen_parse | 0.799x | 0.776x | 1.183x |
| fen_write_native_ep | 1.001x | 0.997x | 1.233x |
| snapshot | 0.398x | 0.395x | 0.947x |
| san_parse_all | 1.053x | 1.031x | 1.838x |
| san_render_native | 0.996x | 0.987x | 3.332x |
| apply_children_full | 1.014x | 0.997x | 1.075x |
| make_unmake_full | 1.024x | 1.001x | 0.921x |
| startpos | 1.001x | 1.001x | 2.755x |
| chess960_materialize | 1.001x | 1.000x | 1.199x |
| chess960_perft_d3 | 0.996x | 0.990x | 1.387x |
| polyglot_key | 1.007x | 1.000x | 20.378x |
| polyglot_recompute | 1.031x | 1.004x | 0.960x |
| repetition_key | 1.074x | 1.000x | 17.969x |
| corpus_count | 1.013x | 1.006x | 1.303x |
| corpus_materialize | 1.000x | 0.994x | 1.147x |
| corpus_uci_sorted | 1.023x | 1.001x | 0.989x |
| replay_trusted_native | 1.026x | 1.009x | 0.517x |
| replay_checked_native | 1.013x | 0.990x | 0.883x |

Cold first legal move list: 0.027 ms -> 0.029 ms (0.93x).
