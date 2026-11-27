| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.143x | 1.015x | 1.538x |
| materialize | 1.002x | 0.979x | 1.228x |
| perft_bulk_d4 | 1.104x | 1.014x | 1.430x |
| perft_full_d3 | 1.029x | 0.992x | 1.341x |
| fen_parse | 1.054x | 0.998x | 1.301x |
| fen_write_native_ep | 1.023x | 0.994x | 1.561x |
| snapshot | 1.003x | 0.989x | 1.000x |
| san_parse_all | 1.027x | 0.990x | 2.075x |
| san_render_native | 1.019x | 0.991x | 3.868x |
| apply_children_full | 1.019x | 0.998x | 1.421x |
| make_unmake_full | 1.038x | 0.986x | 1.369x |
| startpos | 1.298x | 1.298x | 1.440x |
| chess960_materialize | 1.040x | 1.019x | 1.332x |
| chess960_perft_d3 | 1.017x | 0.992x | 1.526x |
| polyglot_key | 1.005x | 1.000x | 29.551x |
| polyglot_recompute | 1.000x | 0.997x | 1.013x |
| repetition_key | 1.043x | 1.031x | 22.397x |
| corpus_count | 1.088x | 1.062x | 1.493x |
| corpus_materialize | 1.001x | 1.000x | 1.139x |
| corpus_uci_sorted | 1.007x | 1.005x | 1.437x |
| replay_trusted_native | 0.988x | 0.975x | 0.830x |
| replay_checked_native | 0.999x | 0.993x | 1.084x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.94x).
