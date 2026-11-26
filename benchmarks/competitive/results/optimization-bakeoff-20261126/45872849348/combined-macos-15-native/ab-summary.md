| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.820x | 0.676x | 1.473x |
| materialize | 0.784x | 0.567x | 0.993x |
| perft_bulk_d4 | 0.888x | 0.665x | 1.166x |
| perft_full_d3 | 0.815x | 0.592x | 1.257x |
| fen_parse | 0.920x | 0.770x | 1.232x |
| fen_write_native_ep | 0.834x | 0.587x | 1.600x |
| snapshot | 0.842x | 0.700x | 0.804x |
| san_parse_all | 0.868x | 0.734x | 1.750x |
| san_render_native | 0.825x | 0.599x | 2.979x |
| apply_children_full | 0.678x | 0.548x | 0.898x |
| make_unmake_full | 0.766x | 0.545x | 1.279x |
| startpos | 0.677x | 0.677x | 1.081x |
| chess960_materialize | 0.888x | 0.750x | 1.381x |
| chess960_perft_d3 | 0.861x | 0.805x | 1.719x |
| polyglot_key | 0.899x | 0.861x | 35.142x |
| polyglot_recompute | 0.934x | 0.909x | 1.087x |
| repetition_key | 0.926x | 0.804x | 20.717x |
| corpus_count | 1.049x | 0.900x | 1.049x |
| corpus_materialize | 1.016x | 0.916x | 1.056x |
| corpus_uci_sorted | 1.181x | 1.046x | 1.375x |
| replay_trusted_native | 1.109x | 0.947x | 0.795x |
| replay_checked_native | 1.061x | 1.018x | 1.531x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.83x).
