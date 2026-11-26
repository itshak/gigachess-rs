| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.982x | 1.394x |
| materialize | 1.007x | 1.004x | 1.185x |
| perft_bulk_d4 | 1.046x | 1.010x | 1.312x |
| perft_full_d3 | 1.053x | 1.018x | 1.032x |
| fen_parse | 1.181x | 1.125x | 1.264x |
| fen_write_native_ep | 1.001x | 0.997x | 1.829x |
| snapshot | 1.001x | 1.000x | 1.250x |
| san_parse_all | 1.000x | 0.987x | 2.217x |
| san_render_native | 1.011x | 0.995x | 2.119x |
| apply_children_full | 1.043x | 1.006x | 1.249x |
| make_unmake_full | 1.063x | 0.984x | 1.250x |
| startpos | 0.997x | 0.997x | 1.199x |
| chess960_materialize | 1.002x | 0.996x | 1.495x |
| chess960_perft_d3 | 1.014x | 1.007x | 1.499x |
| polyglot_key | 1.005x | 0.996x | 20.469x |
| polyglot_recompute | 0.993x | 0.991x | 1.024x |
| repetition_key | 0.995x | 0.991x | 15.471x |
| corpus_count | 1.000x | 1.000x | 1.252x |
| corpus_materialize | 0.997x | 0.995x | 1.256x |
| corpus_uci_sorted | 1.015x | 1.013x | 1.678x |
| replay_trusted_native | 1.092x | 1.069x | 1.215x |
| replay_checked_native | 0.976x | 0.974x | 1.037x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.98x).
