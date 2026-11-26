| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.817x | 0.589x | 1.176x |
| materialize | 0.881x | 0.726x | 1.113x |
| perft_bulk_d4 | 0.953x | 0.761x | 1.207x |
| perft_full_d3 | 0.927x | 0.825x | 1.340x |
| fen_parse | 0.870x | 0.665x | 1.101x |
| fen_write_native_ep | 0.928x | 0.814x | 1.762x |
| snapshot | 0.977x | 0.805x | 0.993x |
| san_parse_all | 0.885x | 0.723x | 1.614x |
| san_render_native | 0.918x | 0.769x | 2.718x |
| apply_children_full | 0.921x | 0.740x | 0.973x |
| make_unmake_full | 0.939x | 0.830x | 1.159x |
| startpos | 0.909x | 0.909x | 1.063x |
| chess960_materialize | 0.928x | 0.907x | 1.380x |
| chess960_perft_d3 | 1.091x | 0.988x | 1.094x |
| polyglot_key | 1.195x | 1.029x | 39.300x |
| polyglot_recompute | 0.921x | 0.881x | 0.966x |
| repetition_key | 1.089x | 1.017x | 19.749x |
| corpus_count | 1.095x | 0.922x | 1.115x |
| corpus_materialize | 1.016x | 0.901x | 0.970x |
| corpus_uci_sorted | 1.287x | 1.183x | 1.550x |
| replay_trusted_native | 1.049x | 0.954x | 0.529x |
| replay_checked_native | 0.983x | 0.843x | 1.036x |

Cold first legal move list: 0.006 ms -> 0.004 ms (1.26x).
