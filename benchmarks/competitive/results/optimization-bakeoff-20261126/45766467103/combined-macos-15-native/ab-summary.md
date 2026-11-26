| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.068x | 1.028x | 1.197x |
| materialize | 1.028x | 1.000x | 0.984x |
| perft_bulk_d4 | 1.210x | 1.132x | 1.146x |
| perft_full_d3 | 1.254x | 1.200x | 0.703x |
| fen_parse | 1.035x | 1.007x | 1.202x |
| fen_write_native_ep | 0.998x | 0.922x | 1.301x |
| snapshot | 0.991x | 0.933x | 0.725x |
| san_parse_all | 1.353x | 1.255x | 1.467x |
| san_render_native | 1.527x | 1.343x | 2.317x |
| apply_children_full | 1.083x | 1.036x | 0.706x |
| make_unmake_full | 1.041x | 0.959x | 0.931x |
| startpos | 0.885x | 0.885x | 0.842x |
| chess960_materialize | 1.053x | 1.042x | 1.260x |
| chess960_perft_d3 | 1.202x | 1.151x | 1.276x |
| polyglot_key | 0.955x | 0.928x | 35.122x |
| polyglot_recompute | 0.938x | 0.925x | 1.056x |
| repetition_key | 0.999x | 0.978x | 17.300x |
| corpus_count | 1.056x | 1.026x | 1.208x |
| corpus_materialize | 1.006x | 1.000x | 1.111x |
| corpus_uci_sorted | 1.208x | 1.183x | 0.777x |
| replay_trusted_native | 0.979x | 0.923x | 0.388x |
| replay_checked_native | 1.038x | 1.028x | 0.843x |

Cold first legal move list: 119.930 ms → 0.005 ms (23594.42x).
