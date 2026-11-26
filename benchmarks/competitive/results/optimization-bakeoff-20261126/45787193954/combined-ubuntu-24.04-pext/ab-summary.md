| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.993x | 1.470x |
| materialize | 1.074x | 1.019x | 1.561x |
| perft_bulk_d4 | 0.989x | 0.976x | 1.258x |
| perft_full_d3 | 1.277x | 1.101x | 1.494x |
| fen_parse | 0.973x | 0.894x | 1.069x |
| fen_write_native_ep | 1.008x | 0.991x | 1.930x |
| snapshot | 1.289x | 1.286x | 1.153x |
| san_parse_all | 1.102x | 1.043x | 1.970x |
| san_render_native | 1.019x | 1.015x | 2.116x |
| apply_children_full | 1.050x | 1.024x | 1.101x |
| make_unmake_full | 1.005x | 0.997x | 1.162x |
| startpos | 1.097x | 1.097x | 1.119x |
| chess960_materialize | 1.074x | 1.067x | 1.417x |
| chess960_perft_d3 | 1.008x | 1.000x | 1.385x |
| polyglot_key | 1.015x | 1.001x | 39.740x |
| polyglot_recompute | 1.000x | 0.999x | 1.012x |
| repetition_key | 0.870x | 0.788x | 16.565x |
| corpus_count | 1.002x | 1.001x | 1.414x |
| corpus_materialize | 1.013x | 1.011x | 1.126x |
| corpus_uci_sorted | 0.997x | 0.995x | 1.574x |
| replay_trusted_native | 1.008x | 0.995x | 0.726x |
| replay_checked_native | 1.017x | 1.014x | 1.041x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.08x).
