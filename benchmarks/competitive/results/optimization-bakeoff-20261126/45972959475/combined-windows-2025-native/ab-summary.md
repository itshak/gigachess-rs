| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.042x | 0.952x | 1.494x |
| materialize | 1.023x | 0.997x | 1.266x |
| perft_bulk_d4 | 1.055x | 1.003x | 1.362x |
| perft_full_d3 | 1.068x | 1.007x | 1.374x |
| fen_parse | 1.099x | 1.029x | 1.319x |
| fen_write_native_ep | 0.989x | 0.979x | 1.733x |
| snapshot | 1.042x | 0.987x | 0.992x |
| san_parse_all | 1.035x | 0.992x | 2.063x |
| san_render_native | 1.055x | 1.000x | 3.885x |
| apply_children_full | 1.035x | 0.944x | 1.436x |
| make_unmake_full | 1.092x | 1.002x | 1.423x |
| startpos | 1.444x | 1.444x | 1.250x |
| chess960_materialize | 1.118x | 1.104x | 1.373x |
| chess960_perft_d3 | 1.101x | 1.095x | 1.567x |
| polyglot_key | 0.968x | 0.907x | 25.291x |
| polyglot_recompute | 1.043x | 1.015x | 1.015x |
| repetition_key | 0.978x | 0.941x | 19.878x |
| corpus_count | 1.009x | 1.005x | 1.367x |
| corpus_materialize | 1.008x | 1.006x | 1.185x |
| corpus_uci_sorted | 1.000x | 0.996x | 1.416x |
| replay_trusted_native | 1.017x | 0.981x | 0.830x |
| replay_checked_native | 1.007x | 0.988x | 1.083x |

Cold first legal move list: 0.020 ms -> 0.018 ms (1.08x).
