| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.979x | 1.565x |
| materialize | 1.012x | 0.994x | 1.345x |
| perft_bulk_d4 | 1.043x | 1.019x | 1.457x |
| perft_full_d3 | 1.136x | 1.013x | 1.516x |
| fen_parse | 0.985x | 0.976x | 1.197x |
| fen_write_native_ep | 0.997x | 0.995x | 1.616x |
| snapshot | 1.011x | 1.007x | 0.936x |
| san_parse_all | 0.997x | 0.992x | 1.824x |
| san_render_native | 0.992x | 0.975x | 2.671x |
| apply_children_full | 1.002x | 0.979x | 1.165x |
| make_unmake_full | 1.015x | 0.996x | 1.569x |
| startpos | 0.999x | 0.999x | 1.276x |
| chess960_materialize | 1.006x | 1.002x | 1.497x |
| chess960_perft_d3 | 1.017x | 1.004x | 1.522x |
| polyglot_key | 0.988x | 0.974x | 25.544x |
| polyglot_recompute | 1.005x | 0.998x | 1.089x |
| repetition_key | 0.991x | 0.983x | 19.406x |
| corpus_count | 1.092x | 1.063x | 1.495x |
| corpus_materialize | 1.004x | 1.000x | 1.242x |
| corpus_uci_sorted | 0.982x | 0.982x | 1.438x |
| replay_trusted_native | 0.986x | 0.985x | 0.617x |
| replay_checked_native | 1.018x | 1.016x | 1.157x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.89x).
