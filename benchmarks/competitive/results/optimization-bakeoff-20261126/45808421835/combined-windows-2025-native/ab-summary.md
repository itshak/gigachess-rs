| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.995x | 1.367x |
| materialize | 0.992x | 0.984x | 1.086x |
| perft_bulk_d4 | 0.997x | 0.963x | 1.329x |
| perft_full_d3 | 1.022x | 0.973x | 1.032x |
| fen_parse | 0.791x | 0.779x | 1.222x |
| fen_write_native_ep | 0.999x | 0.987x | 1.590x |
| snapshot | 0.487x | 0.477x | 1.078x |
| san_parse_all | 1.022x | 0.984x | 1.900x |
| san_render_native | 0.985x | 0.971x | 3.474x |
| apply_children_full | 1.035x | 0.991x | 1.434x |
| make_unmake_full | 1.037x | 0.997x | 1.282x |
| startpos | 0.995x | 0.995x | 2.535x |
| chess960_materialize | 1.003x | 0.998x | 1.278x |
| chess960_perft_d3 | 1.007x | 0.993x | 1.493x |
| polyglot_key | 0.980x | 0.972x | 20.286x |
| polyglot_recompute | 0.979x | 0.967x | 0.952x |
| repetition_key | 1.066x | 1.011x | 16.804x |
| corpus_count | 0.952x | 0.929x | 1.241x |
| corpus_materialize | 1.007x | 1.007x | 1.169x |
| corpus_uci_sorted | 1.092x | 1.054x | 1.549x |
| replay_trusted_native | 1.038x | 0.969x | 0.831x |
| replay_checked_native | 1.172x | 1.165x | 0.930x |

Cold first legal move list: 0.029 ms -> 0.028 ms (1.06x).
