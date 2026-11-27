| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.983x | 1.573x |
| materialize | 1.014x | 1.000x | 1.335x |
| perft_bulk_d4 | 1.045x | 1.020x | 1.465x |
| perft_full_d3 | 1.132x | 1.002x | 1.507x |
| fen_parse | 1.006x | 0.998x | 1.228x |
| fen_write_native_ep | 0.997x | 0.994x | 1.575x |
| snapshot | 0.992x | 0.988x | 0.945x |
| san_parse_all | 0.998x | 0.994x | 1.826x |
| san_render_native | 0.999x | 0.991x | 2.668x |
| apply_children_full | 0.994x | 0.989x | 1.156x |
| make_unmake_full | 0.994x | 0.984x | 1.546x |
| startpos | 1.001x | 1.001x | 1.305x |
| chess960_materialize | 1.009x | 1.008x | 1.508x |
| chess960_perft_d3 | 1.014x | 1.000x | 1.523x |
| polyglot_key | 1.011x | 0.994x | 25.767x |
| polyglot_recompute | 0.998x | 0.997x | 1.089x |
| repetition_key | 0.997x | 0.995x | 19.348x |
| corpus_count | 1.088x | 1.054x | 1.490x |
| corpus_materialize | 1.000x | 0.998x | 1.239x |
| corpus_uci_sorted | 0.993x | 0.991x | 1.453x |
| replay_trusted_native | 0.996x | 0.994x | 0.643x |
| replay_checked_native | 1.014x | 1.008x | 1.141x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.01x).
