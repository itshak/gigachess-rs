| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.953x | 0.862x | 1.240x |
| materialize | 0.992x | 0.949x | 1.207x |
| perft_bulk_d4 | 0.960x | 0.899x | 1.292x |
| perft_full_d3 | 0.966x | 0.916x | 1.223x |
| fen_parse | 1.061x | 0.968x | 1.303x |
| fen_write_native_ep | 1.002x | 0.919x | 1.004x |
| snapshot | 0.591x | 0.569x | 0.851x |
| san_parse_all | 0.967x | 0.906x | 1.928x |
| san_render_native | 0.931x | 0.830x | 3.646x |
| apply_children_full | 1.007x | 0.917x | 1.175x |
| make_unmake_full | 0.992x | 0.932x | 1.311x |
| startpos | 0.998x | 0.998x | 1.138x |
| chess960_materialize | 0.919x | 0.908x | 1.198x |
| chess960_perft_d3 | 0.924x | 0.876x | 1.266x |
| polyglot_key | 0.981x | 0.971x | 26.227x |
| polyglot_recompute | 0.958x | 0.951x | 0.996x |
| repetition_key | 0.996x | 0.943x | 20.981x |
| corpus_count | 0.832x | 0.740x | 1.177x |
| corpus_materialize | 0.954x | 0.940x | 1.148x |
| corpus_uci_sorted | 0.988x | 0.972x | 1.408x |
| replay_trusted_native | 0.974x | 0.903x | 0.632x |
| replay_checked_native | 0.951x | 0.932x | 0.949x |

Cold first legal move list: 0.027 ms -> 0.026 ms (1.06x).
