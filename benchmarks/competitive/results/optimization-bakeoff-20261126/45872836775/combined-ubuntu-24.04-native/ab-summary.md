| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.995x | 1.500x |
| materialize | 1.047x | 1.024x | 1.165x |
| perft_bulk_d4 | 0.991x | 0.967x | 1.383x |
| perft_full_d3 | 1.002x | 0.996x | 1.203x |
| fen_parse | 0.975x | 0.915x | 1.121x |
| fen_write_native_ep | 0.997x | 0.994x | 1.860x |
| snapshot | 1.000x | 0.994x | 1.247x |
| san_parse_all | 1.002x | 0.995x | 2.170x |
| san_render_native | 0.994x | 0.957x | 2.088x |
| apply_children_full | 0.988x | 0.963x | 1.288x |
| make_unmake_full | 0.974x | 0.943x | 1.169x |
| startpos | 0.997x | 0.997x | 1.199x |
| chess960_materialize | 1.013x | 1.003x | 1.574x |
| chess960_perft_d3 | 1.004x | 0.996x | 1.589x |
| polyglot_key | 0.999x | 0.999x | 20.860x |
| polyglot_recompute | 0.818x | 0.817x | 0.839x |
| repetition_key | 0.995x | 0.993x | 18.869x |
| corpus_count | 0.994x | 0.993x | 1.289x |
| corpus_materialize | 0.994x | 0.987x | 1.224x |
| corpus_uci_sorted | 0.988x | 0.980x | 1.670x |
| replay_trusted_native | 1.175x | 1.009x | 1.189x |
| replay_checked_native | 1.019x | 1.017x | 1.207x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.75x).
