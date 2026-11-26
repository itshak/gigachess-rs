| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.997x | 1.358x |
| materialize | 0.995x | 0.991x | 1.227x |
| perft_bulk_d4 | 0.999x | 0.992x | 1.348x |
| perft_full_d3 | 0.995x | 0.993x | 1.081x |
| fen_parse | 0.996x | 0.988x | 1.233x |
| fen_write_native_ep | 1.002x | 0.996x | 1.066x |
| snapshot | 0.985x | 0.980x | 0.897x |
| san_parse_all | 1.001x | 0.997x | 1.842x |
| san_render_native | 0.997x | 0.984x | 2.529x |
| apply_children_full | 1.136x | 1.084x | 1.181x |
| make_unmake_full | 1.135x | 1.103x | 1.617x |
| startpos | 1.000x | 1.000x | 1.184x |
| chess960_materialize | 1.003x | 1.000x | 1.319x |
| chess960_perft_d3 | 0.994x | 0.994x | 1.312x |
| polyglot_key | 0.998x | 0.993x | 26.225x |
| polyglot_recompute | 0.993x | 0.987x | 1.071x |
| repetition_key | 1.006x | 1.000x | 19.533x |
| corpus_count | 0.996x | 0.993x | 1.451x |
| corpus_materialize | 0.995x | 0.992x | 1.187x |
| corpus_uci_sorted | 0.995x | 0.995x | 1.455x |
| replay_trusted_native | 1.223x | 1.149x | 0.620x |
| replay_checked_native | 0.969x | 0.957x | 0.964x |

Cold first legal move list: 0.010 ms -> 0.012 ms (0.88x).
