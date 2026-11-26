| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.014x | 0.975x | 1.318x |
| materialize | 1.004x | 0.989x | 1.255x |
| perft_bulk_d4 | 1.014x | 1.005x | 1.265x |
| perft_full_d3 | 0.977x | 0.954x | 1.177x |
| fen_parse | 1.107x | 1.076x | 1.111x |
| fen_write_native_ep | 0.994x | 0.983x | 1.741x |
| snapshot | 1.009x | 0.999x | 1.250x |
| san_parse_all | 1.005x | 0.969x | 2.056x |
| san_render_native | 0.993x | 0.971x | 2.203x |
| apply_children_full | 1.008x | 0.996x | 1.215x |
| make_unmake_full | 1.000x | 0.990x | 1.234x |
| startpos | 0.999x | 0.999x | 1.258x |
| chess960_materialize | 1.001x | 0.995x | 1.294x |
| chess960_perft_d3 | 0.979x | 0.963x | 1.372x |
| polyglot_key | 0.915x | 0.835x | 27.728x |
| polyglot_recompute | 1.025x | 1.024x | 1.054x |
| repetition_key | 1.010x | 0.999x | 19.478x |
| corpus_count | 1.006x | 1.000x | 1.299x |
| corpus_materialize | 0.976x | 0.971x | 1.119x |
| corpus_uci_sorted | 0.993x | 0.993x | 1.727x |
| replay_trusted_native | 1.003x | 1.001x | 0.815x |
| replay_checked_native | 0.992x | 0.982x | 1.036x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.05x).
