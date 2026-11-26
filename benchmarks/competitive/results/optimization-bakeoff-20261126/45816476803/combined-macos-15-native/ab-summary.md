| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.972x | 0.917x | 1.308x |
| materialize | 0.982x | 0.936x | 1.019x |
| perft_bulk_d4 | 1.026x | 1.005x | 1.137x |
| perft_full_d3 | 0.986x | 0.961x | 1.348x |
| fen_parse | 0.925x | 0.902x | 1.154x |
| fen_write_native_ep | 0.980x | 0.927x | 1.756x |
| snapshot | 0.993x | 0.946x | 1.007x |
| san_parse_all | 1.004x | 0.977x | 1.704x |
| san_render_native | 1.012x | 0.986x | 2.617x |
| apply_children_full | 1.060x | 1.024x | 1.108x |
| make_unmake_full | 0.977x | 0.899x | 1.173x |
| startpos | 0.946x | 0.946x | 1.294x |
| chess960_materialize | 0.984x | 0.982x | 1.396x |
| chess960_perft_d3 | 0.971x | 0.965x | 1.254x |
| polyglot_key | 0.973x | 0.948x | 38.874x |
| polyglot_recompute | 1.002x | 0.999x | 0.972x |
| repetition_key | 0.976x | 0.963x | 18.848x |
| corpus_count | 1.000x | 0.979x | 1.033x |
| corpus_materialize | 0.954x | 0.943x | 1.072x |
| corpus_uci_sorted | 0.978x | 0.933x | 1.405x |
| replay_trusted_native | 0.994x | 0.966x | 0.594x |
| replay_checked_native | 0.986x | 0.936x | 1.059x |

Cold first legal move list: 0.004 ms -> 0.004 ms (1.21x).
