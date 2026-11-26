| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.824x | 1.527x |
| materialize | 1.044x | 0.920x | 1.056x |
| perft_bulk_d4 | 0.993x | 0.854x | 1.231x |
| perft_full_d3 | 1.135x | 1.048x | 1.232x |
| fen_parse | 1.098x | 0.999x | 1.282x |
| fen_write_native_ep | 1.047x | 0.963x | 1.713x |
| snapshot | 0.961x | 0.639x | 1.010x |
| san_parse_all | 1.012x | 0.930x | 1.715x |
| san_render_native | 1.130x | 0.937x | 2.636x |
| apply_children_full | 1.098x | 0.962x | 1.118x |
| make_unmake_full | 1.105x | 0.913x | 1.308x |
| startpos | 0.850x | 0.850x | 1.182x |
| chess960_materialize | 1.116x | 0.979x | 1.520x |
| chess960_perft_d3 | 1.073x | 1.041x | 1.537x |
| polyglot_key | 1.120x | 1.109x | 39.815x |
| polyglot_recompute | 1.057x | 0.999x | 0.995x |
| repetition_key | 0.999x | 0.956x | 19.614x |
| corpus_count | 1.148x | 1.031x | 1.185x |
| corpus_materialize | 1.105x | 1.100x | 1.218x |
| corpus_uci_sorted | 1.051x | 1.031x | 1.371x |
| replay_trusted_native | 1.253x | 1.211x | 0.712x |
| replay_checked_native | 1.244x | 1.085x | 1.151x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.06x).
