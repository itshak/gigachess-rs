| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.992x | 1.235x |
| materialize | 0.994x | 0.978x | 1.554x |
| perft_bulk_d4 | 0.996x | 0.988x | 1.176x |
| perft_full_d3 | 1.025x | 1.010x | 1.568x |
| fen_parse | 1.006x | 0.990x | 1.270x |
| fen_write_native_ep | 0.996x | 0.969x | 1.962x |
| snapshot | 0.821x | 0.809x | 1.003x |
| san_parse_all | 0.981x | 0.952x | 1.740x |
| san_render_native | 1.005x | 0.972x | 2.251x |
| apply_children_full | 0.999x | 0.982x | 1.074x |
| make_unmake_full | 1.008x | 0.987x | 1.215x |
| startpos | 1.000x | 1.000x | 1.286x |
| chess960_materialize | 0.995x | 0.991x | 1.340x |
| chess960_perft_d3 | 0.997x | 0.997x | 1.335x |
| polyglot_key | 0.962x | 0.920x | 40.033x |
| polyglot_recompute | 0.977x | 0.975x | 1.032x |
| repetition_key | 0.906x | 0.877x | 23.783x |
| corpus_count | 0.998x | 0.996x | 1.225x |
| corpus_materialize | 1.009x | 1.005x | 1.151x |
| corpus_uci_sorted | 1.015x | 1.012x | 1.584x |
| replay_trusted_native | 0.989x | 0.981x | 0.682x |
| replay_checked_native | 1.040x | 1.028x | 1.095x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.92x).
