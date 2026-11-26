| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.974x | 0.925x | 1.454x |
| materialize | 0.974x | 0.938x | 1.239x |
| perft_bulk_d4 | 0.983x | 0.934x | 1.281x |
| perft_full_d3 | 0.878x | 0.823x | 1.109x |
| fen_parse | 0.846x | 0.803x | 1.078x |
| fen_write_native_ep | 0.991x | 0.977x | 1.463x |
| snapshot | 0.988x | 0.961x | 1.494x |
| san_parse_all | 0.932x | 0.909x | 1.664x |
| san_render_native | 0.934x | 0.859x | 2.018x |
| apply_children_full | 0.975x | 0.928x | 1.301x |
| make_unmake_full | 0.913x | 0.849x | 1.132x |
| startpos | 0.729x | 0.729x | 0.912x |
| chess960_materialize | 0.989x | 0.948x | 1.462x |
| chess960_perft_d3 | 0.924x | 0.885x | 1.380x |
| polyglot_key | 1.117x | 1.000x | 38.324x |
| polyglot_recompute | 0.982x | 0.961x | 0.955x |
| repetition_key | 1.208x | 1.159x | 28.024x |
| corpus_count | 1.031x | 1.013x | 1.171x |
| corpus_materialize | 1.052x | 1.033x | 1.176x |
| corpus_uci_sorted | 1.032x | 1.021x | 1.706x |
| replay_trusted_native | 1.037x | 1.028x | 0.716x |
| replay_checked_native | 0.921x | 0.903x | 1.016x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.89x).
