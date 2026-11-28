| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.054x | 1.000x | 1.354x |
| materialize | 0.982x | 0.945x | 1.727x |
| perft_bulk_d4 | 1.067x | 1.043x | 1.259x |
| perft_full_d3 | 1.168x | 1.039x | 1.541x |
| fen_parse | 1.021x | 0.990x | 1.183x |
| fen_write_native_ep | 1.003x | 0.976x | 2.078x |
| snapshot | 0.999x | 0.984x | 0.998x |
| san_parse_all | 1.000x | 0.990x | 1.901x |
| san_render_native | 1.014x | 0.991x | 2.129x |
| apply_children_full | 1.021x | 1.003x | 1.401x |
| make_unmake_full | 0.996x | 0.968x | 1.285x |
| startpos | 0.999x | 0.999x | 1.358x |
| chess960_materialize | 0.925x | 0.913x | 1.460x |
| chess960_perft_d3 | 1.012x | 0.993x | 1.516x |
| polyglot_key | 0.970x | 0.950x | 42.681x |
| polyglot_recompute | 1.059x | 1.050x | 1.082x |
| repetition_key | 0.997x | 0.982x | 23.992x |
| corpus_count | 1.152x | 1.113x | 1.360x |
| corpus_materialize | 1.005x | 0.997x | 1.147x |
| corpus_uci_sorted | 0.993x | 0.990x | 1.548x |
| replay_trusted_native | 0.979x | 0.950x | 0.878x |
| replay_checked_native | 0.990x | 0.984x | 1.168x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.02x).
