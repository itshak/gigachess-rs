| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.835x | 0.671x | 1.129x |
| materialize | 0.969x | 0.814x | 1.048x |
| perft_bulk_d4 | 0.966x | 0.904x | 1.336x |
| perft_full_d3 | 1.024x | 0.887x | 1.375x |
| fen_parse | 0.901x | 0.698x | 1.077x |
| fen_write_native_ep | 1.295x | 1.021x | 1.630x |
| snapshot | 0.883x | 0.562x | 0.869x |
| san_parse_all | 0.850x | 0.742x | 1.912x |
| san_render_native | 0.851x | 0.652x | 2.740x |
| apply_children_full | 0.855x | 0.721x | 0.959x |
| make_unmake_full | 0.919x | 0.691x | 1.155x |
| startpos | 1.011x | 1.011x | 1.825x |
| chess960_materialize | 1.000x | 0.971x | 1.430x |
| chess960_perft_d3 | 0.851x | 0.815x | 1.479x |
| polyglot_key | 0.875x | 0.803x | 35.511x |
| polyglot_recompute | 0.991x | 0.873x | 0.959x |
| repetition_key | 1.052x | 0.914x | 21.897x |
| corpus_count | 0.843x | 0.810x | 1.236x |
| corpus_materialize | 0.860x | 0.853x | 1.053x |
| corpus_uci_sorted | 0.881x | 0.804x | 1.260x |
| replay_trusted_native | 0.946x | 0.831x | 0.543x |
| replay_checked_native | 0.928x | 0.814x | 1.085x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.03x).
