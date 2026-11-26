| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 0.998x | 1.358x |
| materialize | 1.051x | 1.004x | 1.073x |
| perft_bulk_d4 | 1.061x | 1.011x | 1.337x |
| perft_full_d3 | 0.994x | 0.955x | 1.039x |
| fen_parse | 1.083x | 1.048x | 1.305x |
| fen_write_native_ep | 0.999x | 0.986x | 1.591x |
| snapshot | 0.993x | 0.976x | 1.212x |
| san_parse_all | 1.084x | 1.003x | 1.939x |
| san_render_native | 0.995x | 0.978x | 3.361x |
| apply_children_full | 0.967x | 0.946x | 1.026x |
| make_unmake_full | 1.021x | 0.951x | 1.226x |
| startpos | 0.875x | 0.875x | 0.876x |
| chess960_materialize | 0.996x | 0.992x | 1.249x |
| chess960_perft_d3 | 0.992x | 0.984x | 1.397x |
| polyglot_key | 1.009x | 0.992x | 20.379x |
| polyglot_recompute | 1.005x | 0.996x | 0.986x |
| repetition_key | 1.068x | 1.009x | 17.897x |
| corpus_count | 1.002x | 1.000x | 1.314x |
| corpus_materialize | 1.001x | 0.998x | 1.166x |
| corpus_uci_sorted | 0.995x | 0.994x | 1.479x |
| replay_trusted_native | 0.970x | 0.954x | 0.711x |
| replay_checked_native | 0.998x | 0.993x | 0.963x |

Cold first legal move list: 0.028 ms -> 0.022 ms (1.31x).
