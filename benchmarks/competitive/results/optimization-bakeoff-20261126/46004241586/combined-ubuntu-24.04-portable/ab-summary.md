| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.055x | 0.994x | 1.354x |
| materialize | 1.001x | 0.993x | 1.779x |
| perft_bulk_d4 | 1.062x | 1.027x | 1.263x |
| perft_full_d3 | 1.165x | 1.044x | 1.549x |
| fen_parse | 0.914x | 0.800x | 1.070x |
| fen_write_native_ep | 1.002x | 0.963x | 1.893x |
| snapshot | 0.997x | 0.980x | 0.999x |
| san_parse_all | 0.995x | 0.968x | 1.933x |
| san_render_native | 0.959x | 0.939x | 2.095x |
| apply_children_full | 0.999x | 0.995x | 1.342x |
| make_unmake_full | 1.049x | 0.978x | 1.265x |
| startpos | 1.002x | 1.002x | 1.431x |
| chess960_materialize | 0.947x | 0.940x | 1.480x |
| chess960_perft_d3 | 1.015x | 0.999x | 1.515x |
| polyglot_key | 1.001x | 0.998x | 41.600x |
| polyglot_recompute | 1.002x | 1.002x | 0.976x |
| repetition_key | 1.166x | 1.014x | 24.615x |
| corpus_count | 1.185x | 1.107x | 1.338x |
| corpus_materialize | 0.987x | 0.981x | 1.130x |
| corpus_uci_sorted | 0.988x | 0.983x | 1.568x |
| replay_trusted_native | 0.989x | 0.969x | 0.879x |
| replay_checked_native | 1.003x | 0.994x | 1.139x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.99x).
