| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.967x | 0.870x | 1.457x |
| materialize | 0.916x | 0.757x | 1.171x |
| perft_bulk_d4 | 0.984x | 0.845x | 1.332x |
| perft_full_d3 | 0.952x | 0.907x | 1.333x |
| fen_parse | 0.974x | 0.727x | 1.280x |
| fen_write_native_ep | 0.911x | 0.815x | 1.561x |
| snapshot | 0.994x | 0.975x | 1.009x |
| san_parse_all | 0.943x | 0.862x | 2.086x |
| san_render_native | 1.005x | 0.963x | 3.942x |
| apply_children_full | 0.984x | 0.937x | 1.424x |
| make_unmake_full | 0.927x | 0.835x | 1.355x |
| startpos | 0.994x | 0.994x | 1.245x |
| chess960_materialize | 0.928x | 0.815x | 1.186x |
| chess960_perft_d3 | 0.944x | 0.879x | 1.494x |
| polyglot_key | 0.961x | 0.936x | 25.526x |
| polyglot_recompute | 0.984x | 0.983x | 0.997x |
| repetition_key | 0.973x | 0.927x | 19.743x |
| corpus_count | 1.066x | 1.039x | 1.429x |
| corpus_materialize | 0.990x | 0.957x | 1.133x |
| corpus_uci_sorted | 1.006x | 0.993x | 1.466x |
| replay_trusted_native | 0.996x | 0.979x | 0.847x |
| replay_checked_native | 0.998x | 0.991x | 1.094x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.05x).
