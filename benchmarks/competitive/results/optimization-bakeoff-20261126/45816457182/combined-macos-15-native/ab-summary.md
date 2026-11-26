| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.953x | 1.283x |
| materialize | 1.022x | 0.978x | 1.030x |
| perft_bulk_d4 | 1.020x | 0.992x | 1.132x |
| perft_full_d3 | 0.994x | 0.907x | 1.301x |
| fen_parse | 1.015x | 0.994x | 1.227x |
| fen_write_native_ep | 1.005x | 0.903x | 1.716x |
| snapshot | 1.023x | 0.948x | 1.011x |
| san_parse_all | 1.007x | 0.872x | 1.776x |
| san_render_native | 1.079x | 0.926x | 2.753x |
| apply_children_full | 0.988x | 0.962x | 1.037x |
| make_unmake_full | 1.015x | 0.963x | 1.190x |
| startpos | 1.026x | 1.026x | 1.198x |
| chess960_materialize | 1.035x | 1.034x | 1.451x |
| chess960_perft_d3 | 1.020x | 1.012x | 1.235x |
| polyglot_key | 1.042x | 1.033x | 38.364x |
| polyglot_recompute | 1.008x | 1.002x | 0.972x |
| repetition_key | 1.050x | 1.016x | 18.966x |
| corpus_count | 1.059x | 1.001x | 0.998x |
| corpus_materialize | 1.050x | 1.016x | 1.115x |
| corpus_uci_sorted | 1.060x | 1.022x | 1.447x |
| replay_trusted_native | 1.017x | 1.011x | 0.599x |
| replay_checked_native | 1.001x | 0.976x | 1.085x |

Cold first legal move list: 0.003 ms -> 0.004 ms (0.98x).
