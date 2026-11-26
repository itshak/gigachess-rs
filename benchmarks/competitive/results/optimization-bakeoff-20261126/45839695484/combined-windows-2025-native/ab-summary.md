| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.975x | 1.509x |
| materialize | 1.014x | 0.984x | 1.155x |
| perft_bulk_d4 | 0.993x | 0.975x | 1.427x |
| perft_full_d3 | 1.012x | 0.995x | 0.966x |
| fen_parse | 1.003x | 0.997x | 1.297x |
| fen_write_native_ep | 0.998x | 0.990x | 1.597x |
| snapshot | 0.993x | 0.952x | 1.209x |
| san_parse_all | 1.010x | 0.995x | 1.922x |
| san_render_native | 1.007x | 0.978x | 3.307x |
| apply_children_full | 0.984x | 0.938x | 1.270x |
| make_unmake_full | 1.005x | 0.951x | 1.282x |
| startpos | 1.001x | 1.001x | 1.001x |
| chess960_materialize | 1.006x | 1.000x | 1.357x |
| chess960_perft_d3 | 1.056x | 1.031x | 1.573x |
| polyglot_key | 0.999x | 0.998x | 20.236x |
| polyglot_recompute | 1.061x | 1.055x | 0.955x |
| repetition_key | 1.012x | 0.996x | 18.809x |
| corpus_count | 1.049x | 1.004x | 1.293x |
| corpus_materialize | 1.038x | 1.006x | 1.211x |
| corpus_uci_sorted | 1.046x | 1.042x | 1.500x |
| replay_trusted_native | 0.981x | 0.980x | 1.112x |
| replay_checked_native | 1.035x | 1.031x | 1.011x |

Cold first legal move list: 0.016 ms -> 0.014 ms (1.15x).
