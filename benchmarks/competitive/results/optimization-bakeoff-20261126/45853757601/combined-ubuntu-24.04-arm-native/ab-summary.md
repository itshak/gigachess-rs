| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.994x | 1.552x |
| materialize | 1.001x | 0.999x | 1.320x |
| perft_bulk_d4 | 1.002x | 1.000x | 1.402x |
| perft_full_d3 | 1.001x | 1.000x | 1.331x |
| fen_parse | 1.010x | 1.000x | 1.218x |
| fen_write_native_ep | 1.002x | 1.001x | 1.607x |
| snapshot | 0.990x | 0.984x | 0.961x |
| san_parse_all | 0.998x | 0.994x | 1.830x |
| san_render_native | 1.004x | 0.991x | 2.526x |
| apply_children_full | 0.987x | 0.980x | 1.141x |
| make_unmake_full | 1.020x | 0.996x | 1.558x |
| startpos | 0.999x | 0.999x | 1.333x |
| chess960_materialize | 1.002x | 1.000x | 1.496x |
| chess960_perft_d3 | 1.008x | 1.006x | 1.503x |
| polyglot_key | 0.986x | 0.984x | 25.669x |
| polyglot_recompute | 0.995x | 0.995x | 1.081x |
| repetition_key | 0.988x | 0.977x | 19.231x |
| corpus_count | 1.006x | 1.002x | 1.363x |
| corpus_materialize | 0.998x | 0.998x | 1.228x |
| corpus_uci_sorted | 1.000x | 0.996x | 1.471x |
| replay_trusted_native | 0.981x | 0.968x | 0.613x |
| replay_checked_native | 1.001x | 0.993x | 1.050x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.96x).
