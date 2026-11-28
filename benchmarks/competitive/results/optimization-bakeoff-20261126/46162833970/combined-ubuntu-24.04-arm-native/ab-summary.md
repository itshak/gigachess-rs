| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.973x | 1.561x |
| materialize | 1.011x | 1.000x | 1.344x |
| perft_bulk_d4 | 1.037x | 1.013x | 1.452x |
| perft_full_d3 | 1.116x | 0.994x | 1.489x |
| fen_parse | 1.018x | 0.944x | 1.239x |
| fen_write_native_ep | 0.999x | 0.996x | 1.581x |
| snapshot | 0.999x | 0.998x | 0.949x |
| san_parse_all | 0.997x | 0.991x | 1.827x |
| san_render_native | 1.002x | 0.998x | 2.681x |
| apply_children_full | 1.004x | 0.993x | 1.159x |
| make_unmake_full | 1.017x | 1.011x | 1.592x |
| startpos | 0.999x | 0.999x | 1.328x |
| chess960_materialize | 1.011x | 1.009x | 1.512x |
| chess960_perft_d3 | 1.015x | 1.003x | 1.516x |
| polyglot_key | 1.013x | 1.001x | 25.791x |
| polyglot_recompute | 1.369x | 1.279x | 1.482x |
| repetition_key | 1.025x | 1.000x | 19.634x |
| corpus_count | 1.098x | 1.075x | 1.497x |
| corpus_materialize | 1.008x | 1.004x | 1.232x |
| corpus_uci_sorted | 0.994x | 0.991x | 1.443x |
| replay_trusted_native | 1.016x | 1.011x | 0.637x |
| replay_checked_native | 1.021x | 1.014x | 1.147x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.88x).
