| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.913x | 0.821x | 1.230x |
| materialize | 0.928x | 0.828x | 1.031x |
| perft_bulk_d4 | 0.942x | 0.798x | 1.263x |
| perft_full_d3 | 0.930x | 0.856x | 0.770x |
| fen_parse | 0.862x | 0.788x | 1.157x |
| fen_write_native_ep | 0.925x | 0.792x | 1.283x |
| snapshot | 1.008x | 0.962x | 0.985x |
| san_parse_all | 0.939x | 0.912x | 1.592x |
| san_render_native | 0.912x | 0.750x | 2.542x |
| apply_children_full | 0.935x | 0.871x | 0.775x |
| make_unmake_full | 0.901x | 0.801x | 0.977x |
| startpos | 0.617x | 0.617x | 0.845x |
| chess960_materialize | 0.889x | 0.849x | 1.231x |
| chess960_perft_d3 | 0.843x | 0.832x | 1.375x |
| polyglot_key | 0.872x | 0.870x | 37.298x |
| polyglot_recompute | 0.934x | 0.900x | 1.028x |
| repetition_key | 0.935x | 0.902x | 19.312x |
| corpus_count | 0.875x | 0.859x | 1.227x |
| corpus_materialize | 0.979x | 0.969x | 1.223x |
| corpus_uci_sorted | 0.913x | 0.906x | 1.002x |
| replay_trusted_native | 1.017x | 0.936x | 0.484x |
| replay_checked_native | 0.887x | 0.866x | 0.907x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.87x).
