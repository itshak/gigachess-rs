| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.996x | 1.294x |
| materialize | 0.997x | 0.991x | 1.202x |
| perft_bulk_d4 | 0.999x | 0.997x | 1.286x |
| perft_full_d3 | 1.326x | 1.286x | 0.990x |
| fen_parse | 1.002x | 0.999x | 1.245x |
| fen_write_native_ep | 1.001x | 0.998x | 1.054x |
| snapshot | 0.973x | 0.971x | 0.946x |
| san_parse_all | 1.001x | 0.995x | 1.775x |
| san_render_native | 0.998x | 0.989x | 2.381x |
| apply_children_full | 1.002x | 0.983x | 0.832x |
| make_unmake_full | 0.996x | 0.992x | 1.131x |
| startpos | 0.968x | 0.968x | 1.148x |
| chess960_materialize | 1.001x | 1.000x | 1.234x |
| chess960_perft_d3 | 0.996x | 0.995x | 1.222x |
| polyglot_key | 1.013x | 1.004x | 26.249x |
| polyglot_recompute | 1.001x | 1.000x | 1.067x |
| repetition_key | 1.008x | 1.000x | 19.279x |
| corpus_count | 0.997x | 0.988x | 1.418x |
| corpus_materialize | 0.994x | 0.992x | 1.188x |
| corpus_uci_sorted | 0.999x | 0.999x | 0.974x |
| replay_trusted_native | 0.999x | 0.996x | 0.411x |
| replay_checked_native | 1.006x | 1.005x | 0.890x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.18x).
