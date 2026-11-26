| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.961x | 0.905x | 1.184x |
| materialize | 0.973x | 0.951x | 1.298x |
| perft_bulk_d4 | 0.912x | 0.891x | 1.097x |
| perft_full_d3 | 0.866x | 0.752x | 0.929x |
| fen_parse | 0.912x | 0.897x | 1.074x |
| fen_write_native_ep | 0.993x | 0.984x | 1.448x |
| snapshot | 1.005x | 0.986x | 0.998x |
| san_parse_all | 0.934x | 0.905x | 1.746x |
| san_render_native | 0.825x | 0.801x | 1.879x |
| apply_children_full | 0.972x | 0.883x | 1.162x |
| make_unmake_full | 0.920x | 0.870x | 1.170x |
| startpos | 1.003x | 1.003x | 1.280x |
| chess960_materialize | 0.881x | 0.832x | 1.397x |
| chess960_perft_d3 | 0.911x | 0.869x | 1.334x |
| polyglot_key | 0.915x | 0.849x | 29.553x |
| polyglot_recompute | 0.945x | 0.933x | 0.953x |
| repetition_key | 0.690x | 0.659x | 14.128x |
| corpus_count | 0.967x | 0.956x | 1.054x |
| corpus_materialize | 0.959x | 0.926x | 1.167x |
| corpus_uci_sorted | 1.012x | 1.012x | 1.620x |
| replay_trusted_native | 0.952x | 0.905x | 0.650x |
| replay_checked_native | 0.825x | 0.759x | 0.986x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.72x).
