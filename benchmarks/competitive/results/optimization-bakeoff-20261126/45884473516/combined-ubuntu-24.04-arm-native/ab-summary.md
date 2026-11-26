| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.993x | 1.552x |
| materialize | 1.005x | 1.004x | 1.332x |
| perft_bulk_d4 | 0.998x | 0.993x | 1.391x |
| perft_full_d3 | 0.986x | 0.959x | 1.314x |
| fen_parse | 0.995x | 0.986x | 1.211x |
| fen_write_native_ep | 0.999x | 0.998x | 1.573x |
| snapshot | 0.988x | 0.985x | 0.948x |
| san_parse_all | 0.973x | 0.969x | 1.781x |
| san_render_native | 1.005x | 0.964x | 2.701x |
| apply_children_full | 0.990x | 0.971x | 1.152x |
| make_unmake_full | 1.007x | 0.977x | 1.582x |
| startpos | 1.000x | 1.000x | 1.306x |
| chess960_materialize | 0.992x | 0.986x | 1.478x |
| chess960_perft_d3 | 0.993x | 0.985x | 1.485x |
| polyglot_key | 0.980x | 0.954x | 25.208x |
| polyglot_recompute | 0.994x | 0.987x | 1.083x |
| repetition_key | 0.985x | 0.972x | 19.052x |
| corpus_count | 0.994x | 0.986x | 1.351x |
| corpus_materialize | 0.993x | 0.989x | 1.229x |
| corpus_uci_sorted | 1.003x | 1.003x | 1.456x |
| replay_trusted_native | 0.935x | 0.934x | 0.603x |
| replay_checked_native | 0.975x | 0.972x | 1.101x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.96x).
