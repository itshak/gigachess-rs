| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 1.000x | 1.291x |
| materialize | 0.998x | 0.996x | 1.199x |
| perft_bulk_d4 | 0.994x | 0.987x | 1.275x |
| perft_full_d3 | 0.999x | 0.993x | 0.746x |
| fen_parse | 1.002x | 0.996x | 1.238x |
| fen_write_native_ep | 0.993x | 0.986x | 1.045x |
| snapshot | 1.000x | 0.995x | 0.943x |
| san_parse_all | 0.998x | 0.993x | 1.785x |
| san_render_native | 1.004x | 0.995x | 2.429x |
| apply_children_full | 0.996x | 0.975x | 0.833x |
| make_unmake_full | 0.994x | 0.977x | 1.129x |
| startpos | 1.000x | 1.000x | 1.149x |
| chess960_materialize | 0.996x | 0.995x | 1.227x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.227x |
| polyglot_key | 1.004x | 1.001x | 25.385x |
| polyglot_recompute | 0.998x | 0.998x | 1.066x |
| repetition_key | 0.996x | 0.991x | 18.641x |
| corpus_count | 0.990x | 0.966x | 1.380x |
| corpus_materialize | 0.986x | 0.979x | 1.165x |
| corpus_uci_sorted | 1.001x | 0.998x | 0.968x |
| replay_trusted_native | 0.997x | 0.989x | 0.421x |
| replay_checked_native | 0.935x | 0.930x | 0.820x |

Cold first legal move list: 0.013 ms -> 0.011 ms (1.14x).
