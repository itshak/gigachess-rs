| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.977x | 1.283x |
| materialize | 1.005x | 0.986x | 1.165x |
| perft_bulk_d4 | 0.997x | 0.984x | 1.179x |
| perft_full_d3 | 1.035x | 0.927x | 1.212x |
| fen_parse | 1.009x | 0.986x | 1.167x |
| fen_write_native_ep | 1.012x | 0.982x | 1.832x |
| snapshot | 1.003x | 0.998x | 1.287x |
| san_parse_all | 0.997x | 0.977x | 2.034x |
| san_render_native | 1.008x | 1.000x | 2.204x |
| apply_children_full | 0.993x | 0.970x | 1.288x |
| make_unmake_full | 1.032x | 1.008x | 1.297x |
| startpos | 0.999x | 0.999x | 1.251x |
| chess960_materialize | 1.000x | 0.986x | 1.316x |
| chess960_perft_d3 | 0.996x | 0.993x | 1.398x |
| polyglot_key | 1.031x | 1.005x | 26.617x |
| polyglot_recompute | 1.005x | 1.003x | 1.060x |
| repetition_key | 1.064x | 1.028x | 19.853x |
| corpus_count | 1.004x | 0.985x | 1.237x |
| corpus_materialize | 0.995x | 0.991x | 1.113x |
| corpus_uci_sorted | 0.989x | 0.978x | 1.768x |
| replay_trusted_native | 1.026x | 1.000x | 0.863x |
| replay_checked_native | 0.997x | 0.993x | 1.034x |

Cold first legal move list: 0.008 ms -> 0.010 ms (0.85x).
