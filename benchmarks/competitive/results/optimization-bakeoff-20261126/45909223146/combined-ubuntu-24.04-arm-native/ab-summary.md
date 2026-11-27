| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.994x | 1.553x |
| materialize | 1.003x | 1.000x | 1.336x |
| perft_bulk_d4 | 1.034x | 1.016x | 1.438x |
| perft_full_d3 | 0.995x | 0.985x | 1.325x |
| fen_parse | 0.994x | 0.984x | 1.216x |
| fen_write_native_ep | 0.999x | 0.996x | 1.576x |
| snapshot | 0.999x | 0.994x | 0.946x |
| san_parse_all | 0.999x | 0.992x | 1.814x |
| san_render_native | 0.999x | 0.992x | 2.672x |
| apply_children_full | 0.996x | 0.989x | 1.155x |
| make_unmake_full | 1.006x | 0.999x | 1.565x |
| startpos | 0.999x | 0.999x | 1.301x |
| chess960_materialize | 0.996x | 0.996x | 1.494x |
| chess960_perft_d3 | 1.018x | 1.003x | 1.524x |
| polyglot_key | 0.988x | 0.988x | 25.106x |
| polyglot_recompute | 0.999x | 0.998x | 1.091x |
| repetition_key | 1.029x | 0.999x | 18.939x |
| corpus_count | 1.090x | 1.051x | 1.493x |
| corpus_materialize | 0.997x | 0.994x | 1.237x |
| corpus_uci_sorted | 0.997x | 0.994x | 1.459x |
| replay_trusted_native | 0.993x | 0.986x | 0.631x |
| replay_checked_native | 0.982x | 0.973x | 1.111x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.96x).
