| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.985x | 0.922x | 1.502x |
| materialize | 0.938x | 0.846x | 1.242x |
| perft_bulk_d4 | 0.973x | 0.932x | 1.369x |
| perft_full_d3 | 0.962x | 0.911x | 1.297x |
| fen_parse | 0.977x | 0.927x | 1.290x |
| fen_write_native_ep | 0.954x | 0.901x | 1.571x |
| snapshot | 0.950x | 0.846x | 0.947x |
| san_parse_all | 0.965x | 0.850x | 2.078x |
| san_render_native | 0.990x | 0.928x | 3.752x |
| apply_children_full | 0.933x | 0.854x | 1.295x |
| make_unmake_full | 0.943x | 0.865x | 1.266x |
| startpos | 1.001x | 1.001x | 1.250x |
| chess960_materialize | 1.018x | 0.992x | 1.358x |
| chess960_perft_d3 | 1.000x | 0.993x | 1.532x |
| polyglot_key | 0.940x | 0.924x | 28.397x |
| polyglot_recompute | 0.974x | 0.967x | 1.015x |
| repetition_key | 0.953x | 0.936x | 21.745x |
| corpus_count | 1.026x | 1.019x | 1.382x |
| corpus_materialize | 1.016x | 1.008x | 1.180x |
| corpus_uci_sorted | 0.951x | 0.909x | 1.330x |
| replay_trusted_native | 0.917x | 0.884x | 0.764x |
| replay_checked_native | 0.962x | 0.943x | 0.967x |

Cold first legal move list: 0.019 ms -> 0.021 ms (0.91x).
