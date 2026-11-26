| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.036x | 0.987x | 1.306x |
| materialize | 1.007x | 0.994x | 1.183x |
| perft_bulk_d4 | 1.006x | 0.991x | 1.304x |
| perft_full_d3 | 0.949x | 0.837x | 1.191x |
| fen_parse | 1.138x | 1.104x | 1.340x |
| fen_write_native_ep | 1.019x | 0.989x | 1.030x |
| snapshot | 1.001x | 0.994x | 1.034x |
| san_parse_all | 1.003x | 0.991x | 1.990x |
| san_render_native | 1.007x | 0.985x | 3.701x |
| apply_children_full | 0.997x | 0.991x | 1.185x |
| make_unmake_full | 0.992x | 0.969x | 1.260x |
| startpos | 0.979x | 0.979x | 1.086x |
| chess960_materialize | 1.068x | 1.038x | 1.270x |
| chess960_perft_d3 | 1.060x | 1.030x | 1.340x |
| polyglot_key | 1.038x | 1.018x | 27.166x |
| polyglot_recompute | 1.024x | 1.015x | 1.022x |
| repetition_key | 1.023x | 1.018x | 19.691x |
| corpus_count | 1.087x | 1.066x | 1.350x |
| corpus_materialize | 1.035x | 0.999x | 1.172x |
| corpus_uci_sorted | 1.087x | 1.074x | 1.410x |
| replay_trusted_native | 1.026x | 1.003x | 0.676x |
| replay_checked_native | 1.024x | 1.016x | 0.978x |

Cold first legal move list: 0.024 ms -> 0.046 ms (0.53x).
