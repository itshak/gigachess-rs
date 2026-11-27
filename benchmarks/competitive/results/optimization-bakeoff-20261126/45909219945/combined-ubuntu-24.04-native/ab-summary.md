| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.984x | 1.517x |
| materialize | 0.952x | 0.895x | 1.515x |
| perft_bulk_d4 | 1.047x | 1.025x | 1.220x |
| perft_full_d3 | 0.996x | 0.989x | 1.351x |
| fen_parse | 0.926x | 0.835x | 1.054x |
| fen_write_native_ep | 0.986x | 0.946x | 2.094x |
| snapshot | 0.997x | 0.991x | 1.152x |
| san_parse_all | 1.007x | 0.984x | 2.034x |
| san_render_native | 1.000x | 0.998x | 2.134x |
| apply_children_full | 0.998x | 0.996x | 1.335x |
| make_unmake_full | 0.998x | 0.985x | 1.202x |
| startpos | 1.004x | 1.004x | 1.117x |
| chess960_materialize | 0.992x | 0.990x | 1.547x |
| chess960_perft_d3 | 0.997x | 0.990x | 1.495x |
| polyglot_key | 1.025x | 0.994x | 39.344x |
| polyglot_recompute | 1.016x | 1.012x | 1.008x |
| repetition_key | 1.260x | 1.125x | 23.765x |
| corpus_count | 1.084x | 1.070x | 1.171x |
| corpus_materialize | 1.020x | 1.014x | 1.073x |
| corpus_uci_sorted | 0.998x | 0.995x | 1.552x |
| replay_trusted_native | 0.992x | 0.965x | 0.875x |
| replay_checked_native | 1.011x | 1.003x | 1.201x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.83x).
