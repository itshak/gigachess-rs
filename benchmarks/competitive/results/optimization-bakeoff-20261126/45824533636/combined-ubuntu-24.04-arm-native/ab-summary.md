| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.380x |
| materialize | 1.002x | 0.996x | 1.247x |
| perft_bulk_d4 | 1.007x | 1.003x | 1.241x |
| perft_full_d3 | 1.005x | 0.995x | 1.068x |
| fen_parse | 1.010x | 1.005x | 1.246x |
| fen_write_native_ep | 0.999x | 0.994x | 1.566x |
| snapshot | 1.008x | 1.001x | 0.949x |
| san_parse_all | 1.003x | 0.997x | 1.809x |
| san_render_native | 1.002x | 0.987x | 2.525x |
| apply_children_full | 0.999x | 0.968x | 1.282x |
| make_unmake_full | 0.999x | 0.983x | 1.524x |
| startpos | 1.000x | 1.000x | 1.307x |
| chess960_materialize | 0.982x | 0.965x | 1.365x |
| chess960_perft_d3 | 0.992x | 0.991x | 1.334x |
| polyglot_key | 1.007x | 0.998x | 26.175x |
| polyglot_recompute | 1.003x | 1.003x | 1.072x |
| repetition_key | 0.994x | 0.989x | 18.416x |
| corpus_count | 1.028x | 1.015x | 1.200x |
| corpus_materialize | 1.008x | 1.004x | 1.147x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.450x |
| replay_trusted_native | 1.004x | 1.001x | 0.631x |
| replay_checked_native | 1.000x | 0.999x | 1.062x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.21x).
