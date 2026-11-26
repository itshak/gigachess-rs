| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.733x | 1.660x |
| materialize | 0.889x | 0.790x | 1.075x |
| perft_bulk_d4 | 1.026x | 0.930x | 1.236x |
| perft_full_d3 | 1.101x | 0.912x | 1.438x |
| fen_parse | 1.015x | 0.755x | 1.132x |
| fen_write_native_ep | 0.952x | 0.831x | 1.770x |
| snapshot | 0.990x | 0.758x | 0.974x |
| san_parse_all | 0.916x | 0.793x | 1.654x |
| san_render_native | 0.936x | 0.772x | 2.867x |
| apply_children_full | 0.908x | 0.766x | 1.088x |
| make_unmake_full | 0.953x | 0.704x | 1.369x |
| startpos | 0.533x | 0.533x | 0.822x |
| chess960_materialize | 0.877x | 0.728x | 1.650x |
| chess960_perft_d3 | 0.831x | 0.801x | 1.462x |
| polyglot_key | 0.814x | 0.797x | 39.325x |
| polyglot_recompute | 0.922x | 0.800x | 1.170x |
| repetition_key | 1.057x | 0.996x | 21.989x |
| corpus_count | 0.931x | 0.895x | 1.398x |
| corpus_materialize | 0.840x | 0.826x | 1.062x |
| corpus_uci_sorted | 0.907x | 0.891x | 1.465x |
| replay_trusted_native | 1.049x | 1.046x | 0.579x |
| replay_checked_native | 0.944x | 0.854x | 1.214x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.23x).
