| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.942x | 0.863x | 1.430x |
| materialize | 1.016x | 0.973x | 1.076x |
| perft_bulk_d4 | 0.986x | 0.855x | 1.118x |
| perft_full_d3 | 1.042x | 0.925x | 1.396x |
| fen_parse | 0.941x | 0.793x | 1.172x |
| fen_write_native_ep | 0.897x | 0.675x | 1.675x |
| snapshot | 0.944x | 0.592x | 0.946x |
| san_parse_all | 0.967x | 0.703x | 1.809x |
| san_render_native | 1.024x | 0.791x | 2.382x |
| apply_children_full | 0.974x | 0.850x | 1.070x |
| make_unmake_full | 1.017x | 0.749x | 1.268x |
| startpos | 0.989x | 0.989x | 1.239x |
| chess960_materialize | 1.032x | 0.828x | 1.704x |
| chess960_perft_d3 | 1.011x | 0.967x | 1.139x |
| polyglot_key | 1.022x | 0.985x | 35.257x |
| polyglot_recompute | 1.013x | 0.942x | 0.895x |
| repetition_key | 1.197x | 1.089x | 21.646x |
| corpus_count | 1.014x | 0.844x | 0.886x |
| corpus_materialize | 1.142x | 1.126x | 1.087x |
| corpus_uci_sorted | 1.493x | 1.484x | 1.401x |
| replay_trusted_native | 1.048x | 1.021x | 0.652x |
| replay_checked_native | 1.124x | 1.086x | 1.042x |

Cold first legal move list: 0.011 ms -> 0.006 ms (1.77x).
