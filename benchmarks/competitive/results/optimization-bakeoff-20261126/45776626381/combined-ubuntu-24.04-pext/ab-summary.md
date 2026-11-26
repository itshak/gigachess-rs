| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.938x | 1.347x |
| materialize | 1.027x | 1.010x | 1.331x |
| perft_bulk_d4 | 1.032x | 1.006x | 1.344x |
| perft_full_d3 | 1.534x | 1.428x | 1.226x |
| fen_parse | 1.010x | 0.980x | 1.150x |
| fen_write_native_ep | 0.994x | 0.962x | 0.955x |
| snapshot | 1.148x | 1.142x | 0.857x |
| san_parse_all | 1.047x | 1.027x | 2.047x |
| san_render_native | 1.066x | 1.023x | 2.313x |
| apply_children_full | 1.174x | 1.129x | 1.104x |
| make_unmake_full | 1.241x | 1.224x | 1.241x |
| startpos | 1.165x | 1.165x | 1.332x |
| chess960_materialize | 1.004x | 0.974x | 1.299x |
| chess960_perft_d3 | 1.052x | 1.024x | 1.361x |
| polyglot_key | 0.930x | 0.866x | 25.606x |
| polyglot_recompute | 1.001x | 0.997x | 1.035x |
| repetition_key | 0.946x | 0.879x | 19.797x |
| corpus_count | 1.040x | 1.024x | 1.507x |
| corpus_materialize | 1.010x | 1.003x | 1.216x |
| corpus_uci_sorted | 1.747x | 1.693x | 1.702x |
| replay_trusted_native | 1.197x | 1.167x | 0.710x |
| replay_checked_native | 1.053x | 1.048x | 0.980x |

Cold first legal move list: 0.014 ms -> 0.013 ms (1.09x).
