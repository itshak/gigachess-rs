| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.984x | 1.405x |
| materialize | 1.044x | 1.011x | 1.478x |
| perft_bulk_d4 | 1.002x | 0.992x | 1.208x |
| perft_full_d3 | 0.999x | 0.994x | 1.507x |
| fen_parse | 1.019x | 1.004x | 1.103x |
| fen_write_native_ep | 0.995x | 0.969x | 1.320x |
| snapshot | 1.415x | 1.407x | 1.151x |
| san_parse_all | 0.998x | 0.986x | 1.877x |
| san_render_native | 0.991x | 0.989x | 2.120x |
| apply_children_full | 1.009x | 1.004x | 1.118x |
| make_unmake_full | 1.003x | 0.995x | 1.152x |
| startpos | 1.094x | 1.094x | 1.124x |
| chess960_materialize | 1.004x | 1.002x | 1.473x |
| chess960_perft_d3 | 0.998x | 0.992x | 1.385x |
| polyglot_key | 1.001x | 0.996x | 39.621x |
| polyglot_recompute | 1.039x | 1.026x | 1.012x |
| repetition_key | 1.088x | 0.930x | 21.179x |
| corpus_count | 1.002x | 1.000x | 1.339x |
| corpus_materialize | 1.012x | 1.008x | 1.089x |
| corpus_uci_sorted | 0.982x | 0.978x | 1.560x |
| replay_trusted_native | 0.998x | 0.991x | 0.705x |
| replay_checked_native | 0.999x | 0.996x | 1.129x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.94x).
