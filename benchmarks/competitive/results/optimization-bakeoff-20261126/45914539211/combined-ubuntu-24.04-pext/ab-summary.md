| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.058x | 1.028x | 1.565x |
| materialize | 1.013x | 0.982x | 1.284x |
| perft_bulk_d4 | 1.038x | 0.974x | 1.444x |
| perft_full_d3 | 1.224x | 1.024x | 1.570x |
| fen_parse | 0.948x | 0.919x | 1.135x |
| fen_write_native_ep | 1.011x | 1.000x | 1.726x |
| snapshot | 1.002x | 0.990x | 1.247x |
| san_parse_all | 1.003x | 0.988x | 2.047x |
| san_render_native | 0.995x | 0.986x | 2.196x |
| apply_children_full | 1.039x | 1.002x | 1.378x |
| make_unmake_full | 1.000x | 0.979x | 1.267x |
| startpos | 0.998x | 0.998x | 1.250x |
| chess960_materialize | 1.017x | 1.003x | 1.389x |
| chess960_perft_d3 | 0.990x | 0.976x | 1.480x |
| polyglot_key | 1.052x | 1.038x | 25.623x |
| polyglot_recompute | 0.994x | 0.988x | 1.034x |
| repetition_key | 0.989x | 0.983x | 19.484x |
| corpus_count | 1.072x | 1.043x | 1.546x |
| corpus_materialize | 0.994x | 0.988x | 1.177x |
| corpus_uci_sorted | 0.989x | 0.973x | 1.795x |
| replay_trusted_native | 0.963x | 0.951x | 0.866x |
| replay_checked_native | 0.965x | 0.964x | 1.091x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.02x).
