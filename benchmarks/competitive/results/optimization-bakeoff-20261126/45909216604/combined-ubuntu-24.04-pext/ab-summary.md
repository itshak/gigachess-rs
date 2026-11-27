| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.041x | 1.007x | 1.543x |
| materialize | 0.994x | 0.971x | 1.267x |
| perft_bulk_d4 | 1.021x | 0.997x | 1.460x |
| perft_full_d3 | 0.996x | 0.984x | 1.283x |
| fen_parse | 0.998x | 0.983x | 1.240x |
| fen_write_native_ep | 0.953x | 0.916x | 1.654x |
| snapshot | 0.721x | 0.718x | 0.900x |
| san_parse_all | 0.996x | 0.985x | 2.006x |
| san_render_native | 0.994x | 0.981x | 2.275x |
| apply_children_full | 1.008x | 0.995x | 1.328x |
| make_unmake_full | 1.013x | 0.972x | 1.334x |
| startpos | 1.000x | 1.000x | 1.252x |
| chess960_materialize | 0.995x | 0.978x | 1.393x |
| chess960_perft_d3 | 1.002x | 0.998x | 1.494x |
| polyglot_key | 1.072x | 1.044x | 29.736x |
| polyglot_recompute | 0.999x | 0.999x | 1.037x |
| repetition_key | 0.982x | 0.980x | 20.848x |
| corpus_count | 1.075x | 1.042x | 1.505x |
| corpus_materialize | 0.983x | 0.981x | 1.180x |
| corpus_uci_sorted | 0.999x | 0.996x | 1.744x |
| replay_trusted_native | 0.972x | 0.971x | 0.870x |
| replay_checked_native | 0.983x | 0.968x | 1.110x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.09x).
