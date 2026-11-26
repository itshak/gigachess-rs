| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.985x | 1.174x |
| materialize | 0.793x | 0.694x | 1.341x |
| perft_bulk_d4 | 0.999x | 0.994x | 1.158x |
| perft_full_d3 | 0.986x | 0.942x | 1.470x |
| fen_parse | 0.997x | 0.890x | 1.060x |
| fen_write_native_ep | 0.994x | 0.963x | 2.104x |
| snapshot | 1.115x | 1.106x | 1.005x |
| san_parse_all | 1.109x | 1.068x | 1.902x |
| san_render_native | 1.018x | 0.987x | 2.214x |
| apply_children_full | 0.991x | 0.970x | 1.073x |
| make_unmake_full | 1.000x | 0.995x | 1.251x |
| startpos | 1.001x | 1.001x | 1.288x |
| chess960_materialize | 0.938x | 0.837x | 1.237x |
| chess960_perft_d3 | 0.994x | 0.989x | 1.338x |
| polyglot_key | 1.072x | 1.007x | 40.257x |
| polyglot_recompute | 1.012x | 0.998x | 1.013x |
| repetition_key | 1.195x | 1.107x | 24.949x |
| corpus_count | 1.007x | 0.997x | 1.278x |
| corpus_materialize | 0.961x | 0.957x | 1.103x |
| corpus_uci_sorted | 0.963x | 0.954x | 1.516x |
| replay_trusted_native | 1.000x | 0.998x | 0.686x |
| replay_checked_native | 0.997x | 0.991x | 1.094x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.96x).
