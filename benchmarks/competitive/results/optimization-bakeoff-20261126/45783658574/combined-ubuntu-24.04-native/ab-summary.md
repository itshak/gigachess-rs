| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 1.000x | 1.412x |
| materialize | 1.002x | 0.998x | 1.554x |
| perft_bulk_d4 | 0.996x | 0.980x | 1.209x |
| perft_full_d3 | 0.992x | 0.980x | 1.494x |
| fen_parse | 1.149x | 1.116x | 1.163x |
| fen_write_native_ep | 0.980x | 0.966x | 1.352x |
| snapshot | 0.713x | 0.699x | 1.162x |
| san_parse_all | 0.995x | 0.981x | 1.901x |
| san_render_native | 1.009x | 1.002x | 2.157x |
| apply_children_full | 0.995x | 0.986x | 1.409x |
| make_unmake_full | 0.995x | 0.973x | 1.141x |
| startpos | 0.909x | 0.909x | 1.287x |
| chess960_materialize | 1.058x | 1.056x | 1.355x |
| chess960_perft_d3 | 1.000x | 1.000x | 1.386x |
| polyglot_key | 1.000x | 0.997x | 41.118x |
| polyglot_recompute | 1.038x | 1.028x | 1.043x |
| repetition_key | 1.217x | 1.126x | 24.169x |
| corpus_count | 1.002x | 1.002x | 1.338x |
| corpus_materialize | 1.011x | 1.011x | 1.098x |
| corpus_uci_sorted | 1.008x | 0.994x | 1.564x |
| replay_trusted_native | 0.999x | 0.996x | 0.722x |
| replay_checked_native | 1.002x | 0.995x | 1.044x |

Cold first legal move list: 0.009 ms -> 0.010 ms (0.95x).
