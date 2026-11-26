| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.085x | 0.958x | 1.311x |
| materialize | 1.136x | 0.881x | 1.101x |
| perft_bulk_d4 | 1.079x | 0.909x | 1.128x |
| perft_full_d3 | 1.056x | 0.892x | 1.222x |
| fen_parse | 1.001x | 0.869x | 1.173x |
| fen_write_native_ep | 1.039x | 0.757x | 1.923x |
| snapshot | 1.021x | 0.604x | 0.860x |
| san_parse_all | 0.930x | 0.676x | 1.787x |
| san_render_native | 0.966x | 0.812x | 2.775x |
| apply_children_full | 0.854x | 0.749x | 1.004x |
| make_unmake_full | 0.868x | 0.810x | 1.161x |
| startpos | 1.341x | 1.341x | 1.116x |
| chess960_materialize | 0.893x | 0.886x | 1.440x |
| chess960_perft_d3 | 0.835x | 0.683x | 1.208x |
| polyglot_key | 0.843x | 0.767x | 35.019x |
| polyglot_recompute | 0.829x | 0.781x | 1.110x |
| repetition_key | 0.829x | 0.761x | 19.460x |
| corpus_count | 0.808x | 0.738x | 0.922x |
| corpus_materialize | 0.982x | 0.918x | 1.269x |
| corpus_uci_sorted | 0.738x | 0.689x | 1.170x |
| replay_trusted_native | 0.631x | 0.478x | 0.486x |
| replay_checked_native | 0.833x | 0.687x | 1.126x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.37x).
