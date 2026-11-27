| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.980x | 1.565x |
| materialize | 1.012x | 0.996x | 1.349x |
| perft_bulk_d4 | 1.039x | 1.014x | 1.447x |
| perft_full_d3 | 1.135x | 1.017x | 1.521x |
| fen_parse | 0.980x | 0.926x | 1.192x |
| fen_write_native_ep | 0.990x | 0.988x | 1.563x |
| snapshot | 1.018x | 1.015x | 0.936x |
| san_parse_all | 1.000x | 0.997x | 1.820x |
| san_render_native | 1.002x | 0.997x | 2.701x |
| apply_children_full | 1.006x | 0.982x | 1.162x |
| make_unmake_full | 1.014x | 1.000x | 1.573x |
| startpos | 1.001x | 1.001x | 1.273x |
| chess960_materialize | 1.007x | 1.005x | 1.509x |
| chess960_perft_d3 | 1.020x | 1.015x | 1.522x |
| polyglot_key | 1.004x | 1.001x | 25.834x |
| polyglot_recompute | 1.039x | 0.955x | 1.124x |
| repetition_key | 1.000x | 1.000x | 19.285x |
| corpus_count | 1.082x | 1.050x | 1.507x |
| corpus_materialize | 1.005x | 1.005x | 1.243x |
| corpus_uci_sorted | 0.980x | 0.980x | 1.439x |
| replay_trusted_native | 0.993x | 0.992x | 0.623x |
| replay_checked_native | 1.009x | 1.007x | 1.145x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.87x).
