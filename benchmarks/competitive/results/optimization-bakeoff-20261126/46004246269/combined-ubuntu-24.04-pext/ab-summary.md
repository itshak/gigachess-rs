| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.064x | 1.023x | 1.568x |
| materialize | 1.013x | 0.994x | 1.289x |
| perft_bulk_d4 | 1.032x | 0.970x | 1.448x |
| perft_full_d3 | 1.209x | 1.018x | 1.558x |
| fen_parse | 1.003x | 0.992x | 1.243x |
| fen_write_native_ep | 0.963x | 0.931x | 1.707x |
| snapshot | 0.778x | 0.776x | 0.974x |
| san_parse_all | 0.996x | 0.984x | 2.112x |
| san_render_native | 0.997x | 0.982x | 2.208x |
| apply_children_full | 1.003x | 0.986x | 1.325x |
| make_unmake_full | 1.015x | 0.968x | 1.321x |
| startpos | 1.000x | 1.000x | 1.251x |
| chess960_materialize | 0.983x | 0.977x | 1.388x |
| chess960_perft_d3 | 1.011x | 0.990x | 1.506x |
| polyglot_key | 1.071x | 1.043x | 29.811x |
| polyglot_recompute | 1.000x | 0.998x | 1.039x |
| repetition_key | 0.992x | 0.988x | 21.147x |
| corpus_count | 1.092x | 1.077x | 1.570x |
| corpus_materialize | 0.985x | 0.981x | 1.183x |
| corpus_uci_sorted | 0.998x | 0.994x | 1.814x |
| replay_trusted_native | 0.961x | 0.943x | 0.867x |
| replay_checked_native | 0.956x | 0.949x | 1.077x |

Cold first legal move list: 0.009 ms -> 0.007 ms (1.29x).
