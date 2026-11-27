| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.988x | 1.541x |
| materialize | 1.019x | 0.982x | 1.142x |
| perft_bulk_d4 | 1.012x | 0.981x | 1.432x |
| perft_full_d3 | 0.985x | 0.955x | 1.220x |
| fen_parse | 1.018x | 0.907x | 1.367x |
| fen_write_native_ep | 1.048x | 0.961x | 1.571x |
| snapshot | 1.004x | 0.981x | 1.167x |
| san_parse_all | 0.997x | 0.987x | 1.989x |
| san_render_native | 0.991x | 0.969x | 3.337x |
| apply_children_full | 0.994x | 0.979x | 1.311x |
| make_unmake_full | 0.977x | 0.895x | 1.251x |
| startpos | 0.998x | 0.998x | 1.000x |
| chess960_materialize | 1.004x | 1.003x | 1.363x |
| chess960_perft_d3 | 0.993x | 0.991x | 1.584x |
| polyglot_key | 1.024x | 0.999x | 20.894x |
| polyglot_recompute | 0.972x | 0.932x | 0.981x |
| repetition_key | 0.972x | 0.963x | 19.090x |
| corpus_count | 0.993x | 0.990x | 1.365x |
| corpus_materialize | 0.999x | 0.999x | 1.223x |
| corpus_uci_sorted | 0.992x | 0.985x | 1.492x |
| replay_trusted_native | 0.960x | 0.946x | 1.028x |
| replay_checked_native | 0.974x | 0.954x | 1.066x |

Cold first legal move list: 0.014 ms -> 0.017 ms (0.86x).
