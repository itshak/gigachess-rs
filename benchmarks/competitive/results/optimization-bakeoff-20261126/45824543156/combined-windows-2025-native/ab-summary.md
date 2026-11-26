| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.980x | 0.921x | 1.317x |
| materialize | 0.966x | 0.936x | 1.167x |
| perft_bulk_d4 | 0.967x | 0.944x | 1.225x |
| perft_full_d3 | 1.004x | 0.935x | 1.266x |
| fen_parse | 0.842x | 0.793x | 1.155x |
| fen_write_native_ep | 0.992x | 0.961x | 1.546x |
| snapshot | 0.974x | 0.937x | 1.486x |
| san_parse_all | 0.974x | 0.912x | 1.937x |
| san_render_native | 1.157x | 0.997x | 3.589x |
| apply_children_full | 1.150x | 1.066x | 1.341x |
| make_unmake_full | 0.958x | 0.856x | 1.274x |
| startpos | 0.343x | 0.343x | 0.444x |
| chess960_materialize | 0.996x | 0.992x | 1.296x |
| chess960_perft_d3 | 0.992x | 0.992x | 1.436x |
| polyglot_key | 1.059x | 1.024x | 37.253x |
| polyglot_recompute | 1.010x | 1.005x | 1.004x |
| repetition_key | 1.241x | 1.174x | 29.544x |
| corpus_count | 0.978x | 0.977x | 1.066x |
| corpus_materialize | 0.954x | 0.914x | 0.994x |
| corpus_uci_sorted | 0.983x | 0.977x | 1.384x |
| replay_trusted_native | 0.997x | 0.984x | 0.723x |
| replay_checked_native | 0.991x | 0.989x | 1.113x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.03x).
