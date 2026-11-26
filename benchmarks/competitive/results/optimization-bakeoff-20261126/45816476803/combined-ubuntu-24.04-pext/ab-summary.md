| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.996x | 1.295x |
| materialize | 1.002x | 0.989x | 1.203x |
| perft_bulk_d4 | 0.998x | 0.981x | 1.224x |
| perft_full_d3 | 1.025x | 0.987x | 1.215x |
| fen_parse | 1.145x | 1.107x | 1.208x |
| fen_write_native_ep | 0.989x | 0.975x | 1.721x |
| snapshot | 0.997x | 0.990x | 0.970x |
| san_parse_all | 1.002x | 0.989x | 2.102x |
| san_render_native | 0.999x | 0.992x | 2.202x |
| apply_children_full | 1.010x | 0.987x | 1.211x |
| make_unmake_full | 1.013x | 1.005x | 1.246x |
| startpos | 1.001x | 1.001x | 1.251x |
| chess960_materialize | 1.004x | 0.998x | 1.329x |
| chess960_perft_d3 | 0.983x | 0.983x | 1.378x |
| polyglot_key | 0.986x | 0.974x | 25.752x |
| polyglot_recompute | 1.001x | 1.000x | 1.030x |
| repetition_key | 0.985x | 0.970x | 18.581x |
| corpus_count | 1.003x | 1.002x | 1.303x |
| corpus_materialize | 1.007x | 1.006x | 1.130x |
| corpus_uci_sorted | 0.990x | 0.979x | 1.794x |
| replay_trusted_native | 1.000x | 0.991x | 0.803x |
| replay_checked_native | 0.994x | 0.981x | 1.026x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.99x).
