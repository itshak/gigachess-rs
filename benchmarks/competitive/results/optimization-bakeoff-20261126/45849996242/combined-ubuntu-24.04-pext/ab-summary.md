| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 0.987x | 1.551x |
| materialize | 1.001x | 0.986x | 1.589x |
| perft_bulk_d4 | 1.011x | 1.002x | 1.190x |
| perft_full_d3 | 1.135x | 1.016x | 1.535x |
| fen_parse | 0.936x | 0.838x | 1.050x |
| fen_write_native_ep | 0.992x | 0.984x | 2.104x |
| snapshot | 1.003x | 0.995x | 0.997x |
| san_parse_all | 1.100x | 1.078x | 1.888x |
| san_render_native | 0.990x | 0.978x | 2.086x |
| apply_children_full | 1.001x | 0.986x | 1.313x |
| make_unmake_full | 1.009x | 0.993x | 1.172x |
| startpos | 0.896x | 0.896x | 0.988x |
| chess960_materialize | 1.009x | 0.991x | 1.570x |
| chess960_perft_d3 | 0.998x | 0.994x | 1.471x |
| polyglot_key | 0.995x | 0.990x | 38.973x |
| polyglot_recompute | 1.004x | 0.992x | 0.958x |
| repetition_key | 0.817x | 0.696x | 19.284x |
| corpus_count | 1.070x | 1.025x | 1.131x |
| corpus_materialize | 1.008x | 1.005x | 1.086x |
| corpus_uci_sorted | 0.998x | 0.996x | 1.551x |
| replay_trusted_native | 0.969x | 0.931x | 0.882x |
| replay_checked_native | 1.013x | 1.013x | 1.122x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.89x).
