| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.956x | 1.424x |
| materialize | 0.977x | 0.915x | 1.181x |
| perft_bulk_d4 | 1.006x | 0.981x | 1.252x |
| perft_full_d3 | 0.989x | 0.970x | 1.254x |
| fen_parse | 0.913x | 0.869x | 1.165x |
| fen_write_native_ep | 0.957x | 0.796x | 1.507x |
| snapshot | 0.995x | 0.988x | 1.489x |
| san_parse_all | 0.992x | 0.986x | 1.725x |
| san_render_native | 0.991x | 0.941x | 2.233x |
| apply_children_full | 0.945x | 0.850x | 1.297x |
| make_unmake_full | 0.951x | 0.813x | 1.130x |
| startpos | 0.955x | 0.955x | 1.242x |
| chess960_materialize | 0.983x | 0.979x | 1.526x |
| chess960_perft_d3 | 0.987x | 0.987x | 1.538x |
| polyglot_key | 1.004x | 0.982x | 37.067x |
| polyglot_recompute | 0.952x | 0.921x | 0.959x |
| repetition_key | 0.970x | 0.960x | 27.420x |
| corpus_count | 0.981x | 0.971x | 1.125x |
| corpus_materialize | 0.996x | 0.985x | 1.125x |
| corpus_uci_sorted | 0.981x | 0.973x | 1.636x |
| replay_trusted_native | 1.000x | 0.991x | 0.738x |
| replay_checked_native | 0.985x | 0.971x | 1.166x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.07x).
