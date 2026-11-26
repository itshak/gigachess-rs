| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.993x | 0.893x | 1.523x |
| materialize | 1.036x | 0.879x | 1.391x |
| perft_bulk_d4 | 0.971x | 0.807x | 1.132x |
| perft_full_d3 | 1.010x | 0.974x | 1.443x |
| fen_parse | 0.957x | 0.906x | 1.218x |
| fen_write_native_ep | 0.998x | 0.978x | 1.686x |
| snapshot | 1.129x | 1.106x | 1.131x |
| san_parse_all | 0.999x | 0.972x | 1.761x |
| san_render_native | 1.015x | 0.977x | 4.092x |
| apply_children_full | 0.804x | 0.758x | 1.076x |
| make_unmake_full | 0.877x | 0.824x | 1.058x |
| startpos | 0.998x | 0.998x | 0.998x |
| chess960_materialize | 0.984x | 0.948x | 1.336x |
| chess960_perft_d3 | 1.001x | 0.992x | 1.500x |
| polyglot_key | 1.019x | 0.998x | 40.936x |
| polyglot_recompute | 1.011x | 1.009x | 1.018x |
| repetition_key | 1.278x | 1.264x | 33.616x |
| corpus_count | 1.004x | 0.985x | 0.960x |
| corpus_materialize | 1.028x | 1.011x | 1.076x |
| corpus_uci_sorted | 1.019x | 0.999x | 1.543x |
| replay_trusted_native | 0.867x | 0.787x | 0.698x |
| replay_checked_native | 0.916x | 0.904x | 1.091x |

Cold first legal move list: 0.018 ms -> 0.024 ms (0.74x).
