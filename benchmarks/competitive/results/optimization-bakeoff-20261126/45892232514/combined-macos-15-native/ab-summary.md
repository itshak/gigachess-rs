| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.914x | 0.660x | 1.525x |
| materialize | 0.981x | 0.822x | 1.099x |
| perft_bulk_d4 | 0.757x | 0.657x | 0.998x |
| perft_full_d3 | 0.688x | 0.609x | 0.953x |
| fen_parse | 0.891x | 0.750x | 1.190x |
| fen_write_native_ep | 0.888x | 0.710x | 1.486x |
| snapshot | 1.088x | 0.783x | 0.935x |
| san_parse_all | 0.992x | 0.886x | 1.796x |
| san_render_native | 1.063x | 0.778x | 3.122x |
| apply_children_full | 0.834x | 0.656x | 1.035x |
| make_unmake_full | 0.762x | 0.646x | 1.028x |
| startpos | 2.068x | 2.068x | 2.432x |
| chess960_materialize | 1.022x | 0.961x | 1.527x |
| chess960_perft_d3 | 0.942x | 0.866x | 1.371x |
| polyglot_key | 1.253x | 1.243x | 41.867x |
| polyglot_recompute | 1.219x | 1.113x | 0.952x |
| repetition_key | 1.189x | 1.163x | 21.133x |
| corpus_count | 0.930x | 0.751x | 1.261x |
| corpus_materialize | 1.050x | 0.931x | 0.952x |
| corpus_uci_sorted | 1.011x | 0.915x | 1.295x |
| replay_trusted_native | 1.065x | 0.991x | 0.614x |
| replay_checked_native | 0.963x | 0.723x | 1.077x |

Cold first legal move list: 0.006 ms -> 0.010 ms (0.64x).
