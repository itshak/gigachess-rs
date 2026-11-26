| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.993x | 1.195x |
| materialize | 0.990x | 0.977x | 1.636x |
| perft_bulk_d4 | 1.002x | 0.999x | 1.166x |
| perft_full_d3 | 0.990x | 0.971x | 1.486x |
| fen_parse | 1.009x | 0.998x | 1.128x |
| fen_write_native_ep | 0.997x | 0.986x | 1.828x |
| snapshot | 1.009x | 1.002x | 1.004x |
| san_parse_all | 1.006x | 0.995x | 1.769x |
| san_render_native | 0.974x | 0.959x | 2.173x |
| apply_children_full | 0.992x | 0.975x | 1.126x |
| make_unmake_full | 0.987x | 0.957x | 1.191x |
| startpos | 1.003x | 1.003x | 1.285x |
| chess960_materialize | 0.944x | 0.939x | 1.329x |
| chess960_perft_d3 | 0.997x | 0.995x | 1.343x |
| polyglot_key | 1.020x | 0.997x | 40.548x |
| polyglot_recompute | 1.029x | 0.960x | 0.963x |
| repetition_key | 1.197x | 1.073x | 23.675x |
| corpus_count | 0.999x | 0.997x | 1.257x |
| corpus_materialize | 1.001x | 1.000x | 1.172x |
| corpus_uci_sorted | 0.994x | 0.993x | 1.588x |
| replay_trusted_native | 1.008x | 1.000x | 0.721x |
| replay_checked_native | 1.001x | 0.999x | 1.106x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.05x).
