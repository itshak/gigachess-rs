| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.996x | 1.287x |
| materialize | 0.994x | 0.985x | 1.340x |
| perft_bulk_d4 | 0.995x | 0.984x | 1.296x |
| perft_full_d3 | 1.195x | 1.034x | 1.522x |
| fen_parse | 1.011x | 0.991x | 1.094x |
| fen_write_native_ep | 1.041x | 1.002x | 1.724x |
| snapshot | 0.999x | 0.996x | 0.974x |
| san_parse_all | 1.001x | 0.994x | 1.997x |
| san_render_native | 1.034x | 0.993x | 2.333x |
| apply_children_full | 1.019x | 0.977x | 1.406x |
| make_unmake_full | 0.977x | 0.942x | 1.417x |
| startpos | 1.000x | 1.000x | 1.330x |
| chess960_materialize | 0.969x | 0.956x | 1.433x |
| chess960_perft_d3 | 1.007x | 1.004x | 1.564x |
| polyglot_key | 1.001x | 0.997x | 23.480x |
| polyglot_recompute | 0.983x | 0.976x | 1.000x |
| repetition_key | 0.991x | 0.978x | 17.836x |
| corpus_count | 1.008x | 1.006x | 1.296x |
| corpus_materialize | 0.994x | 0.991x | 1.211x |
| corpus_uci_sorted | 0.997x | 0.988x | 1.834x |
| replay_trusted_native | 0.993x | 0.993x | 0.891x |
| replay_checked_native | 0.996x | 0.990x | 1.135x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.09x).
