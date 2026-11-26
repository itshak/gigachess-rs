| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.972x | 0.916x | 1.440x |
| materialize | 0.966x | 0.919x | 1.384x |
| perft_bulk_d4 | 0.992x | 0.974x | 1.272x |
| perft_full_d3 | 0.962x | 0.878x | 1.577x |
| fen_parse | 0.873x | 0.793x | 1.226x |
| fen_write_native_ep | 0.978x | 0.893x | 1.702x |
| snapshot | 1.617x | 1.466x | 0.942x |
| san_parse_all | 1.004x | 0.893x | 1.758x |
| san_render_native | 0.985x | 0.973x | 4.169x |
| apply_children_full | 0.953x | 0.819x | 1.096x |
| make_unmake_full | 0.953x | 0.875x | 1.064x |
| startpos | 0.996x | 0.996x | 0.989x |
| chess960_materialize | 0.997x | 0.986x | 1.319x |
| chess960_perft_d3 | 0.993x | 0.987x | 1.354x |
| polyglot_key | 1.006x | 1.004x | 31.501x |
| polyglot_recompute | 0.994x | 0.978x | 0.987x |
| repetition_key | 0.989x | 0.975x | 24.623x |
| corpus_count | 0.986x | 0.944x | 1.325x |
| corpus_materialize | 1.008x | 1.004x | 1.153x |
| corpus_uci_sorted | 0.989x | 0.984x | 1.551x |
| replay_trusted_native | 0.979x | 0.972x | 0.671x |
| replay_checked_native | 0.961x | 0.922x | 1.063x |

Cold first legal move list: 0.028 ms -> 0.030 ms (0.96x).
