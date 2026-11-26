| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.994x | 1.353x |
| materialize | 1.003x | 0.999x | 1.243x |
| perft_bulk_d4 | 0.998x | 0.992x | 1.340x |
| perft_full_d3 | 1.005x | 1.001x | 1.074x |
| fen_parse | 1.003x | 0.999x | 1.241x |
| fen_write_native_ep | 0.996x | 0.988x | 1.596x |
| snapshot | 1.018x | 1.015x | 0.915x |
| san_parse_all | 0.999x | 0.994x | 1.818x |
| san_render_native | 0.998x | 0.992x | 2.559x |
| apply_children_full | 1.008x | 0.999x | 1.053x |
| make_unmake_full | 0.995x | 0.993x | 1.432x |
| startpos | 1.073x | 1.073x | 1.184x |
| chess960_materialize | 0.996x | 0.993x | 1.321x |
| chess960_perft_d3 | 0.993x | 0.989x | 1.310x |
| polyglot_key | 0.999x | 0.996x | 25.645x |
| polyglot_recompute | 1.005x | 1.002x | 1.076x |
| repetition_key | 0.984x | 0.969x | 19.234x |
| corpus_count | 0.993x | 0.988x | 1.456x |
| corpus_materialize | 0.996x | 0.996x | 1.200x |
| corpus_uci_sorted | 1.004x | 1.000x | 1.458x |
| replay_trusted_native | 1.020x | 1.003x | 0.508x |
| replay_checked_native | 0.983x | 0.977x | 0.982x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.04x).
