| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.023x | 0.981x | 1.477x |
| materialize | 1.019x | 0.979x | 1.238x |
| perft_bulk_d4 | 1.011x | 0.993x | 1.373x |
| perft_full_d3 | 1.218x | 1.037x | 1.586x |
| fen_parse | 1.097x | 1.043x | 1.313x |
| fen_write_native_ep | 1.010x | 0.993x | 1.606x |
| snapshot | 0.998x | 0.989x | 0.997x |
| san_parse_all | 1.001x | 0.922x | 2.055x |
| san_render_native | 1.049x | 1.000x | 3.956x |
| apply_children_full | 1.055x | 0.966x | 1.397x |
| make_unmake_full | 0.985x | 0.884x | 1.308x |
| startpos | 1.235x | 1.235x | 1.254x |
| chess960_materialize | 1.016x | 0.994x | 1.317x |
| chess960_perft_d3 | 0.994x | 0.976x | 1.531x |
| polyglot_key | 1.007x | 1.000x | 28.201x |
| polyglot_recompute | 0.989x | 0.977x | 1.036x |
| repetition_key | 1.090x | 1.026x | 21.592x |
| corpus_count | 0.976x | 0.962x | 1.365x |
| corpus_materialize | 0.979x | 0.950x | 1.145x |
| corpus_uci_sorted | 0.993x | 0.991x | 1.435x |
| replay_trusted_native | 0.937x | 0.905x | 0.807x |
| replay_checked_native | 1.012x | 1.001x | 1.151x |

Cold first legal move list: 0.018 ms -> 0.018 ms (1.01x).
