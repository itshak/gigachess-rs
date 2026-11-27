| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.083x | 0.818x | 1.648x |
| materialize | 1.193x | 0.969x | 1.052x |
| perft_bulk_d4 | 1.194x | 1.019x | 1.315x |
| perft_full_d3 | 1.087x | 0.896x | 1.317x |
| fen_parse | 1.117x | 0.996x | 1.188x |
| fen_write_native_ep | 1.003x | 0.902x | 1.724x |
| snapshot | 0.928x | 0.814x | 0.856x |
| san_parse_all | 1.029x | 0.845x | 1.679x |
| san_render_native | 1.063x | 0.802x | 2.753x |
| apply_children_full | 1.149x | 0.979x | 1.138x |
| make_unmake_full | 1.053x | 0.923x | 1.314x |
| startpos | 1.147x | 1.147x | 1.527x |
| chess960_materialize | 1.173x | 1.114x | 1.676x |
| chess960_perft_d3 | 1.091x | 0.986x | 1.730x |
| polyglot_key | 1.056x | 0.971x | 40.785x |
| polyglot_recompute | 1.077x | 0.904x | 1.057x |
| repetition_key | 1.115x | 0.993x | 20.596x |
| corpus_count | 0.910x | 0.878x | 1.016x |
| corpus_materialize | 0.982x | 0.906x | 1.137x |
| corpus_uci_sorted | 0.987x | 0.979x | 1.359x |
| replay_trusted_native | 1.056x | 0.885x | 0.625x |
| replay_checked_native | 1.091x | 1.056x | 1.272x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.18x).
