| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.995x | 1.556x |
| materialize | 0.926x | 0.880x | 1.504x |
| perft_bulk_d4 | 1.006x | 0.995x | 1.198x |
| perft_full_d3 | 0.995x | 0.987x | 1.345x |
| fen_parse | 0.936x | 0.832x | 1.094x |
| fen_write_native_ep | 1.008x | 0.986x | 2.051x |
| snapshot | 0.999x | 0.989x | 0.993x |
| san_parse_all | 1.022x | 0.985x | 1.887x |
| san_render_native | 0.976x | 0.959x | 2.073x |
| apply_children_full | 0.987x | 0.960x | 1.335x |
| make_unmake_full | 0.922x | 0.902x | 1.114x |
| startpos | 0.903x | 0.903x | 0.999x |
| chess960_materialize | 1.016x | 1.012x | 1.386x |
| chess960_perft_d3 | 0.998x | 0.990x | 1.468x |
| polyglot_key | 0.995x | 0.994x | 40.366x |
| polyglot_recompute | 0.986x | 0.977x | 1.022x |
| repetition_key | 0.977x | 0.949x | 23.976x |
| corpus_count | 1.004x | 0.997x | 1.136x |
| corpus_materialize | 1.004x | 1.001x | 1.077x |
| corpus_uci_sorted | 1.012x | 1.009x | 1.563x |
| replay_trusted_native | 1.007x | 0.951x | 0.886x |
| replay_checked_native | 0.999x | 0.980x | 1.167x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).
