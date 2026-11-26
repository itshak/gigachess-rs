| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.977x | 0.923x | 1.161x |
| materialize | 0.975x | 0.953x | 1.614x |
| perft_bulk_d4 | 0.856x | 0.782x | 1.000x |
| perft_full_d3 | 0.867x | 0.855x | 1.318x |
| fen_parse | 0.884x | 0.813x | 1.068x |
| fen_write_native_ep | 1.013x | 0.994x | 1.822x |
| snapshot | 1.009x | 0.992x | 1.122x |
| san_parse_all | 0.927x | 0.870x | 1.762x |
| san_render_native | 0.861x | 0.827x | 1.891x |
| apply_children_full | 0.871x | 0.836x | 0.988x |
| make_unmake_full | 0.683x | 0.610x | 0.810x |
| startpos | 1.175x | 1.175x | 1.578x |
| chess960_materialize | 0.992x | 0.946x | 1.316x |
| chess960_perft_d3 | 0.811x | 0.786x | 1.080x |
| polyglot_key | 1.018x | 1.003x | 41.086x |
| polyglot_recompute | 0.922x | 0.897x | 0.984x |
| repetition_key | 0.980x | 0.974x | 20.727x |
| corpus_count | 0.989x | 0.974x | 1.223x |
| corpus_materialize | 1.019x | 1.018x | 1.155x |
| corpus_uci_sorted | 1.002x | 0.999x | 1.554x |
| replay_trusted_native | 0.981x | 0.876x | 0.702x |
| replay_checked_native | 0.743x | 0.637x | 0.832x |

Cold first legal move list: 0.011 ms -> 0.009 ms (1.13x).
