| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.076x | 0.928x | 1.602x |
| materialize | 1.039x | 0.930x | 1.095x |
| perft_bulk_d4 | 1.084x | 1.020x | 1.299x |
| perft_full_d3 | 1.066x | 0.954x | 1.499x |
| fen_parse | 1.024x | 0.835x | 1.202x |
| fen_write_native_ep | 0.945x | 0.846x | 1.784x |
| snapshot | 0.860x | 0.563x | 0.920x |
| san_parse_all | 0.889x | 0.734x | 1.647x |
| san_render_native | 0.990x | 0.910x | 2.817x |
| apply_children_full | 1.000x | 0.870x | 1.109x |
| make_unmake_full | 1.054x | 0.944x | 1.323x |
| startpos | 0.970x | 0.970x | 1.175x |
| chess960_materialize | 1.051x | 1.024x | 1.557x |
| chess960_perft_d3 | 0.977x | 0.908x | 1.470x |
| polyglot_key | 0.970x | 0.961x | 38.823x |
| polyglot_recompute | 0.874x | 0.842x | 0.867x |
| repetition_key | 0.972x | 0.967x | 19.692x |
| corpus_count | 1.055x | 1.038x | 1.282x |
| corpus_materialize | 1.001x | 0.997x | 1.138x |
| corpus_uci_sorted | 0.953x | 0.936x | 1.407x |
| replay_trusted_native | 0.962x | 0.953x | 0.766x |
| replay_checked_native | 1.024x | 1.013x | 1.281x |

Cold first legal move list: 0.005 ms -> 0.003 ms (1.77x).
