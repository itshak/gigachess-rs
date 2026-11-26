| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.971x | 1.192x |
| materialize | 1.011x | 0.992x | 1.267x |
| perft_bulk_d4 | 1.015x | 0.998x | 1.225x |
| perft_full_d3 | 1.014x | 0.943x | 1.333x |
| fen_parse | 0.854x | 0.837x | 1.054x |
| fen_write_native_ep | 0.981x | 0.975x | 0.920x |
| snapshot | 1.001x | 0.998x | 0.974x |
| san_parse_all | 0.989x | 0.949x | 2.007x |
| san_render_native | 0.961x | 0.924x | 2.324x |
| apply_children_full | 1.023x | 1.013x | 1.217x |
| make_unmake_full | 0.987x | 0.969x | 1.325x |
| startpos | 1.030x | 1.030x | 1.343x |
| chess960_materialize | 0.981x | 0.931x | 1.274x |
| chess960_perft_d3 | 1.021x | 1.014x | 1.402x |
| polyglot_key | 0.963x | 0.918x | 24.570x |
| polyglot_recompute | 0.970x | 0.970x | 1.018x |
| repetition_key | 0.940x | 0.890x | 17.993x |
| corpus_count | 1.002x | 1.002x | 1.285x |
| corpus_materialize | 1.017x | 1.014x | 1.175x |
| corpus_uci_sorted | 1.004x | 0.993x | 1.709x |
| replay_trusted_native | 1.026x | 1.000x | 0.728x |
| replay_checked_native | 1.011x | 1.006x | 1.052x |

Cold first legal move list: 0.016 ms -> 0.013 ms (1.28x).
