| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.989x | 0.854x | 1.293x |
| materialize | 1.106x | 1.026x | 1.287x |
| perft_bulk_d4 | 0.895x | 0.762x | 1.171x |
| perft_full_d3 | 1.052x | 0.941x | 1.247x |
| fen_parse | 0.994x | 0.926x | 1.218x |
| fen_write_native_ep | 1.007x | 0.930x | 1.679x |
| snapshot | 1.571x | 1.208x | 0.894x |
| san_parse_all | 0.953x | 0.901x | 2.098x |
| san_render_native | 0.969x | 0.898x | 3.595x |
| apply_children_full | 1.067x | 1.024x | 1.337x |
| make_unmake_full | 0.992x | 0.885x | 1.340x |
| startpos | 1.159x | 1.159x | 1.325x |
| chess960_materialize | 1.008x | 1.006x | 1.235x |
| chess960_perft_d3 | 1.043x | 0.984x | 1.407x |
| polyglot_key | 0.950x | 0.906x | 26.310x |
| polyglot_recompute | 0.956x | 0.936x | 0.997x |
| repetition_key | 0.986x | 0.896x | 19.295x |
| corpus_count | 0.844x | 0.726x | 1.205x |
| corpus_materialize | 0.891x | 0.886x | 1.111x |
| corpus_uci_sorted | 0.904x | 0.892x | 1.286x |
| replay_trusted_native | 0.996x | 0.945x | 0.752x |
| replay_checked_native | 0.952x | 0.903x | 1.048x |

Cold first legal move list: 0.033 ms -> 0.020 ms (1.68x).
