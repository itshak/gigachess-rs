| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.034x | 1.001x | 1.269x |
| materialize | 1.070x | 0.981x | 1.162x |
| perft_bulk_d4 | 1.379x | 1.264x | 1.433x |
| perft_full_d3 | 2.048x | 1.813x | 1.215x |
| fen_parse | 1.204x | 1.135x | 1.333x |
| fen_write_native_ep | 1.050x | 1.012x | 1.036x |
| snapshot | 1.203x | 1.051x | 1.007x |
| san_parse_all | 2.063x | 1.940x | 1.911x |
| san_render_native | 1.932x | 1.611x | 3.665x |
| apply_children_full | 1.463x | 1.341x | 0.999x |
| make_unmake_full | 1.293x | 1.188x | 1.066x |
| startpos | 1.702x | 1.702x | 1.106x |
| chess960_materialize | 1.177x | 1.073x | 1.223x |
| chess960_perft_d3 | 1.399x | 1.245x | 1.300x |
| polyglot_key | 0.988x | 0.977x | 25.459x |
| polyglot_recompute | 0.958x | 0.908x | 0.939x |
| repetition_key | 1.402x | 1.347x | 19.969x |
| corpus_count | 1.105x | 1.059x | 1.361x |
| corpus_materialize | 1.046x | 1.045x | 1.142x |
| corpus_uci_sorted | 1.349x | 1.317x | 0.962x |
| replay_trusted_native | 1.315x | 1.216x | 0.589x |
| replay_checked_native | 1.281x | 1.210x | 0.896x |

Cold first legal move list: 236.834 ms -> 0.033 ms (7242.62x).
