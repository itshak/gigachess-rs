| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.818x | 1.186x |
| materialize | 1.147x | 0.889x | 1.130x |
| perft_bulk_d4 | 1.011x | 0.854x | 1.253x |
| perft_full_d3 | 1.059x | 0.951x | 0.771x |
| fen_parse | 0.893x | 0.707x | 1.098x |
| fen_write_native_ep | 0.970x | 0.799x | 1.236x |
| snapshot | 0.906x | 0.692x | 0.935x |
| san_parse_all | 1.043x | 0.922x | 1.618x |
| san_render_native | 0.995x | 0.932x | 2.437x |
| apply_children_full | 1.061x | 0.926x | 0.803x |
| make_unmake_full | 1.039x | 0.850x | 0.980x |
| startpos | 0.846x | 0.846x | 1.141x |
| chess960_materialize | 0.807x | 0.674x | 0.993x |
| chess960_perft_d3 | 0.989x | 0.912x | 1.423x |
| polyglot_key | 1.132x | 0.997x | 38.699x |
| polyglot_recompute | 1.061x | 1.043x | 1.017x |
| repetition_key | 1.188x | 1.068x | 19.711x |
| corpus_count | 1.114x | 1.074x | 1.285x |
| corpus_materialize | 1.088x | 0.953x | 1.164x |
| corpus_uci_sorted | 1.225x | 1.133x | 1.043x |
| replay_trusted_native | 0.993x | 0.915x | 0.415x |
| replay_checked_native | 1.477x | 1.474x | 0.881x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.03x).
