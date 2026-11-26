| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.560x |
| materialize | 1.060x | 1.026x | 1.599x |
| perft_bulk_d4 | 1.000x | 0.976x | 1.187x |
| perft_full_d3 | 1.212x | 1.067x | 1.655x |
| fen_parse | 1.007x | 0.988x | 1.062x |
| fen_write_native_ep | 0.996x | 0.961x | 2.117x |
| snapshot | 1.002x | 0.999x | 1.154x |
| san_parse_all | 1.012x | 0.984x | 1.861x |
| san_render_native | 0.986x | 0.977x | 2.096x |
| apply_children_full | 0.995x | 0.976x | 1.331x |
| make_unmake_full | 0.988x | 0.970x | 1.175x |
| startpos | 0.996x | 0.996x | 1.113x |
| chess960_materialize | 1.005x | 0.997x | 1.531x |
| chess960_perft_d3 | 0.994x | 0.983x | 1.471x |
| polyglot_key | 1.045x | 0.997x | 39.644x |
| polyglot_recompute | 1.062x | 1.023x | 1.067x |
| repetition_key | 0.855x | 0.701x | 19.575x |
| corpus_count | 0.997x | 0.996x | 1.119x |
| corpus_materialize | 1.008x | 1.001x | 1.087x |
| corpus_uci_sorted | 1.015x | 1.014x | 1.552x |
| replay_trusted_native | 0.945x | 0.891x | 0.860x |
| replay_checked_native | 0.962x | 0.954x | 1.106x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.96x).
