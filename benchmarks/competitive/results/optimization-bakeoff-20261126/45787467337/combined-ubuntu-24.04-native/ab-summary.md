| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.991x | 1.401x |
| materialize | 1.055x | 1.029x | 1.511x |
| perft_bulk_d4 | 1.003x | 1.001x | 1.215x |
| perft_full_d3 | 0.944x | 0.930x | 1.449x |
| fen_parse | 1.058x | 1.037x | 1.098x |
| fen_write_native_ep | 0.970x | 0.920x | 2.051x |
| snapshot | 0.849x | 0.847x | 0.998x |
| san_parse_all | 0.998x | 0.976x | 1.923x |
| san_render_native | 0.968x | 0.958x | 2.072x |
| apply_children_full | 0.936x | 0.906x | 1.014x |
| make_unmake_full | 1.016x | 0.997x | 1.157x |
| startpos | 0.816x | 0.816x | 1.168x |
| chess960_materialize | 1.004x | 0.999x | 1.440x |
| chess960_perft_d3 | 0.995x | 0.992x | 1.366x |
| polyglot_key | 1.004x | 0.998x | 41.089x |
| polyglot_recompute | 1.000x | 0.994x | 1.056x |
| repetition_key | 0.951x | 0.803x | 19.819x |
| corpus_count | 0.997x | 0.997x | 1.332x |
| corpus_materialize | 0.997x | 0.994x | 1.083x |
| corpus_uci_sorted | 1.017x | 1.013x | 1.564x |
| replay_trusted_native | 1.016x | 1.000x | 0.722x |
| replay_checked_native | 1.020x | 1.015x | 1.163x |

Cold first legal move list: 0.009 ms -> 0.009 ms (0.98x).
