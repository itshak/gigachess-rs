| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.945x | 1.204x |
| materialize | 1.024x | 0.950x | 1.305x |
| perft_bulk_d4 | 0.925x | 0.883x | 1.122x |
| perft_full_d3 | 0.995x | 0.942x | 1.320x |
| fen_parse | 1.026x | 0.998x | 1.214x |
| fen_write_native_ep | 1.008x | 0.994x | 1.709x |
| snapshot | 0.998x | 0.997x | 0.977x |
| san_parse_all | 0.987x | 0.979x | 2.048x |
| san_render_native | 0.982x | 0.966x | 2.294x |
| apply_children_full | 0.976x | 0.970x | 1.148x |
| make_unmake_full | 0.998x | 0.982x | 1.334x |
| startpos | 1.001x | 1.001x | 1.334x |
| chess960_materialize | 1.042x | 1.009x | 1.342x |
| chess960_perft_d3 | 1.036x | 0.978x | 1.428x |
| polyglot_key | 1.077x | 1.043x | 24.006x |
| polyglot_recompute | 1.012x | 1.009x | 1.018x |
| repetition_key | 0.991x | 0.948x | 17.889x |
| corpus_count | 0.923x | 0.850x | 1.200x |
| corpus_materialize | 0.936x | 0.906x | 1.117x |
| corpus_uci_sorted | 1.006x | 0.994x | 1.806x |
| replay_trusted_native | 0.969x | 0.955x | 0.683x |
| replay_checked_native | 1.022x | 1.017x | 1.069x |

Cold first legal move list: 0.017 ms -> 0.008 ms (2.15x).
