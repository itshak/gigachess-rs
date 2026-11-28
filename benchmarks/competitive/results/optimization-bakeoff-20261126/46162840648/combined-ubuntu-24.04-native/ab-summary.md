| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.051x | 1.004x | 1.585x |
| materialize | 1.005x | 0.993x | 1.587x |
| perft_bulk_d4 | 1.050x | 1.039x | 1.247x |
| perft_full_d3 | 1.165x | 1.033x | 1.575x |
| fen_parse | 0.990x | 0.966x | 1.118x |
| fen_write_native_ep | 0.999x | 0.986x | 1.892x |
| snapshot | 0.997x | 0.988x | 1.147x |
| san_parse_all | 1.007x | 0.993x | 1.885x |
| san_render_native | 0.995x | 0.978x | 2.112x |
| apply_children_full | 1.005x | 0.991x | 1.344x |
| make_unmake_full | 0.951x | 0.926x | 1.135x |
| startpos | 0.999x | 0.999x | 1.120x |
| chess960_materialize | 0.988x | 0.976x | 1.526x |
| chess960_perft_d3 | 1.009x | 0.993x | 1.521x |
| polyglot_key | 1.009x | 0.999x | 39.548x |
| polyglot_recompute | 1.626x | 1.308x | 1.620x |
| repetition_key | 1.025x | 0.988x | 19.306x |
| corpus_count | 1.207x | 1.122x | 1.310x |
| corpus_materialize | 1.001x | 0.998x | 1.047x |
| corpus_uci_sorted | 0.996x | 0.993x | 1.545x |
| replay_trusted_native | 0.890x | 0.866x | 0.764x |
| replay_checked_native | 0.925x | 0.895x | 1.061x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.95x).
