| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.976x | 1.208x |
| materialize | 1.017x | 0.992x | 1.270x |
| perft_bulk_d4 | 0.998x | 0.990x | 1.176x |
| perft_full_d3 | 1.001x | 0.952x | 1.320x |
| fen_parse | 1.028x | 0.999x | 1.089x |
| fen_write_native_ep | 1.011x | 0.997x | 1.674x |
| snapshot | 1.001x | 1.000x | 1.016x |
| san_parse_all | 1.006x | 0.995x | 1.991x |
| san_render_native | 1.000x | 0.994x | 2.292x |
| apply_children_full | 0.991x | 0.971x | 1.369x |
| make_unmake_full | 0.983x | 0.960x | 1.399x |
| startpos | 1.001x | 1.001x | 1.344x |
| chess960_materialize | 1.054x | 1.008x | 1.329x |
| chess960_perft_d3 | 1.000x | 0.993x | 1.431x |
| polyglot_key | 1.057x | 0.998x | 27.643x |
| polyglot_recompute | 1.005x | 1.005x | 1.015x |
| repetition_key | 1.043x | 0.995x | 19.500x |
| corpus_count | 1.002x | 0.998x | 1.202x |
| corpus_materialize | 1.010x | 1.009x | 1.107x |
| corpus_uci_sorted | 1.007x | 1.005x | 1.828x |
| replay_trusted_native | 0.977x | 0.972x | 0.892x |
| replay_checked_native | 0.976x | 0.971x | 1.025x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.97x).
