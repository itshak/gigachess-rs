| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.884x | 1.502x |
| materialize | 1.027x | 0.985x | 1.337x |
| perft_bulk_d4 | 1.042x | 1.019x | 1.436x |
| perft_full_d3 | 1.217x | 1.117x | 1.610x |
| fen_parse | 1.025x | 0.960x | 1.313x |
| fen_write_native_ep | 0.958x | 0.861x | 1.500x |
| snapshot | 1.034x | 0.987x | 0.992x |
| san_parse_all | 0.997x | 0.966x | 2.134x |
| san_render_native | 0.988x | 0.931x | 3.820x |
| apply_children_full | 0.978x | 0.915x | 1.411x |
| make_unmake_full | 0.969x | 0.919x | 1.370x |
| startpos | 1.002x | 1.002x | 1.248x |
| chess960_materialize | 1.020x | 0.998x | 1.405x |
| chess960_perft_d3 | 1.007x | 0.980x | 1.594x |
| polyglot_key | 0.950x | 0.949x | 26.657x |
| polyglot_recompute | 0.998x | 0.991x | 1.027x |
| repetition_key | 1.094x | 0.975x | 22.245x |
| corpus_count | 0.999x | 0.982x | 1.417x |
| corpus_materialize | 1.005x | 0.999x | 1.238x |
| corpus_uci_sorted | 0.962x | 0.936x | 1.378x |
| replay_trusted_native | 0.955x | 0.910x | 0.857x |
| replay_checked_native | 0.996x | 0.984x | 1.070x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.05x).
