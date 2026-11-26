| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.030x | 0.980x | 1.212x |
| materialize | 0.987x | 0.958x | 1.659x |
| perft_bulk_d4 | 0.920x | 0.870x | 1.067x |
| perft_full_d3 | 0.967x | 0.896x | 1.449x |
| fen_parse | 0.934x | 0.838x | 1.055x |
| fen_write_native_ep | 0.995x | 0.972x | 2.051x |
| snapshot | 1.254x | 1.247x | 0.999x |
| san_parse_all | 0.985x | 0.946x | 1.908x |
| san_render_native | 0.983x | 0.965x | 2.126x |
| apply_children_full | 1.016x | 0.995x | 1.116x |
| make_unmake_full | 1.010x | 0.986x | 1.249x |
| startpos | 1.000x | 1.000x | 1.356x |
| chess960_materialize | 1.027x | 0.982x | 1.317x |
| chess960_perft_d3 | 1.043x | 0.979x | 1.396x |
| polyglot_key | 1.063x | 1.004x | 43.702x |
| polyglot_recompute | 0.972x | 0.970x | 1.059x |
| repetition_key | 0.949x | 0.753x | 20.668x |
| corpus_count | 0.815x | 0.790x | 1.024x |
| corpus_materialize | 0.945x | 0.931x | 1.106x |
| corpus_uci_sorted | 0.965x | 0.964x | 1.500x |
| replay_trusted_native | 1.021x | 1.019x | 0.710x |
| replay_checked_native | 1.021x | 1.016x | 1.120x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.53x).
