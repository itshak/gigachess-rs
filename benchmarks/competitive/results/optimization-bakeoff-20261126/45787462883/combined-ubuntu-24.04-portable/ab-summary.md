| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.988x | 1.202x |
| materialize | 0.984x | 0.961x | 1.288x |
| perft_bulk_d4 | 0.968x | 0.937x | 1.141x |
| perft_full_d3 | 0.974x | 0.957x | 1.145x |
| fen_parse | 0.914x | 0.902x | 1.106x |
| fen_write_native_ep | 0.995x | 0.975x | 1.875x |
| snapshot | 0.999x | 0.998x | 0.922x |
| san_parse_all | 0.936x | 0.919x | 2.035x |
| san_render_native | 0.875x | 0.841x | 2.191x |
| apply_children_full | 0.961x | 0.911x | 1.046x |
| make_unmake_full | 0.914x | 0.814x | 1.249x |
| startpos | 0.999x | 0.999x | 1.310x |
| chess960_materialize | 1.042x | 1.028x | 1.475x |
| chess960_perft_d3 | 0.964x | 0.961x | 1.357x |
| polyglot_key | 1.006x | 0.998x | 21.833x |
| polyglot_recompute | 0.988x | 0.979x | 0.965x |
| repetition_key | 0.984x | 0.971x | 20.259x |
| corpus_count | 1.001x | 1.000x | 1.219x |
| corpus_materialize | 0.993x | 0.988x | 1.296x |
| corpus_uci_sorted | 1.003x | 1.002x | 1.594x |
| replay_trusted_native | 1.073x | 0.959x | 0.741x |
| replay_checked_native | 0.950x | 0.936x | 0.952x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.17x).
