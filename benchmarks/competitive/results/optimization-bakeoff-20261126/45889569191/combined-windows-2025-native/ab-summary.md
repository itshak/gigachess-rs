| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.037x | 0.996x | 1.570x |
| materialize | 1.019x | 0.987x | 1.385x |
| perft_bulk_d4 | 1.086x | 0.970x | 1.198x |
| perft_full_d3 | 1.234x | 1.111x | 1.643x |
| fen_parse | 0.976x | 0.915x | 1.200x |
| fen_write_native_ep | 0.980x | 0.877x | 1.693x |
| snapshot | 1.006x | 0.867x | 1.064x |
| san_parse_all | 1.010x | 0.972x | 1.789x |
| san_render_native | 0.988x | 0.930x | 4.151x |
| apply_children_full | 1.008x | 0.975x | 1.352x |
| make_unmake_full | 1.017x | 0.931x | 1.213x |
| startpos | 0.987x | 0.987x | 1.001x |
| chess960_materialize | 1.011x | 0.994x | 1.468x |
| chess960_perft_d3 | 0.960x | 0.912x | 1.495x |
| polyglot_key | 0.971x | 0.951x | 39.784x |
| polyglot_recompute | 1.166x | 0.891x | 1.216x |
| repetition_key | 1.212x | 1.192x | 32.867x |
| corpus_count | 0.960x | 0.960x | 0.957x |
| corpus_materialize | 0.966x | 0.958x | 1.063x |
| corpus_uci_sorted | 0.977x | 0.967x | 1.598x |
| replay_trusted_native | 0.976x | 0.946x | 0.837x |
| replay_checked_native | 0.980x | 0.978x | 1.165x |

Cold first legal move list: 0.016 ms -> 0.015 ms (1.01x).
