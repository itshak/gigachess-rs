| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.973x | 1.549x |
| materialize | 1.003x | 0.957x | 1.156x |
| perft_bulk_d4 | 1.017x | 0.939x | 1.426x |
| perft_full_d3 | 0.987x | 0.962x | 0.984x |
| fen_parse | 0.992x | 0.946x | 1.296x |
| fen_write_native_ep | 1.008x | 0.986x | 1.658x |
| snapshot | 0.978x | 0.863x | 1.096x |
| san_parse_all | 0.967x | 0.872x | 1.909x |
| san_render_native | 1.020x | 0.970x | 3.522x |
| apply_children_full | 1.007x | 0.943x | 1.322x |
| make_unmake_full | 1.002x | 0.982x | 1.277x |
| startpos | 0.981x | 0.981x | 0.982x |
| chess960_materialize | 0.964x | 0.948x | 1.348x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.743x |
| polyglot_key | 0.998x | 0.975x | 21.461x |
| polyglot_recompute | 1.043x | 0.967x | 1.054x |
| repetition_key | 0.976x | 0.940x | 19.468x |
| corpus_count | 0.991x | 0.979x | 1.364x |
| corpus_materialize | 1.017x | 1.012x | 1.201x |
| corpus_uci_sorted | 0.928x | 0.905x | 1.477x |
| replay_trusted_native | 0.847x | 0.733x | 0.944x |
| replay_checked_native | 0.933x | 0.894x | 0.933x |

Cold first legal move list: 0.012 ms -> 0.013 ms (0.95x).
