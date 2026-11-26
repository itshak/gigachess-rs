| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.988x | 1.309x |
| materialize | 0.993x | 0.978x | 1.333x |
| perft_bulk_d4 | 0.996x | 0.977x | 1.298x |
| perft_full_d3 | 0.997x | 0.989x | 1.267x |
| fen_parse | 1.048x | 1.029x | 1.205x |
| fen_write_native_ep | 1.009x | 1.004x | 1.607x |
| snapshot | 1.001x | 1.000x | 1.002x |
| san_parse_all | 0.992x | 0.976x | 1.989x |
| san_render_native | 0.989x | 0.982x | 2.201x |
| apply_children_full | 0.995x | 0.989x | 1.404x |
| make_unmake_full | 0.982x | 0.955x | 1.404x |
| startpos | 0.999x | 0.999x | 1.337x |
| chess960_materialize | 0.973x | 0.944x | 1.459x |
| chess960_perft_d3 | 0.999x | 0.999x | 1.531x |
| polyglot_key | 0.941x | 0.893x | 21.949x |
| polyglot_recompute | 0.740x | 0.727x | 0.745x |
| repetition_key | 1.011x | 0.979x | 17.950x |
| corpus_count | 0.996x | 0.993x | 1.278x |
| corpus_materialize | 0.996x | 0.995x | 1.201x |
| corpus_uci_sorted | 0.998x | 0.989x | 1.822x |
| replay_trusted_native | 0.995x | 0.994x | 0.901x |
| replay_checked_native | 1.016x | 1.015x | 1.149x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.93x).
