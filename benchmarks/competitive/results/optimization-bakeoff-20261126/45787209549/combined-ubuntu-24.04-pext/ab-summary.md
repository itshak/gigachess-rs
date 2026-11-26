| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.994x | 1.481x |
| materialize | 0.997x | 0.990x | 1.577x |
| perft_bulk_d4 | 0.997x | 0.967x | 1.266x |
| perft_full_d3 | 0.969x | 0.931x | 1.472x |
| fen_parse | 0.966x | 0.885x | 1.076x |
| fen_write_native_ep | 1.010x | 0.993x | 1.916x |
| snapshot | 1.415x | 1.407x | 1.154x |
| san_parse_all | 1.164x | 1.091x | 1.945x |
| san_render_native | 0.994x | 0.974x | 2.150x |
| apply_children_full | 0.983x | 0.975x | 1.064x |
| make_unmake_full | 0.991x | 0.952x | 1.116x |
| startpos | 1.224x | 1.224x | 1.205x |
| chess960_materialize | 0.927x | 0.915x | 1.378x |
| chess960_perft_d3 | 1.000x | 0.996x | 1.394x |
| polyglot_key | 0.957x | 0.915x | 38.464x |
| polyglot_recompute | 1.059x | 1.007x | 1.086x |
| repetition_key | 0.975x | 0.959x | 18.902x |
| corpus_count | 1.003x | 0.995x | 1.412x |
| corpus_materialize | 1.016x | 1.012x | 1.143x |
| corpus_uci_sorted | 0.991x | 0.989x | 1.566x |
| replay_trusted_native | 0.979x | 0.952x | 0.701x |
| replay_checked_native | 0.995x | 0.980x | 1.011x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.01x).
