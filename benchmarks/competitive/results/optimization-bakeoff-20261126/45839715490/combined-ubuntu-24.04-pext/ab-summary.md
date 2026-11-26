| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.993x | 1.473x |
| materialize | 1.011x | 0.989x | 1.230x |
| perft_bulk_d4 | 0.981x | 0.963x | 1.284x |
| perft_full_d3 | 1.021x | 0.995x | 1.106x |
| fen_parse | 1.015x | 0.985x | 1.195x |
| fen_write_native_ep | 0.992x | 0.976x | 1.536x |
| snapshot | 1.052x | 0.982x | 1.487x |
| san_parse_all | 1.004x | 0.986x | 1.781x |
| san_render_native | 0.970x | 0.933x | 2.161x |
| apply_children_full | 1.017x | 0.994x | 1.574x |
| make_unmake_full | 1.052x | 1.006x | 1.221x |
| startpos | 1.015x | 1.015x | 1.240x |
| chess960_materialize | 1.027x | 1.022x | 1.553x |
| chess960_perft_d3 | 1.014x | 1.011x | 1.522x |
| polyglot_key | 1.037x | 1.029x | 36.401x |
| polyglot_recompute | 0.981x | 0.972x | 0.946x |
| repetition_key | 1.006x | 1.002x | 27.471x |
| corpus_count | 1.011x | 0.973x | 1.173x |
| corpus_materialize | 0.991x | 0.949x | 1.127x |
| corpus_uci_sorted | 1.005x | 1.001x | 1.648x |
| replay_trusted_native | 0.986x | 0.958x | 0.703x |
| replay_checked_native | 0.995x | 0.972x | 1.101x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.05x).
