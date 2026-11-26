| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.956x | 0.917x | 1.415x |
| materialize | 0.945x | 0.877x | 1.415x |
| perft_bulk_d4 | 0.807x | 0.625x | 1.020x |
| perft_full_d3 | 0.939x | 0.896x | 1.459x |
| fen_parse | 0.992x | 0.966x | 1.112x |
| fen_write_native_ep | 0.966x | 0.901x | 1.851x |
| snapshot | 0.913x | 0.910x | 1.154x |
| san_parse_all | 1.041x | 1.011x | 1.747x |
| san_render_native | 1.015x | 0.999x | 2.117x |
| apply_children_full | 1.019x | 1.004x | 1.093x |
| make_unmake_full | 1.003x | 0.991x | 1.154x |
| startpos | 0.997x | 0.997x | 1.288x |
| chess960_materialize | 0.921x | 0.889x | 1.342x |
| chess960_perft_d3 | 1.001x | 0.981x | 1.382x |
| polyglot_key | 1.127x | 0.999x | 37.395x |
| polyglot_recompute | 0.980x | 0.969x | 1.037x |
| repetition_key | 1.055x | 1.010x | 18.900x |
| corpus_count | 0.674x | 0.601x | 0.943x |
| corpus_materialize | 0.953x | 0.941x | 1.080x |
| corpus_uci_sorted | 0.970x | 0.961x | 1.551x |
| replay_trusted_native | 1.007x | 0.994x | 0.705x |
| replay_checked_native | 1.019x | 1.017x | 1.132x |

Cold first legal move list: 0.008 ms -> 0.005 ms (1.55x).
