| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.975x | 1.497x |
| materialize | 1.006x | 0.997x | 1.172x |
| perft_bulk_d4 | 1.009x | 0.985x | 1.350x |
| perft_full_d3 | 0.999x | 0.992x | 1.218x |
| fen_parse | 0.999x | 0.989x | 1.169x |
| fen_write_native_ep | 0.995x | 0.975x | 1.853x |
| snapshot | 0.979x | 0.879x | 1.221x |
| san_parse_all | 0.998x | 0.993x | 2.219x |
| san_render_native | 1.006x | 0.988x | 2.086x |
| apply_children_full | 1.006x | 1.002x | 1.296x |
| make_unmake_full | 0.980x | 0.928x | 1.185x |
| startpos | 1.001x | 1.001x | 1.206x |
| chess960_materialize | 1.015x | 1.012x | 1.566x |
| chess960_perft_d3 | 1.008x | 1.006x | 1.599x |
| polyglot_key | 1.002x | 1.002x | 20.726x |
| polyglot_recompute | 1.002x | 0.991x | 1.022x |
| repetition_key | 1.000x | 0.999x | 18.936x |
| corpus_count | 1.003x | 0.994x | 1.308x |
| corpus_materialize | 0.980x | 0.973x | 1.211x |
| corpus_uci_sorted | 0.975x | 0.973x | 1.653x |
| replay_trusted_native | 1.069x | 0.995x | 1.135x |
| replay_checked_native | 1.007x | 0.999x | 1.150x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.03x).
