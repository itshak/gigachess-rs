| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 0.997x | 1.573x |
| materialize | 1.003x | 0.992x | 1.342x |
| perft_bulk_d4 | 1.013x | 1.004x | 1.415x |
| perft_full_d3 | 1.142x | 1.026x | 1.525x |
| fen_parse | 0.986x | 0.930x | 1.209x |
| fen_write_native_ep | 0.999x | 0.996x | 1.592x |
| snapshot | 1.009x | 1.005x | 0.949x |
| san_parse_all | 1.001x | 0.996x | 1.835x |
| san_render_native | 1.007x | 1.001x | 2.686x |
| apply_children_full | 1.004x | 0.998x | 1.158x |
| make_unmake_full | 0.998x | 0.984x | 1.554x |
| startpos | 1.000x | 1.000x | 1.308x |
| chess960_materialize | 1.003x | 0.998x | 1.495x |
| chess960_perft_d3 | 1.001x | 0.999x | 1.502x |
| polyglot_key | 0.999x | 0.995x | 25.449x |
| polyglot_recompute | 0.994x | 0.919x | 1.085x |
| repetition_key | 0.988x | 0.977x | 18.853x |
| corpus_count | 1.005x | 1.001x | 1.386x |
| corpus_materialize | 0.998x | 0.995x | 1.249x |
| corpus_uci_sorted | 1.000x | 1.000x | 1.461x |
| replay_trusted_native | 1.007x | 1.007x | 0.627x |
| replay_checked_native | 0.994x | 0.992x | 1.124x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.02x).
