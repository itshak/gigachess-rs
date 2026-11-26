| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.976x | 0.931x | 1.148x |
| materialize | 0.976x | 0.949x | 1.671x |
| perft_bulk_d4 | 0.963x | 0.952x | 1.130x |
| perft_full_d3 | 0.883x | 0.859x | 1.329x |
| fen_parse | 0.933x | 0.891x | 1.064x |
| fen_write_native_ep | 0.997x | 0.980x | 1.835x |
| snapshot | 1.001x | 0.994x | 1.250x |
| san_parse_all | 0.925x | 0.864x | 1.753x |
| san_render_native | 0.918x | 0.876x | 1.969x |
| apply_children_full | 0.945x | 0.921x | 1.094x |
| make_unmake_full | 0.893x | 0.853x | 1.100x |
| startpos | 1.165x | 1.165x | 1.501x |
| chess960_materialize | 0.933x | 0.879x | 1.321x |
| chess960_perft_d3 | 0.930x | 0.919x | 1.243x |
| polyglot_key | 1.009x | 1.003x | 44.391x |
| polyglot_recompute | 0.955x | 0.911x | 1.034x |
| repetition_key | 1.085x | 0.959x | 23.824x |
| corpus_count | 0.986x | 0.971x | 1.226x |
| corpus_materialize | 1.015x | 1.011x | 1.161x |
| corpus_uci_sorted | 0.987x | 0.983x | 1.549x |
| replay_trusted_native | 0.781x | 0.701x | 0.543x |
| replay_checked_native | 0.696x | 0.593x | 0.784x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.98x).
