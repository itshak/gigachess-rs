| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.026x | 0.980x | 1.564x |
| materialize | 0.985x | 0.945x | 1.116x |
| perft_bulk_d4 | 1.032x | 0.989x | 1.483x |
| perft_full_d3 | 1.291x | 1.091x | 1.724x |
| fen_parse | 0.958x | 0.742x | 1.328x |
| fen_write_native_ep | 0.995x | 0.921x | 1.559x |
| snapshot | 0.994x | 0.979x | 1.160x |
| san_parse_all | 1.002x | 0.996x | 1.976x |
| san_render_native | 1.015x | 0.993x | 3.353x |
| apply_children_full | 1.023x | 1.001x | 1.347x |
| make_unmake_full | 1.010x | 0.932x | 1.307x |
| startpos | 0.997x | 0.997x | 1.002x |
| chess960_materialize | 0.982x | 0.957x | 1.332x |
| chess960_perft_d3 | 1.013x | 0.994x | 1.566x |
| polyglot_key | 0.987x | 0.977x | 20.564x |
| polyglot_recompute | 1.010x | 0.996x | 0.976x |
| repetition_key | 1.003x | 0.973x | 19.121x |
| corpus_count | 1.028x | 1.025x | 1.367x |
| corpus_materialize | 0.995x | 0.994x | 1.197x |
| corpus_uci_sorted | 1.026x | 1.020x | 1.510x |
| replay_trusted_native | 0.928x | 0.867x | 1.009x |
| replay_checked_native | 0.958x | 0.926x | 1.047x |

Cold first legal move list: 0.018 ms -> 0.019 ms (0.96x).
