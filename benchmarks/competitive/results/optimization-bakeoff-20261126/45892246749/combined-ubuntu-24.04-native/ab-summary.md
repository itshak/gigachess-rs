| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.962x | 0.923x | 1.441x |
| materialize | 0.969x | 0.949x | 1.549x |
| perft_bulk_d4 | 0.846x | 0.796x | 0.990x |
| perft_full_d3 | 1.017x | 0.934x | 1.415x |
| fen_parse | 0.916x | 0.884x | 1.035x |
| fen_write_native_ep | 0.896x | 0.839x | 1.856x |
| snapshot | 0.864x | 0.862x | 1.001x |
| san_parse_all | 0.969x | 0.954x | 1.833x |
| san_render_native | 0.943x | 0.857x | 2.001x |
| apply_children_full | 0.884x | 0.817x | 1.285x |
| make_unmake_full | 0.776x | 0.699x | 0.928x |
| startpos | 0.913x | 0.913x | 1.022x |
| chess960_materialize | 0.918x | 0.895x | 1.472x |
| chess960_perft_d3 | 0.862x | 0.811x | 1.299x |
| polyglot_key | 0.997x | 0.996x | 39.978x |
| polyglot_recompute | 0.967x | 0.958x | 0.974x |
| repetition_key | 1.101x | 0.913x | 21.961x |
| corpus_count | 0.962x | 0.949x | 1.046x |
| corpus_materialize | 1.018x | 1.015x | 1.070x |
| corpus_uci_sorted | 1.017x | 1.017x | 1.590x |
| replay_trusted_native | 0.992x | 0.948x | 0.831x |
| replay_checked_native | 0.916x | 0.909x | 1.087x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.97x).
