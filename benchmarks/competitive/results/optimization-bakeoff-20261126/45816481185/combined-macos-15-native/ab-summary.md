| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.072x | 0.950x | 1.364x |
| materialize | 0.980x | 0.916x | 1.018x |
| perft_bulk_d4 | 0.980x | 0.837x | 1.087x |
| perft_full_d3 | 0.951x | 0.877x | 1.329x |
| fen_parse | 0.898x | 0.863x | 1.171x |
| fen_write_native_ep | 0.933x | 0.869x | 1.747x |
| snapshot | 0.971x | 0.741x | 0.974x |
| san_parse_all | 1.034x | 0.951x | 1.731x |
| san_render_native | 1.054x | 0.966x | 2.549x |
| apply_children_full | 1.016x | 0.916x | 1.019x |
| make_unmake_full | 0.999x | 0.905x | 1.212x |
| startpos | 1.046x | 1.046x | 1.287x |
| chess960_materialize | 0.950x | 0.932x | 1.445x |
| chess960_perft_d3 | 0.951x | 0.943x | 1.222x |
| polyglot_key | 0.972x | 0.953x | 40.336x |
| polyglot_recompute | 0.915x | 0.907x | 1.095x |
| repetition_key | 0.843x | 0.783x | 17.715x |
| corpus_count | 0.906x | 0.861x | 0.912x |
| corpus_materialize | 1.034x | 0.928x | 1.111x |
| corpus_uci_sorted | 1.035x | 1.021x | 1.406x |
| replay_trusted_native | 0.941x | 0.920x | 0.572x |
| replay_checked_native | 0.936x | 0.852x | 1.049x |

Cold first legal move list: 0.006 ms -> 0.003 ms (1.86x).
