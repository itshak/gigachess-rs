| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.888x | 0.793x | 1.588x |
| materialize | 0.936x | 0.824x | 1.148x |
| perft_bulk_d4 | 0.974x | 0.836x | 1.246x |
| perft_full_d3 | 0.944x | 0.828x | 1.286x |
| fen_parse | 0.926x | 0.811x | 1.218x |
| fen_write_native_ep | 0.952x | 0.657x | 1.689x |
| snapshot | 1.054x | 0.513x | 0.868x |
| san_parse_all | 0.964x | 0.784x | 1.619x |
| san_render_native | 1.075x | 0.901x | 2.602x |
| apply_children_full | 0.998x | 0.782x | 1.061x |
| make_unmake_full | 0.993x | 0.911x | 1.326x |
| startpos | 0.803x | 0.803x | 0.574x |
| chess960_materialize | 0.945x | 0.831x | 1.565x |
| chess960_perft_d3 | 0.961x | 0.904x | 1.528x |
| polyglot_key | 1.007x | 0.995x | 39.631x |
| polyglot_recompute | 0.849x | 0.785x | 1.067x |
| repetition_key | 0.948x | 0.882x | 19.782x |
| corpus_count | 0.854x | 0.626x | 0.992x |
| corpus_materialize | 0.802x | 0.778x | 1.180x |
| corpus_uci_sorted | 0.873x | 0.826x | 1.196x |
| replay_trusted_native | 1.089x | 1.033x | 0.813x |
| replay_checked_native | 0.920x | 0.856x | 0.995x |

Cold first legal move list: 0.004 ms -> 0.006 ms (0.63x).
