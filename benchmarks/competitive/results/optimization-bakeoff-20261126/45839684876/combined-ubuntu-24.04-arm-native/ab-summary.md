| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.997x | 1.547x |
| materialize | 0.998x | 0.994x | 1.315x |
| perft_bulk_d4 | 0.977x | 0.960x | 1.363x |
| perft_full_d3 | 0.990x | 0.982x | 0.825x |
| fen_parse | 1.009x | 1.001x | 1.216x |
| fen_write_native_ep | 0.996x | 0.989x | 1.560x |
| snapshot | 0.999x | 0.995x | 0.945x |
| san_parse_all | 1.000x | 0.988x | 1.834x |
| san_render_native | 0.977x | 0.915x | 2.461x |
| apply_children_full | 0.971x | 0.949x | 1.118x |
| make_unmake_full | 0.932x | 0.895x | 1.453x |
| startpos | 1.001x | 1.001x | 1.307x |
| chess960_materialize | 1.002x | 1.002x | 1.499x |
| chess960_perft_d3 | 0.986x | 0.978x | 1.476x |
| polyglot_key | 1.000x | 0.999x | 25.955x |
| polyglot_recompute | 0.996x | 0.991x | 1.083x |
| repetition_key | 0.997x | 0.994x | 19.299x |
| corpus_count | 0.995x | 0.995x | 1.352x |
| corpus_materialize | 1.009x | 1.006x | 1.225x |
| corpus_uci_sorted | 0.999x | 0.998x | 1.456x |
| replay_trusted_native | 1.046x | 0.977x | 0.643x |
| replay_checked_native | 0.998x | 0.997x | 1.046x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.92x).
