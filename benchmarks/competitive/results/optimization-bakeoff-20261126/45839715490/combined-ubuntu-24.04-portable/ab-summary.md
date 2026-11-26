| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.980x | 1.276x |
| materialize | 1.008x | 0.993x | 1.731x |
| perft_bulk_d4 | 1.003x | 0.995x | 1.186x |
| perft_full_d3 | 1.002x | 0.989x | 1.265x |
| fen_parse | 0.893x | 0.795x | 1.039x |
| fen_write_native_ep | 1.012x | 0.996x | 1.847x |
| snapshot | 1.003x | 0.991x | 1.007x |
| san_parse_all | 1.090x | 1.070x | 1.941x |
| san_render_native | 0.971x | 0.959x | 2.073x |
| apply_children_full | 1.007x | 1.001x | 1.327x |
| make_unmake_full | 1.001x | 0.990x | 1.234x |
| startpos | 0.987x | 0.987x | 1.349x |
| chess960_materialize | 1.008x | 0.999x | 1.572x |
| chess960_perft_d3 | 1.012x | 1.011x | 1.521x |
| polyglot_key | 1.052x | 1.001x | 40.995x |
| polyglot_recompute | 0.954x | 0.950x | 0.973x |
| repetition_key | 1.011x | 0.999x | 20.298x |
| corpus_count | 0.999x | 0.998x | 1.135x |
| corpus_materialize | 1.009x | 1.007x | 1.150x |
| corpus_uci_sorted | 0.990x | 0.986x | 1.534x |
| replay_trusted_native | 0.998x | 0.993x | 0.866x |
| replay_checked_native | 1.002x | 0.998x | 1.117x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.08x).
