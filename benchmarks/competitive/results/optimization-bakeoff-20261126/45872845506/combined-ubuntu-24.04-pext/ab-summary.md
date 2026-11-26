| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.986x | 1.494x |
| materialize | 0.995x | 0.945x | 1.263x |
| perft_bulk_d4 | 0.998x | 0.979x | 1.419x |
| perft_full_d3 | 1.000x | 0.998x | 1.311x |
| fen_parse | 0.954x | 0.939x | 1.191x |
| fen_write_native_ep | 0.939x | 0.876x | 1.640x |
| snapshot | 0.999x | 0.994x | 1.250x |
| san_parse_all | 1.013x | 1.005x | 2.105x |
| san_render_native | 1.003x | 0.988x | 2.215x |
| apply_children_full | 1.033x | 0.962x | 1.361x |
| make_unmake_full | 1.018x | 0.970x | 1.308x |
| startpos | 0.999x | 0.999x | 1.250x |
| chess960_materialize | 0.987x | 0.954x | 1.378x |
| chess960_perft_d3 | 1.001x | 0.988x | 1.494x |
| polyglot_key | 1.070x | 1.042x | 27.276x |
| polyglot_recompute | 0.999x | 0.998x | 1.037x |
| repetition_key | 0.994x | 0.992x | 19.812x |
| corpus_count | 0.998x | 0.993x | 1.423x |
| corpus_materialize | 0.993x | 0.987x | 1.191x |
| corpus_uci_sorted | 0.997x | 0.992x | 1.916x |
| replay_trusted_native | 0.896x | 0.858x | 0.799x |
| replay_checked_native | 0.914x | 0.907x | 1.019x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.98x).
