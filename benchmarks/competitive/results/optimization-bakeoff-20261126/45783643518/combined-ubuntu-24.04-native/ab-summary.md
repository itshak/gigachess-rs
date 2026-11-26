| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.994x | 0.961x | 1.287x |
| materialize | 1.002x | 0.925x | 1.159x |
| perft_bulk_d4 | 1.011x | 0.984x | 1.285x |
| perft_full_d3 | 1.095x | 1.061x | 1.231x |
| fen_parse | 1.072x | 1.056x | 1.083x |
| fen_write_native_ep | 1.011x | 0.993x | 0.944x |
| snapshot | 1.762x | 1.739x | 1.249x |
| san_parse_all | 1.020x | 1.009x | 1.995x |
| san_render_native | 1.012x | 0.998x | 2.310x |
| apply_children_full | 0.992x | 0.979x | 1.109x |
| make_unmake_full | 0.973x | 0.955x | 1.169x |
| startpos | 1.158x | 1.158x | 1.242x |
| chess960_materialize | 1.014x | 1.003x | 1.258x |
| chess960_perft_d3 | 0.988x | 0.985x | 1.330x |
| polyglot_key | 0.992x | 0.960x | 28.547x |
| polyglot_recompute | 0.976x | 0.971x | 1.036x |
| repetition_key | 0.869x | 0.852x | 19.730x |
| corpus_count | 0.989x | 0.976x | 1.360x |
| corpus_materialize | 1.011x | 1.010x | 1.199x |
| corpus_uci_sorted | 1.021x | 1.020x | 1.737x |
| replay_trusted_native | 1.004x | 0.997x | 0.691x |
| replay_checked_native | 0.998x | 0.984x | 1.014x |

Cold first legal move list: 0.014 ms -> 0.013 ms (1.08x).
