| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.993x | 0.958x | 1.478x |
| materialize | 1.040x | 1.032x | 1.166x |
| perft_bulk_d4 | 1.017x | 1.004x | 1.354x |
| perft_full_d3 | 1.014x | 0.972x | 0.935x |
| fen_parse | 0.942x | 0.895x | 1.085x |
| fen_write_native_ep | 0.996x | 0.977x | 1.921x |
| snapshot | 0.527x | 0.519x | 0.659x |
| san_parse_all | 0.999x | 0.993x | 2.200x |
| san_render_native | 0.990x | 0.927x | 2.090x |
| apply_children_full | 0.982x | 0.945x | 1.434x |
| make_unmake_full | 0.960x | 0.943x | 1.188x |
| startpos | 1.002x | 1.002x | 1.202x |
| chess960_materialize | 0.987x | 0.980x | 1.591x |
| chess960_perft_d3 | 0.995x | 0.988x | 1.592x |
| polyglot_key | 1.012x | 1.001x | 20.799x |
| polyglot_recompute | 0.581x | 0.571x | 0.579x |
| repetition_key | 0.998x | 0.995x | 19.079x |
| corpus_count | 0.996x | 0.994x | 1.296x |
| corpus_materialize | 1.012x | 1.007x | 1.274x |
| corpus_uci_sorted | 0.983x | 0.970x | 1.651x |
| replay_trusted_native | 0.973x | 0.957x | 1.115x |
| replay_checked_native | 0.992x | 0.971x | 1.022x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.07x).
