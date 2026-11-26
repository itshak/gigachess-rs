| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.983x | 1.586x |
| materialize | 0.993x | 0.971x | 1.182x |
| perft_bulk_d4 | 1.004x | 0.978x | 1.382x |
| perft_full_d3 | 0.996x | 0.974x | 1.192x |
| fen_parse | 1.040x | 1.014x | 1.345x |
| fen_write_native_ep | 0.976x | 0.968x | 1.822x |
| snapshot | 1.001x | 0.995x | 1.249x |
| san_parse_all | 0.997x | 0.986x | 2.220x |
| san_render_native | 0.927x | 0.846x | 1.990x |
| apply_children_full | 1.012x | 0.976x | 1.331x |
| make_unmake_full | 0.992x | 0.927x | 1.225x |
| startpos | 1.054x | 1.054x | 1.198x |
| chess960_materialize | 1.010x | 0.998x | 1.583x |
| chess960_perft_d3 | 1.014x | 1.000x | 1.641x |
| polyglot_key | 0.991x | 0.982x | 20.327x |
| polyglot_recompute | 1.006x | 0.999x | 1.019x |
| repetition_key | 1.002x | 1.001x | 18.821x |
| corpus_count | 0.997x | 0.995x | 1.291x |
| corpus_materialize | 1.002x | 1.001x | 1.234x |
| corpus_uci_sorted | 0.982x | 0.972x | 1.660x |
| replay_trusted_native | 0.963x | 0.868x | 0.972x |
| replay_checked_native | 0.962x | 0.924x | 1.050x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.12x).
