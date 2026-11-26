| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.025x | 0.978x | 1.202x |
| materialize | 0.938x | 0.876x | 1.558x |
| perft_bulk_d4 | 0.921x | 0.877x | 1.065x |
| perft_full_d3 | 0.939x | 0.886x | 1.419x |
| fen_parse | 1.014x | 0.983x | 1.141x |
| fen_write_native_ep | 0.992x | 0.986x | 2.030x |
| snapshot | 1.248x | 1.237x | 1.001x |
| san_parse_all | 0.988x | 0.946x | 1.873x |
| san_render_native | 0.962x | 0.934x | 2.124x |
| apply_children_full | 0.994x | 0.990x | 1.208x |
| make_unmake_full | 1.002x | 0.961x | 1.174x |
| startpos | 1.004x | 1.004x | 1.428x |
| chess960_materialize | 1.030x | 0.970x | 1.322x |
| chess960_perft_d3 | 1.048x | 0.994x | 1.396x |
| polyglot_key | 0.972x | 0.949x | 40.651x |
| polyglot_recompute | 0.991x | 0.988x | 0.995x |
| repetition_key | 0.798x | 0.706x | 18.766x |
| corpus_count | 0.785x | 0.745x | 1.003x |
| corpus_materialize | 0.922x | 0.902x | 1.088x |
| corpus_uci_sorted | 0.957x | 0.953x | 1.503x |
| replay_trusted_native | 1.007x | 0.997x | 0.813x |
| replay_checked_native | 0.991x | 0.979x | 1.074x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.68x).
