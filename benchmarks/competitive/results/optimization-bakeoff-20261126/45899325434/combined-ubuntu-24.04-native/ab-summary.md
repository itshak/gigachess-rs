| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.972x | 1.488x |
| materialize | 0.990x | 0.948x | 1.144x |
| perft_bulk_d4 | 0.996x | 0.940x | 1.333x |
| perft_full_d3 | 1.304x | 1.082x | 1.588x |
| fen_parse | 1.124x | 1.087x | 1.212x |
| fen_write_native_ep | 1.004x | 0.993x | 1.865x |
| snapshot | 0.777x | 0.775x | 0.973x |
| san_parse_all | 0.996x | 0.982x | 1.927x |
| san_render_native | 0.991x | 0.973x | 2.101x |
| apply_children_full | 1.005x | 0.991x | 1.323x |
| make_unmake_full | 1.007x | 1.002x | 1.224x |
| startpos | 0.999x | 0.999x | 1.199x |
| chess960_materialize | 1.000x | 0.993x | 1.491x |
| chess960_perft_d3 | 1.001x | 0.991x | 1.591x |
| polyglot_key | 1.004x | 0.998x | 20.363x |
| polyglot_recompute | 0.994x | 0.987x | 1.000x |
| repetition_key | 1.006x | 1.005x | 19.071x |
| corpus_count | 1.047x | 1.014x | 1.350x |
| corpus_materialize | 1.005x | 1.004x | 1.202x |
| corpus_uci_sorted | 0.996x | 0.992x | 1.782x |
| replay_trusted_native | 0.986x | 0.986x | 1.105x |
| replay_checked_native | 0.990x | 0.988x | 1.100x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.99x).
