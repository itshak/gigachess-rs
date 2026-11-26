| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.008x | 0.995x | 1.291x |
| materialize | 0.998x | 0.962x | 1.189x |
| perft_bulk_d4 | 0.977x | 0.954x | 1.238x |
| perft_full_d3 | 0.793x | 0.768x | 0.984x |
| fen_parse | 1.106x | 1.060x | 1.231x |
| fen_write_native_ep | 1.033x | 1.005x | 1.736x |
| snapshot | 0.755x | 0.749x | 1.035x |
| san_parse_all | 0.959x | 0.945x | 2.008x |
| san_render_native | 1.023x | 1.009x | 2.306x |
| apply_children_full | 1.116x | 1.000x | 1.085x |
| make_unmake_full | 0.987x | 0.977x | 1.212x |
| startpos | 0.926x | 0.926x | 1.194x |
| chess960_materialize | 0.989x | 0.979x | 1.282x |
| chess960_perft_d3 | 0.973x | 0.964x | 1.311x |
| polyglot_key | 0.933x | 0.927x | 28.230x |
| polyglot_recompute | 1.094x | 1.059x | 1.129x |
| repetition_key | 0.943x | 0.886x | 21.127x |
| corpus_count | 0.989x | 0.981x | 1.333x |
| corpus_materialize | 0.986x | 0.983x | 1.172x |
| corpus_uci_sorted | 1.006x | 1.004x | 1.813x |
| replay_trusted_native | 1.019x | 0.993x | 0.701x |
| replay_checked_native | 1.001x | 0.982x | 1.003x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.01x).
