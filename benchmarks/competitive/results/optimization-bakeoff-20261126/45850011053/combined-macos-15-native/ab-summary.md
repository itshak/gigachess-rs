| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.837x | 0.655x | 1.610x |
| materialize | 0.831x | 0.670x | 1.102x |
| perft_bulk_d4 | 0.846x | 0.733x | 1.220x |
| perft_full_d3 | 0.779x | 0.656x | 1.185x |
| fen_parse | 0.863x | 0.723x | 1.130x |
| fen_write_native_ep | 0.869x | 0.776x | 1.823x |
| snapshot | 0.812x | 0.583x | 0.778x |
| san_parse_all | 0.930x | 0.791x | 1.713x |
| san_render_native | 0.915x | 0.777x | 2.878x |
| apply_children_full | 0.930x | 0.661x | 1.068x |
| make_unmake_full | 0.800x | 0.649x | 1.167x |
| startpos | 0.984x | 0.984x | 1.199x |
| chess960_materialize | 0.961x | 0.938x | 1.536x |
| chess960_perft_d3 | 0.962x | 0.947x | 1.465x |
| polyglot_key | 0.970x | 0.901x | 36.006x |
| polyglot_recompute | 1.024x | 1.004x | 0.995x |
| repetition_key | 1.049x | 1.006x | 19.683x |
| corpus_count | 0.971x | 0.928x | 1.092x |
| corpus_materialize | 1.123x | 1.070x | 1.310x |
| corpus_uci_sorted | 1.014x | 0.943x | 1.500x |
| replay_trusted_native | 0.915x | 0.858x | 0.565x |
| replay_checked_native | 0.996x | 0.928x | 1.141x |

Cold first legal move list: 0.009 ms -> 0.005 ms (1.86x).
