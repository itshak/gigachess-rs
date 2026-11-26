| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.980x | 1.551x |
| materialize | 0.985x | 0.939x | 1.135x |
| perft_bulk_d4 | 0.957x | 0.836x | 1.399x |
| perft_full_d3 | 1.089x | 0.957x | 1.409x |
| fen_parse | 0.922x | 0.875x | 1.319x |
| fen_write_native_ep | 0.990x | 0.953x | 1.698x |
| snapshot | 0.976x | 0.900x | 1.118x |
| san_parse_all | 0.953x | 0.651x | 1.814x |
| san_render_native | 1.018x | 0.990x | 3.318x |
| apply_children_full | 0.973x | 0.957x | 1.281x |
| make_unmake_full | 0.999x | 0.969x | 1.306x |
| startpos | 1.003x | 1.003x | 1.002x |
| chess960_materialize | 0.974x | 0.971x | 1.360x |
| chess960_perft_d3 | 1.004x | 0.998x | 1.601x |
| polyglot_key | 1.005x | 1.002x | 20.139x |
| polyglot_recompute | 1.039x | 1.026x | 0.986x |
| repetition_key | 1.002x | 0.962x | 18.873x |
| corpus_count | 1.065x | 1.034x | 1.298x |
| corpus_materialize | 1.001x | 0.995x | 1.191x |
| corpus_uci_sorted | 1.008x | 1.007x | 1.502x |
| replay_trusted_native | 1.115x | 1.012x | 0.972x |
| replay_checked_native | 1.140x | 1.100x | 1.022x |

Cold first legal move list: 0.013 ms -> 0.016 ms (0.83x).
