| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.998x | 1.562x |
| materialize | 1.002x | 0.999x | 1.322x |
| perft_bulk_d4 | 1.000x | 0.996x | 1.390x |
| perft_full_d3 | 1.003x | 1.001x | 1.338x |
| fen_parse | 0.988x | 0.933x | 1.209x |
| fen_write_native_ep | 1.000x | 0.999x | 1.603x |
| snapshot | 1.001x | 0.996x | 0.952x |
| san_parse_all | 0.999x | 0.996x | 1.828x |
| san_render_native | 1.001x | 0.990x | 2.671x |
| apply_children_full | 1.002x | 0.999x | 1.160x |
| make_unmake_full | 0.999x | 0.988x | 1.552x |
| startpos | 1.001x | 1.001x | 1.304x |
| chess960_materialize | 0.996x | 0.989x | 1.496x |
| chess960_perft_d3 | 1.004x | 1.004x | 1.509x |
| polyglot_key | 1.004x | 0.994x | 25.742x |
| polyglot_recompute | 1.032x | 0.954x | 1.125x |
| repetition_key | 0.994x | 0.990x | 19.290x |
| corpus_count | 0.996x | 0.992x | 1.369x |
| corpus_materialize | 0.997x | 0.996x | 1.227x |
| corpus_uci_sorted | 1.005x | 1.002x | 1.461x |
| replay_trusted_native | 0.996x | 0.987x | 0.645x |
| replay_checked_native | 0.994x | 0.991x | 1.124x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.96x).
