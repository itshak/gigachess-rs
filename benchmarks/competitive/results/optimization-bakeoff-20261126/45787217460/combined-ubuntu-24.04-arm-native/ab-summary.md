| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.993x | 1.352x |
| materialize | 0.994x | 0.980x | 1.229x |
| perft_bulk_d4 | 1.001x | 0.993x | 1.346x |
| perft_full_d3 | 0.926x | 0.907x | 0.974x |
| fen_parse | 1.006x | 0.996x | 1.250x |
| fen_write_native_ep | 1.000x | 0.993x | 1.572x |
| snapshot | 0.938x | 0.930x | 0.894x |
| san_parse_all | 0.999x | 0.996x | 1.812x |
| san_render_native | 0.982x | 0.962x | 2.491x |
| apply_children_full | 0.948x | 0.920x | 0.987x |
| make_unmake_full | 1.028x | 1.019x | 1.484x |
| startpos | 0.963x | 0.963x | 1.188x |
| chess960_materialize | 1.008x | 1.006x | 1.330x |
| chess960_perft_d3 | 0.996x | 0.991x | 1.327x |
| polyglot_key | 0.978x | 0.963x | 25.578x |
| polyglot_recompute | 0.997x | 0.996x | 1.074x |
| repetition_key | 0.965x | 0.960x | 18.553x |
| corpus_count | 1.000x | 0.991x | 1.478x |
| corpus_materialize | 1.011x | 1.007x | 1.213x |
| corpus_uci_sorted | 1.006x | 1.005x | 1.459x |
| replay_trusted_native | 1.021x | 1.008x | 0.513x |
| replay_checked_native | 1.018x | 1.011x | 1.009x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.12x).
