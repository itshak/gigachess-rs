| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.965x | 0.876x | 1.327x |
| materialize | 1.045x | 0.996x | 1.485x |
| perft_bulk_d4 | 1.010x | 0.974x | 1.022x |
| perft_full_d3 | 0.980x | 0.946x | 1.446x |
| fen_parse | 1.070x | 1.004x | 1.143x |
| fen_write_native_ep | 1.005x | 0.969x | 1.932x |
| snapshot | 0.993x | 0.984x | 1.320x |
| san_parse_all | 0.870x | 0.854x | 1.655x |
| san_render_native | 0.988x | 0.978x | 2.090x |
| apply_children_full | 1.007x | 0.988x | 1.285x |
| make_unmake_full | 1.077x | 1.035x | 1.253x |
| startpos | 0.885x | 0.885x | 0.992x |
| chess960_materialize | 0.961x | 0.933x | 1.423x |
| chess960_perft_d3 | 0.988x | 0.978x | 1.392x |
| polyglot_key | 0.993x | 0.989x | 39.654x |
| polyglot_recompute | 0.988x | 0.986x | 1.019x |
| repetition_key | 0.865x | 0.764x | 16.129x |
| corpus_count | 0.989x | 0.985x | 0.869x |
| corpus_materialize | 1.007x | 1.004x | 1.036x |
| corpus_uci_sorted | 0.994x | 0.992x | 1.548x |
| replay_trusted_native | 1.025x | 0.969x | 0.918x |
| replay_checked_native | 1.122x | 1.054x | 1.139x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.11x).
