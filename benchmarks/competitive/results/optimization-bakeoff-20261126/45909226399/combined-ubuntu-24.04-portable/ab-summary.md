| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.971x | 0.897x | 1.249x |
| materialize | 0.993x | 0.973x | 1.738x |
| perft_bulk_d4 | 0.976x | 0.927x | 1.175x |
| perft_full_d3 | 1.001x | 0.964x | 1.307x |
| fen_parse | 1.033x | 1.005x | 1.167x |
| fen_write_native_ep | 0.989x | 0.895x | 2.024x |
| snapshot | 0.986x | 0.936x | 0.995x |
| san_parse_all | 0.979x | 0.911x | 1.864x |
| san_render_native | 0.987x | 0.912x | 2.192x |
| apply_children_full | 0.962x | 0.813x | 1.323x |
| make_unmake_full | 1.082x | 0.975x | 1.292x |
| startpos | 1.025x | 1.025x | 1.359x |
| chess960_materialize | 0.942x | 0.909x | 1.483x |
| chess960_perft_d3 | 1.017x | 1.011x | 1.519x |
| polyglot_key | 0.977x | 0.973x | 37.842x |
| polyglot_recompute | 1.061x | 1.033x | 1.041x |
| repetition_key | 1.178x | 1.006x | 23.224x |
| corpus_count | 1.030x | 1.026x | 1.125x |
| corpus_materialize | 0.994x | 0.990x | 1.133x |
| corpus_uci_sorted | 1.004x | 0.976x | 1.625x |
| replay_trusted_native | 0.921x | 0.911x | 0.833x |
| replay_checked_native | 1.008x | 0.984x | 1.164x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.78x).
