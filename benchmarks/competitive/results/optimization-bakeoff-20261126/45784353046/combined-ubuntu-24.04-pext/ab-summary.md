| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 1.000x | 1.480x |
| materialize | 1.004x | 0.993x | 1.581x |
| perft_bulk_d4 | 1.008x | 1.001x | 1.265x |
| perft_full_d3 | 1.025x | 1.020x | 1.557x |
| fen_parse | 1.071x | 1.042x | 1.063x |
| fen_write_native_ep | 0.965x | 0.909x | 2.059x |
| snapshot | 0.849x | 0.848x | 1.000x |
| san_parse_all | 1.043x | 1.023x | 1.976x |
| san_render_native | 0.963x | 0.955x | 2.077x |
| apply_children_full | 0.955x | 0.933x | 1.091x |
| make_unmake_full | 1.002x | 0.997x | 1.147x |
| startpos | 0.913x | 0.913x | 1.167x |
| chess960_materialize | 0.934x | 0.933x | 1.378x |
| chess960_perft_d3 | 0.995x | 0.995x | 1.386x |
| polyglot_key | 0.985x | 0.972x | 40.854x |
| polyglot_recompute | 0.959x | 0.939x | 1.006x |
| repetition_key | 1.130x | 0.990x | 22.729x |
| corpus_count | 1.003x | 1.002x | 1.425x |
| corpus_materialize | 0.987x | 0.985x | 1.098x |
| corpus_uci_sorted | 0.985x | 0.978x | 1.559x |
| replay_trusted_native | 0.987x | 0.973x | 0.722x |
| replay_checked_native | 0.998x | 0.992x | 1.105x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.06x).
