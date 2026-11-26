| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.962x | 0.793x | 1.453x |
| materialize | 0.995x | 0.859x | 1.229x |
| perft_bulk_d4 | 0.908x | 0.833x | 1.224x |
| perft_full_d3 | 0.866x | 0.806x | 0.905x |
| fen_parse | 1.005x | 0.990x | 1.144x |
| fen_write_native_ep | 1.001x | 0.991x | 1.537x |
| snapshot | 0.960x | 0.841x | 0.959x |
| san_parse_all | 1.002x | 0.996x | 2.182x |
| san_render_native | 0.984x | 0.947x | 3.640x |
| apply_children_full | 1.008x | 0.995x | 1.395x |
| make_unmake_full | 0.988x | 0.979x | 1.372x |
| startpos | 1.000x | 1.000x | 1.245x |
| chess960_materialize | 1.089x | 1.008x | 1.369x |
| chess960_perft_d3 | 0.992x | 0.985x | 1.507x |
| polyglot_key | 0.934x | 0.869x | 27.622x |
| polyglot_recompute | 1.016x | 1.014x | 1.016x |
| repetition_key | 0.943x | 0.886x | 21.079x |
| corpus_count | 1.010x | 0.990x | 1.375x |
| corpus_materialize | 1.003x | 0.998x | 1.150x |
| corpus_uci_sorted | 0.979x | 0.978x | 1.386x |
| replay_trusted_native | 0.987x | 0.981x | 0.828x |
| replay_checked_native | 0.966x | 0.961x | 0.987x |

Cold first legal move list: 0.018 ms -> 0.019 ms (0.95x).
