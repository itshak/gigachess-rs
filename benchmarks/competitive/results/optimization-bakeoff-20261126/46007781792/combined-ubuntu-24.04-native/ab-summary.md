| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.059x | 1.013x | 1.580x |
| materialize | 1.013x | 0.992x | 1.597x |
| perft_bulk_d4 | 1.059x | 1.049x | 1.241x |
| perft_full_d3 | 1.237x | 1.052x | 1.661x |
| fen_parse | 0.990x | 0.964x | 1.147x |
| fen_write_native_ep | 0.996x | 0.988x | 1.909x |
| snapshot | 0.996x | 0.978x | 0.991x |
| san_parse_all | 0.897x | 0.868x | 1.682x |
| san_render_native | 0.984x | 0.976x | 2.083x |
| apply_children_full | 1.006x | 0.995x | 1.363x |
| make_unmake_full | 0.979x | 0.958x | 1.176x |
| startpos | 0.891x | 0.891x | 0.994x |
| chess960_materialize | 0.972x | 0.967x | 1.529x |
| chess960_perft_d3 | 1.015x | 1.000x | 1.531x |
| polyglot_key | 0.981x | 0.962x | 39.482x |
| polyglot_recompute | 0.992x | 0.987x | 1.020x |
| repetition_key | 1.016x | 0.994x | 19.608x |
| corpus_count | 1.236x | 1.130x | 1.345x |
| corpus_materialize | 1.003x | 1.000x | 1.070x |
| corpus_uci_sorted | 1.008x | 1.007x | 1.562x |
| replay_trusted_native | 1.052x | 1.013x | 0.927x |
| replay_checked_native | 1.017x | 1.013x | 1.235x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.75x).
