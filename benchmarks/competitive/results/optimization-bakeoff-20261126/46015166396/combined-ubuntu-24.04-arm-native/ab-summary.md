| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.984x | 1.568x |
| materialize | 1.017x | 1.005x | 1.340x |
| perft_bulk_d4 | 1.039x | 1.016x | 1.453x |
| perft_full_d3 | 1.133x | 1.008x | 1.509x |
| fen_parse | 1.017x | 0.943x | 1.240x |
| fen_write_native_ep | 0.994x | 0.987x | 1.564x |
| snapshot | 0.997x | 0.991x | 0.946x |
| san_parse_all | 0.996x | 0.990x | 1.822x |
| san_render_native | 0.998x | 0.992x | 2.664x |
| apply_children_full | 0.996x | 0.973x | 1.156x |
| make_unmake_full | 1.018x | 1.007x | 1.581x |
| startpos | 0.999x | 0.999x | 1.304x |
| chess960_materialize | 1.008x | 1.004x | 1.507x |
| chess960_perft_d3 | 1.020x | 1.012x | 1.518x |
| polyglot_key | 0.984x | 0.975x | 25.264x |
| polyglot_recompute | 1.305x | 1.212x | 1.414x |
| repetition_key | 0.999x | 0.997x | 19.278x |
| corpus_count | 1.105x | 1.080x | 1.501x |
| corpus_materialize | 1.010x | 1.005x | 1.232x |
| corpus_uci_sorted | 1.005x | 1.004x | 1.463x |
| replay_trusted_native | 1.002x | 0.996x | 0.625x |
| replay_checked_native | 1.018x | 1.015x | 1.152x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.88x).
