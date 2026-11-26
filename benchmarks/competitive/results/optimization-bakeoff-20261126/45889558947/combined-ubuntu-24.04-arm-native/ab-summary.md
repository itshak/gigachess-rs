| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.998x | 1.553x |
| materialize | 0.997x | 0.991x | 1.324x |
| perft_bulk_d4 | 0.998x | 0.996x | 1.391x |
| perft_full_d3 | 1.003x | 1.001x | 1.340x |
| fen_parse | 0.983x | 0.935x | 1.208x |
| fen_write_native_ep | 0.999x | 0.996x | 1.576x |
| snapshot | 1.007x | 1.004x | 0.945x |
| san_parse_all | 1.002x | 0.998x | 1.841x |
| san_render_native | 1.009x | 1.001x | 2.677x |
| apply_children_full | 1.005x | 0.999x | 1.162x |
| make_unmake_full | 0.992x | 0.987x | 1.557x |
| startpos | 1.000x | 1.000x | 1.307x |
| chess960_materialize | 0.997x | 0.986x | 1.495x |
| chess960_perft_d3 | 1.005x | 1.001x | 1.503x |
| polyglot_key | 0.995x | 0.987x | 25.688x |
| polyglot_recompute | 0.992x | 0.921x | 1.081x |
| repetition_key | 0.992x | 0.985x | 19.380x |
| corpus_count | 0.999x | 0.996x | 1.366x |
| corpus_materialize | 0.996x | 0.995x | 1.229x |
| corpus_uci_sorted | 0.998x | 0.994x | 1.462x |
| replay_trusted_native | 0.993x | 0.981x | 0.616x |
| replay_checked_native | 1.000x | 1.000x | 1.131x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.97x).
