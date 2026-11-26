| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.958x | 0.919x | 1.428x |
| materialize | 0.970x | 0.915x | 1.507x |
| perft_bulk_d4 | 0.847x | 0.764x | 1.061x |
| perft_full_d3 | 0.996x | 0.911x | 1.477x |
| fen_parse | 0.932x | 0.859x | 1.007x |
| fen_write_native_ep | 1.006x | 0.976x | 2.139x |
| snapshot | 1.000x | 0.997x | 1.153x |
| san_parse_all | 0.981x | 0.973x | 1.889x |
| san_render_native | 0.994x | 0.987x | 2.126x |
| apply_children_full | 1.069x | 1.018x | 1.287x |
| make_unmake_full | 1.052x | 1.006x | 1.155x |
| startpos | 1.123x | 1.123x | 1.118x |
| chess960_materialize | 1.021x | 0.999x | 1.333x |
| chess960_perft_d3 | 1.003x | 0.986x | 1.387x |
| polyglot_key | 1.059x | 1.000x | 37.816x |
| polyglot_recompute | 1.001x | 0.998x | 1.009x |
| repetition_key | 0.844x | 0.743x | 15.657x |
| corpus_count | 0.687x | 0.624x | 0.963x |
| corpus_materialize | 0.948x | 0.934x | 1.053x |
| corpus_uci_sorted | 0.987x | 0.983x | 1.548x |
| replay_trusted_native | 1.082x | 1.066x | 0.895x |
| replay_checked_native | 0.995x | 0.983x | 1.123x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.25x).
