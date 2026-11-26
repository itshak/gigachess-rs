| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.971x | 0.866x | 1.305x |
| materialize | 0.974x | 0.915x | 1.180x |
| perft_bulk_d4 | 0.982x | 0.934x | 1.340x |
| perft_full_d3 | 0.932x | 0.869x | 1.177x |
| fen_parse | 1.076x | 1.039x | 1.346x |
| fen_write_native_ep | 0.927x | 0.881x | 1.467x |
| snapshot | 0.736x | 0.673x | 0.976x |
| san_parse_all | 0.970x | 0.930x | 1.837x |
| san_render_native | 0.959x | 0.938x | 3.622x |
| apply_children_full | 1.030x | 0.967x | 1.462x |
| make_unmake_full | 0.947x | 0.873x | 1.253x |
| startpos | 0.697x | 0.697x | 0.897x |
| chess960_materialize | 0.987x | 0.968x | 1.316x |
| chess960_perft_d3 | 0.926x | 0.923x | 1.417x |
| polyglot_key | 0.967x | 0.956x | 24.458x |
| polyglot_recompute | 0.982x | 0.980x | 1.010x |
| repetition_key | 1.121x | 1.011x | 23.840x |
| corpus_count | 0.967x | 0.957x | 1.341x |
| corpus_materialize | 1.001x | 0.999x | 1.152x |
| corpus_uci_sorted | 0.996x | 0.949x | 1.483x |
| replay_trusted_native | 0.913x | 0.884x | 0.785x |
| replay_checked_native | 0.831x | 0.814x | 0.895x |

Cold first legal move list: 0.019 ms -> 0.022 ms (0.87x).
