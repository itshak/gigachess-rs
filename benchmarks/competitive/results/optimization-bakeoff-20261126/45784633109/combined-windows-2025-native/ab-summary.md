| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.869x | 1.337x |
| materialize | 1.032x | 0.979x | 1.139x |
| perft_bulk_d4 | 0.981x | 0.881x | 1.317x |
| perft_full_d3 | 0.976x | 0.917x | 1.077x |
| fen_parse | 1.191x | 1.143x | 1.426x |
| fen_write_native_ep | 0.972x | 0.877x | 1.579x |
| snapshot | 0.660x | 0.650x | 1.001x |
| san_parse_all | 0.980x | 0.950x | 1.857x |
| san_render_native | 1.014x | 0.944x | 3.572x |
| apply_children_full | 0.966x | 0.922x | 0.987x |
| make_unmake_full | 1.001x | 0.933x | 1.229x |
| startpos | 1.003x | 1.003x | 1.172x |
| chess960_materialize | 1.023x | 0.962x | 1.252x |
| chess960_perft_d3 | 0.976x | 0.923x | 1.409x |
| polyglot_key | 0.997x | 0.995x | 20.207x |
| polyglot_recompute | 0.942x | 0.918x | 0.970x |
| repetition_key | 0.874x | 0.769x | 14.847x |
| corpus_count | 0.878x | 0.776x | 1.182x |
| corpus_materialize | 0.912x | 0.869x | 1.083x |
| corpus_uci_sorted | 0.948x | 0.932x | 1.511x |
| replay_trusted_native | 0.976x | 0.913x | 0.726x |
| replay_checked_native | 0.923x | 0.878x | 0.910x |

Cold first legal move list: 0.021 ms -> 0.014 ms (1.52x).
