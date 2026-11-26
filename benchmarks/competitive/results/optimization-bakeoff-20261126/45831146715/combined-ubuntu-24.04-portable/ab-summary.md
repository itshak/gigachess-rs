| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 1.000x | 1.227x |
| materialize | 1.000x | 0.990x | 1.653x |
| perft_bulk_d4 | 0.994x | 0.964x | 1.064x |
| perft_full_d3 | 0.884x | 0.849x | 1.267x |
| fen_parse | 0.988x | 0.972x | 1.099x |
| fen_write_native_ep | 0.984x | 0.960x | 1.753x |
| snapshot | 1.000x | 0.996x | 0.998x |
| san_parse_all | 1.000x | 0.968x | 2.055x |
| san_render_native | 0.957x | 0.935x | 2.041x |
| apply_children_full | 0.869x | 0.773x | 1.144x |
| make_unmake_full | 0.834x | 0.790x | 1.039x |
| startpos | 1.003x | 1.003x | 1.358x |
| chess960_materialize | 0.917x | 0.811x | 1.270x |
| chess960_perft_d3 | 0.952x | 0.906x | 1.313x |
| polyglot_key | 1.044x | 1.002x | 40.680x |
| polyglot_recompute | 1.016x | 0.995x | 0.975x |
| repetition_key | 1.063x | 1.015x | 20.085x |
| corpus_count | 1.005x | 1.000x | 1.011x |
| corpus_materialize | 0.998x | 0.997x | 1.089x |
| corpus_uci_sorted | 1.016x | 1.016x | 1.560x |
| replay_trusted_native | 0.795x | 0.721x | 0.724x |
| replay_checked_native | 0.976x | 0.960x | 1.011x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.93x).
