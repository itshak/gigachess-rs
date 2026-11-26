| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.035x | 0.833x | 1.524x |
| materialize | 1.037x | 0.706x | 1.169x |
| perft_bulk_d4 | 0.924x | 0.657x | 1.110x |
| perft_full_d3 | 1.078x | 0.709x | 1.464x |
| fen_parse | 0.942x | 0.719x | 1.132x |
| fen_write_native_ep | 1.069x | 0.786x | 1.871x |
| snapshot | 1.041x | 0.953x | 1.059x |
| san_parse_all | 0.948x | 0.753x | 1.658x |
| san_render_native | 1.021x | 0.880x | 2.827x |
| apply_children_full | 0.974x | 0.743x | 1.061x |
| make_unmake_full | 0.994x | 0.784x | 1.340x |
| startpos | 1.177x | 1.177x | 1.242x |
| chess960_materialize | 1.158x | 1.020x | 1.597x |
| chess960_perft_d3 | 1.010x | 0.938x | 1.504x |
| polyglot_key | 1.018x | 0.972x | 45.091x |
| polyglot_recompute | 0.949x | 0.887x | 0.908x |
| repetition_key | 1.035x | 1.013x | 19.547x |
| corpus_count | 0.932x | 0.800x | 1.087x |
| corpus_materialize | 1.059x | 0.977x | 1.112x |
| corpus_uci_sorted | 1.048x | 0.884x | 1.365x |
| replay_trusted_native | 0.918x | 0.887x | 0.624x |
| replay_checked_native | 0.866x | 0.836x | 1.181x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.85x).
