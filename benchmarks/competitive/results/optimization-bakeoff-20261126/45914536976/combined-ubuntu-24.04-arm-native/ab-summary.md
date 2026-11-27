| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.552x |
| materialize | 1.003x | 1.000x | 1.321x |
| perft_bulk_d4 | 0.999x | 0.996x | 1.387x |
| perft_full_d3 | 1.003x | 1.001x | 1.334x |
| fen_parse | 1.001x | 0.998x | 1.229x |
| fen_write_native_ep | 1.000x | 0.997x | 1.595x |
| snapshot | 0.987x | 0.981x | 0.953x |
| san_parse_all | 0.999x | 0.991x | 1.827x |
| san_render_native | 1.000x | 0.990x | 2.715x |
| apply_children_full | 0.983x | 0.971x | 1.134x |
| make_unmake_full | 1.022x | 0.996x | 1.595x |
| startpos | 1.001x | 1.001x | 1.325x |
| chess960_materialize | 1.016x | 1.002x | 1.500x |
| chess960_perft_d3 | 1.003x | 1.002x | 1.501x |
| polyglot_key | 1.003x | 0.994x | 25.059x |
| polyglot_recompute | 0.996x | 0.994x | 1.088x |
| repetition_key | 0.995x | 0.991x | 18.894x |
| corpus_count | 1.001x | 0.997x | 1.349x |
| corpus_materialize | 0.998x | 0.996x | 1.228x |
| corpus_uci_sorted | 0.999x | 0.999x | 1.466x |
| replay_trusted_native | 0.988x | 0.979x | 0.627x |
| replay_checked_native | 1.001x | 0.998x | 1.129x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).
