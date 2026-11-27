| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.991x | 0.971x | 1.532x |
| materialize | 0.994x | 0.988x | 1.320x |
| perft_bulk_d4 | 0.996x | 0.990x | 1.385x |
| perft_full_d3 | 0.979x | 0.953x | 1.304x |
| fen_parse | 0.987x | 0.983x | 1.210x |
| fen_write_native_ep | 0.999x | 0.994x | 1.564x |
| snapshot | 1.001x | 0.995x | 0.939x |
| san_parse_all | 0.990x | 0.985x | 1.796x |
| san_render_native | 0.969x | 0.938x | 2.591x |
| apply_children_full | 0.975x | 0.925x | 1.127x |
| make_unmake_full | 0.989x | 0.967x | 1.543x |
| startpos | 1.001x | 1.001x | 1.308x |
| chess960_materialize | 1.000x | 0.996x | 1.491x |
| chess960_perft_d3 | 0.994x | 0.990x | 1.491x |
| polyglot_key | 0.985x | 0.970x | 25.176x |
| polyglot_recompute | 0.996x | 0.994x | 1.084x |
| repetition_key | 0.986x | 0.980x | 18.667x |
| corpus_count | 1.002x | 0.998x | 1.355x |
| corpus_materialize | 0.994x | 0.990x | 1.227x |
| corpus_uci_sorted | 1.004x | 1.001x | 1.462x |
| replay_trusted_native | 0.976x | 0.960x | 0.619x |
| replay_checked_native | 1.002x | 0.996x | 1.133x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).
