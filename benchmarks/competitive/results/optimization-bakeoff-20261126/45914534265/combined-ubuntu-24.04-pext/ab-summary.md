| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.998x | 1.561x |
| materialize | 0.978x | 0.935x | 1.501x |
| perft_bulk_d4 | 1.014x | 1.001x | 1.202x |
| perft_full_d3 | 0.994x | 0.989x | 1.376x |
| fen_parse | 1.013x | 0.999x | 1.072x |
| fen_write_native_ep | 1.003x | 0.967x | 2.128x |
| snapshot | 0.997x | 0.987x | 1.150x |
| san_parse_all | 1.016x | 0.992x | 1.999x |
| san_render_native | 0.827x | 0.810x | 1.786x |
| apply_children_full | 0.998x | 0.992x | 1.338x |
| make_unmake_full | 0.967x | 0.907x | 1.160x |
| startpos | 0.894x | 0.894x | 1.118x |
| chess960_materialize | 1.003x | 0.988x | 1.412x |
| chess960_perft_d3 | 1.002x | 1.002x | 1.478x |
| polyglot_key | 1.086x | 1.001x | 39.751x |
| polyglot_recompute | 1.014x | 1.011x | 1.016x |
| repetition_key | 0.861x | 0.693x | 19.410x |
| corpus_count | 0.998x | 0.995x | 1.130x |
| corpus_materialize | 1.017x | 1.015x | 1.091x |
| corpus_uci_sorted | 1.005x | 1.003x | 1.553x |
| replay_trusted_native | 1.023x | 1.005x | 0.925x |
| replay_checked_native | 0.986x | 0.984x | 1.117x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.09x).
