| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.969x | 1.436x |
| materialize | 0.996x | 0.977x | 1.253x |
| perft_bulk_d4 | 1.008x | 0.991x | 1.382x |
| perft_full_d3 | 1.001x | 0.997x | 1.302x |
| fen_parse | 1.073x | 1.031x | 1.167x |
| fen_write_native_ep | 1.001x | 0.981x | 1.800x |
| snapshot | 0.777x | 0.767x | 0.971x |
| san_parse_all | 0.994x | 0.970x | 2.025x |
| san_render_native | 0.996x | 0.983x | 2.183x |
| apply_children_full | 1.018x | 0.993x | 1.341x |
| make_unmake_full | 1.011x | 0.976x | 1.361x |
| startpos | 1.000x | 1.000x | 1.251x |
| chess960_materialize | 0.996x | 0.993x | 1.404x |
| chess960_perft_d3 | 0.987x | 0.987x | 1.503x |
| polyglot_key | 1.084x | 0.998x | 29.865x |
| polyglot_recompute | 1.010x | 0.913x | 1.042x |
| repetition_key | 1.026x | 0.981x | 20.942x |
| corpus_count | 1.004x | 0.996x | 1.364x |
| corpus_materialize | 1.001x | 0.993x | 1.137x |
| corpus_uci_sorted | 1.011x | 1.004x | 1.734x |
| replay_trusted_native | 0.973x | 0.965x | 0.878x |
| replay_checked_native | 0.975x | 0.972x | 1.087x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.10x).
