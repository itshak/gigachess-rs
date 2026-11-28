| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.057x | 1.005x | 1.370x |
| materialize | 0.918x | 0.884x | 1.670x |
| perft_bulk_d4 | 1.069x | 1.040x | 1.260x |
| perft_full_d3 | 1.170x | 1.043x | 1.541x |
| fen_parse | 1.030x | 0.958x | 1.174x |
| fen_write_native_ep | 1.004x | 0.966x | 2.084x |
| snapshot | 0.999x | 0.993x | 1.002x |
| san_parse_all | 1.003x | 0.983x | 1.909x |
| san_render_native | 0.998x | 0.978x | 2.123x |
| apply_children_full | 1.008x | 1.001x | 1.361x |
| make_unmake_full | 1.015x | 1.005x | 1.302x |
| startpos | 1.004x | 1.004x | 1.277x |
| chess960_materialize | 0.997x | 0.989x | 1.592x |
| chess960_perft_d3 | 1.015x | 0.999x | 1.515x |
| polyglot_key | 1.008x | 1.001x | 42.898x |
| polyglot_recompute | 1.604x | 1.266x | 1.644x |
| repetition_key | 0.943x | 0.904x | 22.638x |
| corpus_count | 1.151x | 1.113x | 1.366x |
| corpus_materialize | 1.009x | 1.008x | 1.148x |
| corpus_uci_sorted | 0.996x | 0.990x | 1.548x |
| replay_trusted_native | 0.991x | 0.979x | 0.882x |
| replay_checked_native | 1.005x | 1.002x | 1.190x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.93x).
