| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.784x | 1.356x |
| materialize | 1.245x | 0.973x | 1.097x |
| perft_bulk_d4 | 0.832x | 0.634x | 1.144x |
| perft_full_d3 | 0.980x | 0.786x | 1.320x |
| fen_parse | 0.913x | 0.808x | 1.125x |
| fen_write_native_ep | 0.962x | 0.790x | 1.745x |
| snapshot | 0.846x | 0.613x | 0.808x |
| san_parse_all | 0.826x | 0.671x | 1.869x |
| san_render_native | 0.827x | 0.620x | 2.590x |
| apply_children_full | 0.858x | 0.729x | 1.010x |
| make_unmake_full | 0.853x | 0.742x | 1.086x |
| startpos | 1.097x | 1.097x | 1.186x |
| chess960_materialize | 0.930x | 0.885x | 1.527x |
| chess960_perft_d3 | 0.879x | 0.860x | 1.560x |
| polyglot_key | 0.801x | 0.774x | 32.877x |
| polyglot_recompute | 1.016x | 0.931x | 1.048x |
| repetition_key | 0.999x | 0.981x | 19.903x |
| corpus_count | 0.792x | 0.723x | 1.226x |
| corpus_materialize | 1.023x | 1.011x | 1.193x |
| corpus_uci_sorted | 0.898x | 0.848x | 1.323x |
| replay_trusted_native | 1.033x | 0.945x | 0.692x |
| replay_checked_native | 1.098x | 0.958x | 1.153x |

Cold first legal move list: 0.020 ms -> 0.010 ms (1.92x).
