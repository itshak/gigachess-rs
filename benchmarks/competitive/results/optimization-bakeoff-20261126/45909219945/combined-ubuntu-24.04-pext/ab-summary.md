| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.037x | 1.007x | 1.560x |
| materialize | 1.028x | 1.015x | 1.203x |
| perft_bulk_d4 | 1.037x | 0.993x | 1.476x |
| perft_full_d3 | 1.010x | 0.989x | 1.224x |
| fen_parse | 0.882x | 0.855x | 1.083x |
| fen_write_native_ep | 0.999x | 0.986x | 1.926x |
| snapshot | 1.000x | 0.996x | 1.249x |
| san_parse_all | 0.992x | 0.959x | 1.969x |
| san_render_native | 1.015x | 0.995x | 2.168x |
| apply_children_full | 0.999x | 0.993x | 1.325x |
| make_unmake_full | 0.994x | 0.947x | 1.183x |
| startpos | 1.001x | 1.001x | 1.203x |
| chess960_materialize | 1.058x | 1.055x | 1.551x |
| chess960_perft_d3 | 0.993x | 0.979x | 1.549x |
| polyglot_key | 0.996x | 0.993x | 19.937x |
| polyglot_recompute | 0.974x | 0.966x | 0.979x |
| repetition_key | 1.001x | 0.990x | 18.592x |
| corpus_count | 1.064x | 1.023x | 1.455x |
| corpus_materialize | 1.017x | 1.015x | 1.261x |
| corpus_uci_sorted | 0.998x | 0.995x | 1.742x |
| replay_trusted_native | 1.007x | 1.000x | 1.110x |
| replay_checked_native | 1.005x | 1.001x | 1.143x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.04x).
