| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.997x | 1.083x |
| materialize | 0.998x | 0.996x | 1.171x |
| perft_bulk_d4 | 0.980x | 0.968x | 0.889x |
| perft_full_d3 | 0.999x | 0.992x | 0.611x |
| fen_parse | 1.007x | 1.000x | 1.219x |
| fen_write_native_ep | 1.015x | 1.011x | 1.054x |
| snapshot | 1.007x | 0.999x | 0.776x |
| san_parse_all | 1.433x | 1.410x | 1.304x |
| san_render_native | 1.329x | 1.262x | 1.517x |
| apply_children_full | 1.005x | 0.995x | 0.657x |
| make_unmake_full | 1.012x | 1.006x | 0.903x |
| startpos | 1.131x | 1.131x | 0.836x |
| chess960_materialize | 1.000x | 0.998x | 1.107x |
| chess960_perft_d3 | 0.999x | 0.992x | 0.846x |
| polyglot_key | 1.015x | 1.005x | 23.243x |
| polyglot_recompute | 1.002x | 0.998x | 1.134x |
| repetition_key | 1.068x | 1.052x | 14.820x |
| corpus_count | 0.988x | 0.975x | 1.188x |
| corpus_materialize | 1.001x | 0.995x | 1.126x |
| corpus_uci_sorted | 1.010x | 1.010x | 0.570x |
| replay_trusted_native | 1.001x | 0.997x | 0.323x |
| replay_checked_native | 1.054x | 1.049x | 0.694x |

Cold first legal move list: 154.160 ms → 153.941 ms (1.00x).
