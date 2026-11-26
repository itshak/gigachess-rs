| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.378x |
| materialize | 0.997x | 0.995x | 1.238x |
| perft_bulk_d4 | 1.000x | 0.999x | 1.237x |
| perft_full_d3 | 0.998x | 0.988x | 1.067x |
| fen_parse | 1.000x | 0.998x | 1.229x |
| fen_write_native_ep | 1.000x | 0.996x | 1.634x |
| snapshot | 1.016x | 1.011x | 0.935x |
| san_parse_all | 0.997x | 0.995x | 1.831x |
| san_render_native | 0.999x | 0.996x | 2.524x |
| apply_children_full | 1.006x | 0.997x | 1.296x |
| make_unmake_full | 0.999x | 0.994x | 1.502x |
| startpos | 1.000x | 1.000x | 1.277x |
| chess960_materialize | 0.997x | 0.996x | 1.377x |
| chess960_perft_d3 | 1.000x | 0.998x | 1.352x |
| polyglot_key | 0.982x | 0.968x | 25.471x |
| polyglot_recompute | 1.000x | 1.000x | 1.066x |
| repetition_key | 0.998x | 0.995x | 18.312x |
| corpus_count | 0.993x | 0.988x | 1.220x |
| corpus_materialize | 0.996x | 0.994x | 1.165x |
| corpus_uci_sorted | 1.004x | 0.999x | 1.456x |
| replay_trusted_native | 1.001x | 1.000x | 0.607x |
| replay_checked_native | 1.000x | 1.000x | 1.068x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).
