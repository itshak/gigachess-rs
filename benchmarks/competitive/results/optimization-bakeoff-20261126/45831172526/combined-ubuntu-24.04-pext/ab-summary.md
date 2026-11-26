| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.961x | 1.317x |
| materialize | 0.979x | 0.965x | 1.151x |
| perft_bulk_d4 | 0.989x | 0.953x | 1.165x |
| perft_full_d3 | 0.940x | 0.885x | 1.154x |
| fen_parse | 0.971x | 0.895x | 1.180x |
| fen_write_native_ep | 1.003x | 0.976x | 1.466x |
| snapshot | 0.716x | 0.692x | 1.066x |
| san_parse_all | 0.992x | 0.975x | 1.758x |
| san_render_native | 1.029x | 1.001x | 2.298x |
| apply_children_full | 1.040x | 1.006x | 1.352x |
| make_unmake_full | 0.949x | 0.857x | 1.223x |
| startpos | 0.997x | 0.997x | 1.256x |
| chess960_materialize | 0.997x | 0.981x | 1.419x |
| chess960_perft_d3 | 0.993x | 0.990x | 1.398x |
| polyglot_key | 1.025x | 1.003x | 35.393x |
| polyglot_recompute | 1.087x | 1.015x | 1.039x |
| repetition_key | 1.026x | 1.025x | 27.519x |
| corpus_count | 1.012x | 1.007x | 1.091x |
| corpus_materialize | 1.010x | 1.007x | 1.079x |
| corpus_uci_sorted | 1.010x | 1.004x | 1.666x |
| replay_trusted_native | 0.969x | 0.963x | 0.689x |
| replay_checked_native | 0.997x | 0.987x | 1.143x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.98x).
