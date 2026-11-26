| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.998x | 1.358x |
| materialize | 1.004x | 1.000x | 1.232x |
| perft_bulk_d4 | 1.000x | 0.996x | 1.351x |
| perft_full_d3 | 0.995x | 0.988x | 1.049x |
| fen_parse | 0.997x | 0.995x | 1.234x |
| fen_write_native_ep | 0.986x | 0.977x | 1.025x |
| snapshot | 0.996x | 0.991x | 0.990x |
| san_parse_all | 1.000x | 0.995x | 1.826x |
| san_render_native | 0.998x | 0.992x | 2.557x |
| apply_children_full | 1.002x | 0.994x | 1.028x |
| make_unmake_full | 1.007x | 0.999x | 1.451x |
| startpos | 0.932x | 0.932x | 1.160x |
| chess960_materialize | 1.007x | 1.004x | 1.329x |
| chess960_perft_d3 | 1.003x | 0.997x | 1.321x |
| polyglot_key | 1.011x | 0.997x | 25.888x |
| polyglot_recompute | 0.999x | 0.998x | 1.073x |
| repetition_key | 1.013x | 0.999x | 19.274x |
| corpus_count | 0.983x | 0.979x | 1.448x |
| corpus_materialize | 1.006x | 1.005x | 1.195x |
| corpus_uci_sorted | 1.003x | 1.000x | 1.467x |
| replay_trusted_native | 1.003x | 0.999x | 0.500x |
| replay_checked_native | 0.998x | 0.993x | 0.989x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.05x).
