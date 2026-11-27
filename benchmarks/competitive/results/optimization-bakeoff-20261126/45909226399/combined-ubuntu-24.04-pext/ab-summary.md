| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.985x | 0.943x | 1.452x |
| materialize | 0.996x | 0.967x | 1.269x |
| perft_bulk_d4 | 0.991x | 0.969x | 1.378x |
| perft_full_d3 | 0.990x | 0.983x | 1.282x |
| fen_parse | 0.954x | 0.933x | 1.224x |
| fen_write_native_ep | 1.004x | 0.983x | 1.779x |
| snapshot | 0.776x | 0.767x | 0.970x |
| san_parse_all | 1.015x | 0.998x | 2.080x |
| san_render_native | 0.985x | 0.960x | 2.175x |
| apply_children_full | 0.989x | 0.973x | 1.304x |
| make_unmake_full | 1.007x | 0.974x | 1.312x |
| startpos | 0.999x | 0.999x | 1.250x |
| chess960_materialize | 1.008x | 0.987x | 1.416x |
| chess960_perft_d3 | 0.992x | 0.974x | 1.474x |
| polyglot_key | 1.138x | 1.045x | 29.392x |
| polyglot_recompute | 0.967x | 0.964x | 1.001x |
| repetition_key | 1.001x | 0.977x | 20.379x |
| corpus_count | 1.002x | 1.002x | 1.429x |
| corpus_materialize | 0.997x | 0.996x | 1.190x |
| corpus_uci_sorted | 1.004x | 0.992x | 1.814x |
| replay_trusted_native | 0.973x | 0.944x | 0.896x |
| replay_checked_native | 0.949x | 0.936x | 1.061x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.99x).
