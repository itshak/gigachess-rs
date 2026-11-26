| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.972x | 1.102x |
| materialize | 0.993x | 0.965x | 1.210x |
| perft_bulk_d4 | 0.994x | 0.949x | 1.100x |
| perft_full_d3 | 1.011x | 0.992x | 0.752x |
| fen_parse | 0.981x | 0.929x | 1.210x |
| fen_write_native_ep | 0.987x | 0.908x | 1.216x |
| snapshot | 1.011x | 0.977x | 1.001x |
| san_parse_all | 1.068x | 1.038x | 1.699x |
| san_render_native | 0.988x | 0.933x | 2.188x |
| apply_children_full | 1.015x | 0.978x | 0.819x |
| make_unmake_full | 1.005x | 0.973x | 1.109x |
| startpos | 1.019x | 1.019x | 1.283x |
| chess960_materialize | 0.872x | 0.759x | 1.169x |
| chess960_perft_d3 | 0.986x | 0.986x | 1.254x |
| polyglot_key | 0.999x | 0.992x | 31.726x |
| polyglot_recompute | 1.036x | 0.986x | 0.992x |
| repetition_key | 0.826x | 0.769x | 21.071x |
| corpus_count | 0.999x | 0.986x | 1.070x |
| corpus_materialize | 1.030x | 1.019x | 1.254x |
| corpus_uci_sorted | 1.043x | 1.036x | 1.008x |
| replay_trusted_native | 0.977x | 0.918x | 0.480x |
| replay_checked_native | 1.110x | 1.062x | 1.053x |

Cold first legal move list: 0.014 ms -> 0.012 ms (1.19x).
