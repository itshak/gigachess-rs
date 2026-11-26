| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.981x | 0.920x | 1.272x |
| materialize | 0.999x | 0.977x | 1.173x |
| perft_bulk_d4 | 1.020x | 0.996x | 1.257x |
| perft_full_d3 | 0.678x | 0.610x | 0.844x |
| fen_parse | 1.091x | 1.048x | 1.366x |
| fen_write_native_ep | 0.990x | 0.951x | 1.501x |
| snapshot | 1.130x | 1.093x | 1.190x |
| san_parse_all | 1.038x | 1.008x | 1.994x |
| san_render_native | 1.097x | 0.946x | 3.822x |
| apply_children_full | 1.221x | 1.174x | 1.058x |
| make_unmake_full | 1.008x | 0.970x | 1.248x |
| startpos | 1.154x | 1.154x | 1.246x |
| chess960_materialize | 1.014x | 0.965x | 1.157x |
| chess960_perft_d3 | 1.017x | 1.002x | 1.317x |
| polyglot_key | 0.916x | 0.805x | 33.596x |
| polyglot_recompute | 1.036x | 0.965x | 0.966x |
| repetition_key | 0.872x | 0.849x | 14.228x |
| corpus_count | 1.073x | 0.974x | 1.222x |
| corpus_materialize | 1.137x | 1.083x | 1.192x |
| corpus_uci_sorted | 1.004x | 0.998x | 1.379x |
| replay_trusted_native | 0.983x | 0.963x | 0.621x |
| replay_checked_native | 1.016x | 1.015x | 0.937x |

Cold first legal move list: 0.019 ms -> 0.019 ms (0.96x).
