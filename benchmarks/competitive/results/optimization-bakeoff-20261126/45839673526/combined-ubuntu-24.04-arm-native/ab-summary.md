| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.976x | 0.969x | 1.511x |
| materialize | 0.994x | 0.988x | 1.311x |
| perft_bulk_d4 | 0.961x | 0.949x | 1.340x |
| perft_full_d3 | 1.031x | 0.993x | 0.860x |
| fen_parse | 1.020x | 1.004x | 1.224x |
| fen_write_native_ep | 0.985x | 0.974x | 1.549x |
| snapshot | 1.075x | 1.070x | 0.961x |
| san_parse_all | 0.920x | 0.869x | 1.671x |
| san_render_native | 0.898x | 0.841x | 2.279x |
| apply_children_full | 0.959x | 0.948x | 1.101x |
| make_unmake_full | 0.909x | 0.820x | 1.410x |
| startpos | 0.869x | 0.869x | 1.093x |
| chess960_materialize | 0.987x | 0.986x | 1.474x |
| chess960_perft_d3 | 0.944x | 0.927x | 1.424x |
| polyglot_key | 1.004x | 0.981x | 25.959x |
| polyglot_recompute | 0.990x | 0.988x | 1.079x |
| repetition_key | 1.014x | 0.940x | 19.629x |
| corpus_count | 0.964x | 0.961x | 1.292x |
| corpus_materialize | 0.982x | 0.974x | 1.195x |
| corpus_uci_sorted | 1.003x | 1.003x | 1.466x |
| replay_trusted_native | 0.963x | 0.945x | 0.603x |
| replay_checked_native | 0.961x | 0.959x | 1.011x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.11x).
