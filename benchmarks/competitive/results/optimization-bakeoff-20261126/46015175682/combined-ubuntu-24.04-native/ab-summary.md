| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.059x | 1.015x | 1.584x |
| materialize | 0.990x | 0.974x | 1.578x |
| perft_bulk_d4 | 1.056x | 1.039x | 1.233x |
| perft_full_d3 | 1.238x | 1.054x | 1.707x |
| fen_parse | 0.983x | 0.966x | 1.060x |
| fen_write_native_ep | 0.959x | 0.896x | 2.063x |
| snapshot | 0.992x | 0.964x | 1.144x |
| san_parse_all | 0.901x | 0.888x | 1.683x |
| san_render_native | 0.996x | 0.987x | 2.114x |
| apply_children_full | 1.024x | 1.009x | 1.367x |
| make_unmake_full | 0.988x | 0.942x | 1.175x |
| startpos | 1.000x | 1.000x | 1.125x |
| chess960_materialize | 0.987x | 0.976x | 1.522x |
| chess960_perft_d3 | 1.022x | 1.000x | 1.536x |
| polyglot_key | 0.995x | 0.991x | 38.681x |
| polyglot_recompute | 1.610x | 1.301x | 1.620x |
| repetition_key | 1.011x | 0.988x | 19.288x |
| corpus_count | 1.231x | 1.131x | 1.336x |
| corpus_materialize | 1.021x | 1.017x | 1.070x |
| corpus_uci_sorted | 1.047x | 1.037x | 1.588x |
| replay_trusted_native | 1.002x | 0.993x | 0.871x |
| replay_checked_native | 0.992x | 0.987x | 1.136x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.05x).
