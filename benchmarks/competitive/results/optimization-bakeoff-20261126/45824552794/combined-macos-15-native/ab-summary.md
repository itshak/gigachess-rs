| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.945x | 0.689x | 1.370x |
| materialize | 0.920x | 0.698x | 0.965x |
| perft_bulk_d4 | 0.907x | 0.807x | 1.098x |
| perft_full_d3 | 1.015x | 0.891x | 1.321x |
| fen_parse | 1.003x | 0.831x | 1.245x |
| fen_write_native_ep | 0.942x | 0.664x | 1.803x |
| snapshot | 1.046x | 0.661x | 0.864x |
| san_parse_all | 0.950x | 0.813x | 1.727x |
| san_render_native | 0.970x | 0.881x | 2.505x |
| apply_children_full | 0.902x | 0.668x | 1.060x |
| make_unmake_full | 0.946x | 0.790x | 1.360x |
| startpos | 0.856x | 0.856x | 1.129x |
| chess960_materialize | 0.849x | 0.848x | 1.258x |
| chess960_perft_d3 | 1.009x | 0.940x | 1.356x |
| polyglot_key | 1.160x | 1.159x | 44.133x |
| polyglot_recompute | 1.177x | 1.115x | 1.097x |
| repetition_key | 0.876x | 0.865x | 16.904x |
| corpus_count | 1.176x | 1.170x | 1.207x |
| corpus_materialize | 1.185x | 1.113x | 1.100x |
| corpus_uci_sorted | 1.042x | 0.899x | 1.449x |
| replay_trusted_native | 0.853x | 0.817x | 0.575x |
| replay_checked_native | 1.154x | 1.153x | 1.198x |

Cold first legal move list: 0.007 ms -> 0.011 ms (0.64x).
