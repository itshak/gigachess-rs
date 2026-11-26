| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.015x | 0.975x | 1.339x |
| materialize | 0.987x | 0.827x | 1.164x |
| perft_bulk_d4 | 0.967x | 0.791x | 1.311x |
| perft_full_d3 | 1.003x | 0.956x | 0.724x |
| fen_parse | 1.078x | 1.050x | 1.283x |
| fen_write_native_ep | 0.998x | 0.985x | 1.019x |
| snapshot | 0.573x | 0.567x | 0.858x |
| san_parse_all | 1.025x | 1.015x | 1.844x |
| san_render_native | 1.016x | 0.956x | 3.597x |
| apply_children_full | 0.963x | 0.944x | 0.827x |
| make_unmake_full | 1.001x | 0.990x | 0.986x |
| startpos | 1.009x | 1.009x | 1.141x |
| chess960_materialize | 0.999x | 0.997x | 1.179x |
| chess960_perft_d3 | 0.997x | 0.980x | 1.322x |
| polyglot_key | 0.999x | 0.994x | 29.581x |
| polyglot_recompute | 1.002x | 0.999x | 1.021x |
| repetition_key | 0.989x | 0.980x | 20.006x |
| corpus_count | 1.010x | 0.996x | 1.361x |
| corpus_materialize | 1.008x | 1.005x | 1.162x |
| corpus_uci_sorted | 1.477x | 1.460x | 1.418x |
| replay_trusted_native | 0.997x | 0.990x | 0.513x |
| replay_checked_native | 1.000x | 0.990x | 0.890x |

Cold first legal move list: 0.028 ms -> 0.030 ms (0.91x).
