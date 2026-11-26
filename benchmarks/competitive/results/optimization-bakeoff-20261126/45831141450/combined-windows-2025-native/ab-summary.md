| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.956x | 0.906x | 1.285x |
| materialize | 1.045x | 0.998x | 1.058x |
| perft_bulk_d4 | 0.980x | 0.942x | 1.192x |
| perft_full_d3 | 0.950x | 0.908x | 1.123x |
| fen_parse | 1.258x | 1.164x | 1.437x |
| fen_write_native_ep | 0.939x | 0.875x | 1.501x |
| snapshot | 1.001x | 0.983x | 1.107x |
| san_parse_all | 1.036x | 0.986x | 1.964x |
| san_render_native | 0.999x | 0.968x | 3.348x |
| apply_children_full | 1.141x | 0.901x | 1.197x |
| make_unmake_full | 0.980x | 0.961x | 1.246x |
| startpos | 1.000x | 1.000x | 0.999x |
| chess960_materialize | 1.007x | 0.943x | 1.198x |
| chess960_perft_d3 | 1.104x | 1.028x | 1.373x |
| polyglot_key | 1.001x | 1.001x | 20.238x |
| polyglot_recompute | 1.000x | 0.993x | 0.951x |
| repetition_key | 1.035x | 1.034x | 19.061x |
| corpus_count | 0.999x | 0.970x | 1.130x |
| corpus_materialize | 1.017x | 1.001x | 1.103x |
| corpus_uci_sorted | 1.110x | 1.080x | 1.509x |
| replay_trusted_native | 0.989x | 0.929x | 1.015x |
| replay_checked_native | 1.043x | 0.990x | 0.993x |

Cold first legal move list: 0.014 ms -> 0.016 ms (0.87x).
