| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.047x | 0.956x | 1.316x |
| materialize | 1.062x | 0.968x | 1.009x |
| perft_bulk_d4 | 0.943x | 0.742x | 1.195x |
| perft_full_d3 | 0.930x | 0.858x | 0.693x |
| fen_parse | 0.897x | 0.792x | 1.121x |
| fen_write_native_ep | 0.954x | 0.841x | 1.283x |
| snapshot | 0.780x | 0.736x | 0.778x |
| san_parse_all | 0.972x | 0.911x | 1.580x |
| san_render_native | 0.963x | 0.870x | 2.570x |
| apply_children_full | 0.914x | 0.853x | 0.671x |
| make_unmake_full | 0.833x | 0.783x | 0.854x |
| startpos | 0.829x | 0.829x | 1.002x |
| chess960_materialize | 0.883x | 0.854x | 1.225x |
| chess960_perft_d3 | 0.977x | 0.918x | 1.332x |
| polyglot_key | 0.927x | 0.898x | 34.075x |
| polyglot_recompute | 1.224x | 1.044x | 0.990x |
| repetition_key | 1.033x | 0.893x | 17.072x |
| corpus_count | 1.051x | 0.958x | 1.212x |
| corpus_materialize | 1.093x | 1.048x | 1.310x |
| corpus_uci_sorted | 0.974x | 0.907x | 0.912x |
| replay_trusted_native | 0.869x | 0.852x | 0.395x |
| replay_checked_native | 0.816x | 0.802x | 0.766x |

Cold first legal move list: 0.016 ms -> 0.016 ms (0.98x).
