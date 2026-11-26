| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.547x |
| materialize | 1.000x | 0.997x | 1.321x |
| perft_bulk_d4 | 1.001x | 0.996x | 1.391x |
| perft_full_d3 | 1.000x | 1.000x | 1.332x |
| fen_parse | 0.995x | 0.988x | 1.222x |
| fen_write_native_ep | 1.000x | 0.998x | 1.566x |
| snapshot | 1.001x | 0.998x | 0.945x |
| san_parse_all | 0.998x | 0.993x | 1.831x |
| san_render_native | 1.005x | 0.998x | 2.671x |
| apply_children_full | 0.999x | 0.984x | 1.154x |
| make_unmake_full | 1.045x | 1.021x | 1.628x |
| startpos | 0.999x | 0.999x | 1.301x |
| chess960_materialize | 1.002x | 0.999x | 1.497x |
| chess960_perft_d3 | 1.002x | 1.001x | 1.501x |
| polyglot_key | 0.968x | 0.933x | 25.160x |
| polyglot_recompute | 0.995x | 0.992x | 1.084x |
| repetition_key | 0.977x | 0.953x | 19.019x |
| corpus_count | 0.994x | 0.990x | 1.391x |
| corpus_materialize | 0.991x | 0.990x | 1.219x |
| corpus_uci_sorted | 0.999x | 0.999x | 1.461x |
| replay_trusted_native | 1.068x | 1.027x | 0.672x |
| replay_checked_native | 1.015x | 0.999x | 1.148x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.00x).
