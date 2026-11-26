| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.996x | 1.517x |
| materialize | 1.002x | 0.994x | 1.630x |
| perft_bulk_d4 | 0.988x | 0.966x | 1.158x |
| perft_full_d3 | 1.006x | 0.990x | 1.307x |
| fen_parse | 1.013x | 0.976x | 1.075x |
| fen_write_native_ep | 1.002x | 0.995x | 2.149x |
| snapshot | 1.000x | 0.993x | 0.999x |
| san_parse_all | 0.925x | 0.905x | 1.733x |
| san_render_native | 0.992x | 0.972x | 2.130x |
| apply_children_full | 1.000x | 0.991x | 1.286x |
| make_unmake_full | 0.992x | 0.969x | 1.153x |
| startpos | 1.011x | 1.011x | 1.258x |
| chess960_materialize | 1.007x | 1.006x | 1.554x |
| chess960_perft_d3 | 1.020x | 1.018x | 1.516x |
| polyglot_key | 1.017x | 1.000x | 39.526x |
| polyglot_recompute | 1.053x | 1.044x | 1.014x |
| repetition_key | 1.011x | 1.003x | 23.886x |
| corpus_count | 1.035x | 1.016x | 1.113x |
| corpus_materialize | 1.000x | 0.998x | 1.072x |
| corpus_uci_sorted | 1.000x | 0.994x | 1.563x |
| replay_trusted_native | 1.009x | 0.998x | 0.888x |
| replay_checked_native | 1.012x | 1.009x | 1.037x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.03x).
