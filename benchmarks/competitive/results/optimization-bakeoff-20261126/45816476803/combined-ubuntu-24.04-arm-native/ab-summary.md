| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.393x |
| materialize | 0.996x | 0.985x | 1.244x |
| perft_bulk_d4 | 0.998x | 0.995x | 1.240x |
| perft_full_d3 | 1.000x | 0.999x | 1.072x |
| fen_parse | 0.991x | 0.981x | 1.235x |
| fen_write_native_ep | 1.000x | 0.999x | 1.591x |
| snapshot | 1.002x | 0.998x | 0.950x |
| san_parse_all | 1.000x | 0.994x | 1.811x |
| san_render_native | 0.999x | 0.992x | 2.532x |
| apply_children_full | 0.993x | 0.967x | 1.171x |
| make_unmake_full | 1.014x | 0.998x | 1.500x |
| startpos | 1.001x | 1.001x | 1.305x |
| chess960_materialize | 0.999x | 0.995x | 1.381x |
| chess960_perft_d3 | 1.005x | 1.003x | 1.347x |
| polyglot_key | 0.998x | 0.990x | 25.693x |
| polyglot_recompute | 0.998x | 0.993x | 1.073x |
| repetition_key | 0.997x | 0.993x | 18.374x |
| corpus_count | 0.998x | 0.997x | 1.204x |
| corpus_materialize | 0.999x | 0.996x | 1.162x |
| corpus_uci_sorted | 0.995x | 0.988x | 1.447x |
| replay_trusted_native | 1.000x | 0.995x | 0.553x |
| replay_checked_native | 1.000x | 0.998x | 1.024x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.03x).
