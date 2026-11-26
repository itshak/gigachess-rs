| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.998x | 1.413x |
| materialize | 1.008x | 0.986x | 1.260x |
| perft_bulk_d4 | 0.976x | 0.959x | 1.321x |
| perft_full_d3 | 0.762x | 0.709x | 0.917x |
| fen_parse | 0.846x | 0.823x | 1.049x |
| fen_write_native_ep | 0.998x | 0.991x | 0.916x |
| snapshot | 1.334x | 1.324x | 1.036x |
| san_parse_all | 1.019x | 1.005x | 2.098x |
| san_render_native | 0.953x | 0.931x | 2.202x |
| apply_children_full | 1.026x | 1.019x | 1.131x |
| make_unmake_full | 1.021x | 1.005x | 1.238x |
| startpos | 0.926x | 0.926x | 1.195x |
| chess960_materialize | 1.004x | 0.995x | 1.303x |
| chess960_perft_d3 | 1.005x | 1.000x | 1.383x |
| polyglot_key | 0.869x | 0.792x | 24.245x |
| polyglot_recompute | 0.997x | 0.997x | 1.053x |
| repetition_key | 0.906x | 0.857x | 19.507x |
| corpus_count | 0.980x | 0.973x | 1.474x |
| corpus_materialize | 1.010x | 1.008x | 1.237x |
| corpus_uci_sorted | 1.001x | 0.988x | 1.810x |
| replay_trusted_native | 1.004x | 1.002x | 0.708x |
| replay_checked_native | 0.980x | 0.976x | 0.998x |

Cold first legal move list: 0.013 ms -> 0.015 ms (0.87x).
