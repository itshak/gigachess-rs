| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 0.991x | 1.579x |
| materialize | 1.011x | 0.992x | 1.337x |
| perft_bulk_d4 | 1.049x | 1.025x | 1.465x |
| perft_full_d3 | 1.137x | 1.018x | 1.519x |
| fen_parse | 0.994x | 0.990x | 1.213x |
| fen_write_native_ep | 0.990x | 0.983x | 1.557x |
| snapshot | 1.011x | 1.002x | 0.937x |
| san_parse_all | 0.997x | 0.993x | 1.861x |
| san_render_native | 0.994x | 0.972x | 2.669x |
| apply_children_full | 0.950x | 0.924x | 1.102x |
| make_unmake_full | 0.953x | 0.925x | 1.485x |
| startpos | 0.999x | 0.999x | 1.269x |
| chess960_materialize | 1.003x | 1.000x | 1.495x |
| chess960_perft_d3 | 1.024x | 1.004x | 1.534x |
| polyglot_key | 0.995x | 0.995x | 25.687x |
| polyglot_recompute | 0.999x | 0.999x | 1.089x |
| repetition_key | 0.996x | 0.994x | 19.221x |
| corpus_count | 1.107x | 1.077x | 1.541x |
| corpus_materialize | 1.003x | 1.002x | 1.243x |
| corpus_uci_sorted | 1.003x | 1.003x | 1.460x |
| replay_trusted_native | 0.900x | 0.868x | 0.573x |
| replay_checked_native | 0.958x | 0.957x | 1.075x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.90x).
