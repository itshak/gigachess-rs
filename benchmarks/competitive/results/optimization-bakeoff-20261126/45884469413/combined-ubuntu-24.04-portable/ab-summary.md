| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.988x | 1.323x |
| materialize | 1.126x | 1.081x | 1.760x |
| perft_bulk_d4 | 1.003x | 0.980x | 1.192x |
| perft_full_d3 | 1.001x | 0.996x | 1.329x |
| fen_parse | 1.119x | 1.058x | 1.216x |
| fen_write_native_ep | 1.012x | 1.000x | 1.861x |
| snapshot | 0.997x | 0.989x | 0.997x |
| san_parse_all | 1.013x | 0.993x | 1.908x |
| san_render_native | 0.999x | 0.986x | 2.162x |
| apply_children_full | 0.999x | 0.997x | 1.363x |
| make_unmake_full | 1.047x | 0.999x | 1.258x |
| startpos | 1.001x | 1.001x | 1.363x |
| chess960_materialize | 1.080x | 1.070x | 1.585x |
| chess960_perft_d3 | 1.003x | 1.000x | 1.510x |
| polyglot_key | 1.001x | 1.001x | 37.806x |
| polyglot_recompute | 1.066x | 1.030x | 1.029x |
| repetition_key | 0.968x | 0.957x | 19.229x |
| corpus_count | 1.005x | 1.005x | 1.143x |
| corpus_materialize | 0.999x | 0.997x | 1.151x |
| corpus_uci_sorted | 1.001x | 0.997x | 1.555x |
| replay_trusted_native | 1.002x | 0.998x | 0.919x |
| replay_checked_native | 0.998x | 0.994x | 1.161x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.82x).
