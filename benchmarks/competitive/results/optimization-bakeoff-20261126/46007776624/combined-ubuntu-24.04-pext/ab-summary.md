| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.080x | 1.014x | 1.593x |
| materialize | 1.000x | 0.977x | 1.184x |
| perft_bulk_d4 | 1.055x | 0.991x | 1.459x |
| perft_full_d3 | 1.313x | 1.081x | 1.619x |
| fen_parse | 1.026x | 1.003x | 1.080x |
| fen_write_native_ep | 1.023x | 0.970x | 1.859x |
| snapshot | 0.774x | 0.769x | 0.969x |
| san_parse_all | 1.013x | 0.994x | 1.950x |
| san_render_native | 0.990x | 0.950x | 2.127x |
| apply_children_full | 0.999x | 0.969x | 1.323x |
| make_unmake_full | 0.988x | 0.886x | 1.195x |
| startpos | 1.001x | 1.001x | 1.197x |
| chess960_materialize | 1.000x | 0.972x | 1.484x |
| chess960_perft_d3 | 1.008x | 1.002x | 1.564x |
| polyglot_key | 0.996x | 0.995x | 20.249x |
| polyglot_recompute | 0.755x | 0.741x | 0.771x |
| repetition_key | 0.987x | 0.971x | 18.377x |
| corpus_count | 1.068x | 1.043x | 1.465x |
| corpus_materialize | 1.004x | 1.004x | 1.239x |
| corpus_uci_sorted | 1.001x | 0.996x | 1.817x |
| replay_trusted_native | 1.000x | 1.000x | 1.102x |
| replay_checked_native | 1.001x | 1.001x | 1.117x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.14x).
