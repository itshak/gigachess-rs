| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.969x | 0.887x | 1.431x |
| materialize | 0.974x | 0.922x | 1.087x |
| perft_bulk_d4 | 0.790x | 0.731x | 1.111x |
| perft_full_d3 | 0.992x | 0.907x | 1.292x |
| fen_parse | 0.871x | 0.821x | 1.173x |
| fen_write_native_ep | 0.935x | 0.888x | 1.492x |
| snapshot | 0.994x | 0.963x | 1.051x |
| san_parse_all | 0.864x | 0.820x | 1.673x |
| san_render_native | 0.955x | 0.895x | 3.118x |
| apply_children_full | 0.863x | 0.812x | 1.134x |
| make_unmake_full | 0.707x | 0.585x | 0.928x |
| startpos | 0.861x | 0.861x | 0.856x |
| chess960_materialize | 0.955x | 0.936x | 1.368x |
| chess960_perft_d3 | 0.802x | 0.727x | 1.295x |
| polyglot_key | 1.129x | 0.999x | 23.131x |
| polyglot_recompute | 1.013x | 0.996x | 0.992x |
| repetition_key | 0.958x | 0.889x | 18.496x |
| corpus_count | 0.949x | 0.932x | 1.281x |
| corpus_materialize | 0.917x | 0.866x | 1.130x |
| corpus_uci_sorted | 0.987x | 0.985x | 1.486x |
| replay_trusted_native | 0.728x | 0.664x | 0.818x |
| replay_checked_native | 0.825x | 0.787x | 0.894x |

Cold first legal move list: 0.013 ms -> 0.015 ms (0.86x).
