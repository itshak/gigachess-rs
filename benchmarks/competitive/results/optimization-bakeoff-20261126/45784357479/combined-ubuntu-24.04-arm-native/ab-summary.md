| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.004x | 1.001x | 1.355x |
| materialize | 1.001x | 0.999x | 1.239x |
| perft_bulk_d4 | 1.001x | 1.000x | 1.351x |
| perft_full_d3 | 1.001x | 0.991x | 1.080x |
| fen_parse | 0.996x | 0.991x | 1.231x |
| fen_write_native_ep | 0.995x | 0.988x | 1.571x |
| snapshot | 0.999x | 0.993x | 0.864x |
| san_parse_all | 0.998x | 0.991x | 1.826x |
| san_render_native | 1.002x | 0.996x | 2.571x |
| apply_children_full | 0.998x | 0.988x | 1.043x |
| make_unmake_full | 1.006x | 1.001x | 1.446x |
| startpos | 1.000x | 1.000x | 1.181x |
| chess960_materialize | 1.006x | 1.005x | 1.331x |
| chess960_perft_d3 | 1.003x | 1.000x | 1.327x |
| polyglot_key | 0.993x | 0.981x | 25.592x |
| polyglot_recompute | 0.917x | 0.910x | 0.985x |
| repetition_key | 0.985x | 0.969x | 18.897x |
| corpus_count | 0.973x | 0.948x | 1.445x |
| corpus_materialize | 0.979x | 0.977x | 1.172x |
| corpus_uci_sorted | 0.987x | 0.986x | 1.443x |
| replay_trusted_native | 1.000x | 0.999x | 0.502x |
| replay_checked_native | 0.988x | 0.987x | 0.983x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.06x).
