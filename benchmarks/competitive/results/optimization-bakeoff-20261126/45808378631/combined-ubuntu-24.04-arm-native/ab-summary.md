| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.940x | 1.384x |
| materialize | 1.007x | 0.966x | 1.245x |
| perft_bulk_d4 | 0.918x | 0.898x | 1.244x |
| perft_full_d3 | 1.014x | 0.986x | 1.073x |
| fen_parse | 0.991x | 0.986x | 1.235x |
| fen_write_native_ep | 1.001x | 0.999x | 1.616x |
| snapshot | 0.916x | 0.914x | 0.944x |
| san_parse_all | 0.994x | 0.989x | 1.825x |
| san_render_native | 0.987x | 0.977x | 2.560x |
| apply_children_full | 1.061x | 1.041x | 1.187x |
| make_unmake_full | 0.995x | 0.985x | 1.427x |
| startpos | 1.072x | 1.072x | 1.293x |
| chess960_materialize | 1.039x | 0.981x | 1.382x |
| chess960_perft_d3 | 1.018x | 0.966x | 1.335x |
| polyglot_key | 0.992x | 0.981x | 25.254x |
| polyglot_recompute | 0.999x | 0.997x | 1.071x |
| repetition_key | 0.958x | 0.933x | 18.398x |
| corpus_count | 0.827x | 0.797x | 1.222x |
| corpus_materialize | 0.983x | 0.977x | 1.167x |
| corpus_uci_sorted | 0.996x | 0.996x | 1.453x |
| replay_trusted_native | 1.004x | 1.004x | 0.527x |
| replay_checked_native | 0.986x | 0.976x | 0.964x |

Cold first legal move list: 0.011 ms -> 0.006 ms (1.93x).
