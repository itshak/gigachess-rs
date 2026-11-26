| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.995x | 1.381x |
| materialize | 1.048x | 1.023x | 1.524x |
| perft_bulk_d4 | 0.998x | 0.981x | 1.169x |
| perft_full_d3 | 0.985x | 0.976x | 0.877x |
| fen_parse | 1.032x | 0.996x | 1.235x |
| fen_write_native_ep | 1.037x | 1.013x | 1.227x |
| snapshot | 1.820x | 1.809x | 1.151x |
| san_parse_all | 1.068x | 1.037x | 1.777x |
| san_render_native | 1.015x | 1.004x | 2.095x |
| apply_children_full | 0.945x | 0.918x | 0.994x |
| make_unmake_full | 0.993x | 0.980x | 0.938x |
| startpos | 1.344x | 1.344x | 1.130x |
| chess960_materialize | 0.933x | 0.931x | 1.302x |
| chess960_perft_d3 | 1.002x | 0.994x | 1.324x |
| polyglot_key | 0.982x | 0.971x | 38.616x |
| polyglot_recompute | 1.052x | 1.044x | 1.037x |
| repetition_key | 0.855x | 0.760x | 19.150x |
| corpus_count | 1.010x | 0.998x | 1.313x |
| corpus_materialize | 1.008x | 1.006x | 1.108x |
| corpus_uci_sorted | 1.007x | 1.004x | 0.968x |
| replay_trusted_native | 0.988x | 0.983x | 0.610x |
| replay_checked_native | 0.933x | 0.908x | 0.919x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.99x).
