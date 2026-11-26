| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.050x | 1.014x | 1.373x |
| materialize | 0.977x | 0.903x | 1.409x |
| perft_bulk_d4 | 1.020x | 0.969x | 1.004x |
| perft_full_d3 | 1.000x | 0.964x | 1.497x |
| fen_parse | 1.082x | 1.051x | 1.165x |
| fen_write_native_ep | 1.017x | 0.971x | 2.034x |
| snapshot | 1.001x | 0.983x | 1.332x |
| san_parse_all | 0.982x | 0.961x | 1.817x |
| san_render_native | 0.970x | 0.957x | 2.080x |
| apply_children_full | 1.004x | 0.993x | 1.285x |
| make_unmake_full | 1.004x | 0.949x | 1.143x |
| startpos | 0.904x | 0.904x | 0.999x |
| chess960_materialize | 1.006x | 0.998x | 1.259x |
| chess960_perft_d3 | 0.971x | 0.962x | 1.333x |
| polyglot_key | 0.997x | 0.990x | 40.054x |
| polyglot_recompute | 1.033x | 1.008x | 1.026x |
| repetition_key | 0.936x | 0.868x | 19.846x |
| corpus_count | 1.002x | 0.992x | 0.812x |
| corpus_materialize | 1.007x | 1.004x | 1.012x |
| corpus_uci_sorted | 1.003x | 0.996x | 1.550x |
| replay_trusted_native | 0.973x | 0.964x | 0.833x |
| replay_checked_native | 1.019x | 0.965x | 1.094x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.93x).
