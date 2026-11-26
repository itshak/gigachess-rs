| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.362x |
| materialize | 0.998x | 0.996x | 1.253x |
| perft_bulk_d4 | 1.002x | 1.000x | 1.356x |
| perft_full_d3 | 1.006x | 1.002x | 1.075x |
| fen_parse | 1.005x | 0.998x | 1.236x |
| fen_write_native_ep | 1.002x | 0.998x | 1.580x |
| snapshot | 1.030x | 1.027x | 0.914x |
| san_parse_all | 0.998x | 0.994x | 1.831x |
| san_render_native | 1.002x | 0.990x | 2.554x |
| apply_children_full | 1.077x | 1.044x | 1.127x |
| make_unmake_full | 1.001x | 0.997x | 1.437x |
| startpos | 1.033x | 1.033x | 1.185x |
| chess960_materialize | 0.997x | 0.993x | 1.320x |
| chess960_perft_d3 | 0.998x | 0.993x | 1.320x |
| polyglot_key | 1.007x | 0.990x | 25.682x |
| polyglot_recompute | 0.998x | 0.998x | 1.067x |
| repetition_key | 0.996x | 0.968x | 18.912x |
| corpus_count | 1.022x | 1.012x | 1.461x |
| corpus_materialize | 1.015x | 1.015x | 1.221x |
| corpus_uci_sorted | 0.998x | 0.997x | 1.462x |
| replay_trusted_native | 1.003x | 0.997x | 0.507x |
| replay_checked_native | 0.992x | 0.982x | 0.983x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.12x).
