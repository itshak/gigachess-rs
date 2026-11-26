| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.990x | 1.475x |
| materialize | 1.065x | 1.025x | 1.579x |
| perft_bulk_d4 | 1.009x | 0.997x | 1.268x |
| perft_full_d3 | 0.952x | 0.928x | 1.435x |
| fen_parse | 1.084x | 0.821x | 1.136x |
| fen_write_native_ep | 1.006x | 0.985x | 1.894x |
| snapshot | 2.004x | 1.988x | 1.001x |
| san_parse_all | 1.027x | 1.006x | 1.901x |
| san_render_native | 0.984x | 0.978x | 2.203x |
| apply_children_full | 1.014x | 0.996x | 1.067x |
| make_unmake_full | 0.992x | 0.977x | 1.126x |
| startpos | 1.206x | 1.206x | 0.997x |
| chess960_materialize | 0.993x | 0.991x | 1.459x |
| chess960_perft_d3 | 1.006x | 1.004x | 1.394x |
| polyglot_key | 1.078x | 1.006x | 39.983x |
| polyglot_recompute | 1.002x | 0.996x | 1.014x |
| repetition_key | 1.069x | 1.049x | 19.944x |
| corpus_count | 1.006x | 1.001x | 1.408x |
| corpus_materialize | 0.998x | 0.996x | 1.136x |
| corpus_uci_sorted | 0.989x | 0.989x | 1.575x |
| replay_trusted_native | 1.006x | 0.999x | 0.726x |
| replay_checked_native | 1.000x | 0.997x | 1.055x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.95x).
