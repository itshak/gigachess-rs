| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.918x | 0.633x | 1.197x |
| materialize | 1.070x | 0.907x | 1.043x |
| perft_bulk_d4 | 0.995x | 0.802x | 0.992x |
| perft_full_d3 | 0.969x | 0.787x | 1.211x |
| fen_parse | 0.947x | 0.809x | 1.237x |
| fen_write_native_ep | 1.000x | 0.716x | 1.645x |
| snapshot | 1.114x | 0.914x | 1.008x |
| san_parse_all | 0.998x | 0.642x | 1.746x |
| san_render_native | 0.991x | 0.629x | 2.687x |
| apply_children_full | 0.872x | 0.713x | 1.025x |
| make_unmake_full | 0.947x | 0.692x | 1.176x |
| startpos | 1.452x | 1.452x | 1.712x |
| chess960_materialize | 0.958x | 0.925x | 1.409x |
| chess960_perft_d3 | 1.128x | 1.125x | 1.308x |
| polyglot_key | 1.049x | 0.969x | 41.017x |
| polyglot_recompute | 0.994x | 0.982x | 0.926x |
| repetition_key | 1.257x | 1.197x | 19.614x |
| corpus_count | 1.146x | 1.007x | 1.007x |
| corpus_materialize | 1.025x | 0.916x | 1.074x |
| corpus_uci_sorted | 1.094x | 1.068x | 1.460x |
| replay_trusted_native | 0.922x | 0.750x | 0.660x |
| replay_checked_native | 1.021x | 0.934x | 1.364x |

Cold first legal move list: 0.006 ms -> 0.004 ms (1.34x).
