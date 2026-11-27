| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.982x | 1.501x |
| materialize | 1.002x | 0.997x | 1.587x |
| perft_bulk_d4 | 1.003x | 0.998x | 1.185x |
| perft_full_d3 | 0.999x | 0.997x | 1.343x |
| fen_parse | 0.983x | 0.960x | 1.107x |
| fen_write_native_ep | 0.998x | 0.990x | 2.067x |
| snapshot | 1.000x | 0.994x | 1.330x |
| san_parse_all | 1.001x | 0.986x | 1.890x |
| san_render_native | 1.000x | 0.999x | 2.140x |
| apply_children_full | 0.998x | 0.993x | 1.260x |
| make_unmake_full | 0.998x | 0.989x | 1.203x |
| startpos | 1.002x | 1.002x | 1.256x |
| chess960_materialize | 1.002x | 1.001x | 1.533x |
| chess960_perft_d3 | 1.003x | 0.994x | 1.515x |
| polyglot_key | 1.000x | 0.997x | 39.964x |
| polyglot_recompute | 0.998x | 0.996x | 1.000x |
| repetition_key | 0.994x | 0.989x | 19.753x |
| corpus_count | 1.003x | 0.997x | 1.081x |
| corpus_materialize | 0.998x | 0.997x | 1.045x |
| corpus_uci_sorted | 0.998x | 0.996x | 1.560x |
| replay_trusted_native | 1.036x | 0.996x | 0.886x |
| replay_checked_native | 1.012x | 0.984x | 1.136x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.21x).
