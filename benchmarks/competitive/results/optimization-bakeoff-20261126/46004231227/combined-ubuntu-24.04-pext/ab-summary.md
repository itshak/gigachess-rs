| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 1.000x | 1.539x |
| materialize | 1.005x | 0.997x | 1.202x |
| perft_bulk_d4 | 1.004x | 0.994x | 1.406x |
| perft_full_d3 | 0.998x | 0.978x | 1.205x |
| fen_parse | 1.003x | 0.975x | 1.137x |
| fen_write_native_ep | 0.998x | 0.992x | 1.899x |
| snapshot | 1.001x | 0.997x | 1.249x |
| san_parse_all | 1.019x | 1.003x | 2.292x |
| san_render_native | 1.002x | 0.985x | 2.119x |
| apply_children_full | 0.974x | 0.920x | 1.337x |
| make_unmake_full | 1.028x | 1.011x | 1.234x |
| startpos | 0.999x | 0.999x | 1.200x |
| chess960_materialize | 1.002x | 0.998x | 1.591x |
| chess960_perft_d3 | 1.001x | 0.998x | 1.620x |
| polyglot_key | 1.002x | 1.001x | 20.849x |
| polyglot_recompute | 1.003x | 0.995x | 1.025x |
| repetition_key | 1.007x | 0.997x | 19.454x |
| corpus_count | 0.995x | 0.993x | 1.356x |
| corpus_materialize | 0.982x | 0.978x | 1.252x |
| corpus_uci_sorted | 0.974x | 0.958x | 1.662x |
| replay_trusted_native | 0.978x | 0.953x | 1.171x |
| replay_checked_native | 1.000x | 0.998x | 1.141x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.94x).
