| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.122x | 0.966x | 1.547x |
| materialize | 1.061x | 0.982x | 1.315x |
| perft_bulk_d4 | 1.129x | 1.008x | 1.396x |
| perft_full_d3 | 1.028x | 1.008x | 1.100x |
| fen_parse | 1.001x | 0.996x | 1.232x |
| fen_write_native_ep | 0.998x | 0.996x | 1.568x |
| snapshot | 1.002x | 0.999x | 0.936x |
| san_parse_all | 1.001x | 0.998x | 1.812x |
| san_render_native | 1.001x | 0.993x | 2.513x |
| apply_children_full | 0.998x | 0.970x | 1.288x |
| make_unmake_full | 1.023x | 0.992x | 1.533x |
| startpos | 0.999x | 0.999x | 1.276x |
| chess960_materialize | 1.066x | 1.030x | 1.477x |
| chess960_perft_d3 | 1.114x | 1.070x | 1.500x |
| polyglot_key | 0.989x | 0.989x | 25.037x |
| polyglot_recompute | 0.998x | 0.997x | 1.064x |
| repetition_key | 0.991x | 0.990x | 18.104x |
| corpus_count | 1.134x | 1.089x | 1.385x |
| corpus_materialize | 1.045x | 1.036x | 1.234x |
| corpus_uci_sorted | 1.007x | 1.001x | 1.462x |
| replay_trusted_native | 1.013x | 1.008x | 0.610x |
| replay_checked_native | 0.990x | 0.989x | 1.049x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.01x).
