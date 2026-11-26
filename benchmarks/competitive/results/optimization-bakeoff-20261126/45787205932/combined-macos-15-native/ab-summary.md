| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 0.729x | 1.419x |
| materialize | 0.922x | 0.691x | 0.977x |
| perft_bulk_d4 | 1.037x | 0.788x | 1.311x |
| perft_full_d3 | 0.977x | 0.762x | 1.394x |
| fen_parse | 1.038x | 0.795x | 1.203x |
| fen_write_native_ep | 1.018x | 0.874x | 1.788x |
| snapshot | 1.025x | 0.746x | 0.831x |
| san_parse_all | 0.980x | 0.881x | 1.910x |
| san_render_native | 0.933x | 0.644x | 2.707x |
| apply_children_full | 0.960x | 0.753x | 0.938x |
| make_unmake_full | 0.958x | 0.694x | 1.205x |
| startpos | 0.983x | 0.983x | 1.127x |
| chess960_materialize | 0.976x | 0.895x | 1.271x |
| chess960_perft_d3 | 0.873x | 0.843x | 1.522x |
| polyglot_key | 0.869x | 0.824x | 35.521x |
| polyglot_recompute | 1.083x | 0.883x | 0.973x |
| repetition_key | 0.959x | 0.904x | 18.974x |
| corpus_count | 1.000x | 0.923x | 1.292x |
| corpus_materialize | 1.061x | 1.053x | 1.342x |
| corpus_uci_sorted | 1.020x | 0.951x | 1.293x |
| replay_trusted_native | 0.933x | 0.883x | 0.536x |
| replay_checked_native | 0.949x | 0.937x | 1.011x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.94x).
