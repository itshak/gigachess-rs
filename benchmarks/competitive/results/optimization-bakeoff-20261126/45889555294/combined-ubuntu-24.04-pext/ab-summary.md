| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.900x | 0.883x | 1.409x |
| materialize | 0.953x | 0.945x | 1.505x |
| perft_bulk_d4 | 0.938x | 0.899x | 1.112x |
| perft_full_d3 | 0.916x | 0.905x | 1.221x |
| fen_parse | 1.026x | 0.985x | 1.151x |
| fen_write_native_ep | 0.904x | 0.839x | 1.738x |
| snapshot | 0.751x | 0.749x | 1.147x |
| san_parse_all | 0.965x | 0.947x | 1.905x |
| san_render_native | 1.011x | 0.952x | 2.105x |
| apply_children_full | 0.726x | 0.693x | 1.025x |
| make_unmake_full | 0.981x | 0.933x | 1.173x |
| startpos | 1.253x | 1.253x | 1.586x |
| chess960_materialize | 0.972x | 0.930x | 1.387x |
| chess960_perft_d3 | 0.938x | 0.899x | 1.383x |
| polyglot_key | 1.006x | 0.999x | 38.821x |
| polyglot_recompute | 1.004x | 1.000x | 1.015x |
| repetition_key | 0.925x | 0.921x | 21.956x |
| corpus_count | 0.900x | 0.845x | 0.993x |
| corpus_materialize | 0.990x | 0.988x | 1.076x |
| corpus_uci_sorted | 1.047x | 1.046x | 1.607x |
| replay_trusted_native | 0.988x | 0.972x | 0.869x |
| replay_checked_native | 0.886x | 0.871x | 1.068x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.79x).
