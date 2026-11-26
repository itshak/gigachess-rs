| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.980x | 0.752x | 1.504x |
| materialize | 0.935x | 0.717x | 1.067x |
| perft_bulk_d4 | 0.910x | 0.758x | 1.218x |
| perft_full_d3 | 0.898x | 0.751x | 1.268x |
| fen_parse | 0.899x | 0.753x | 1.105x |
| fen_write_native_ep | 0.972x | 0.840x | 1.693x |
| snapshot | 0.910x | 0.723x | 0.899x |
| san_parse_all | 0.981x | 0.856x | 1.966x |
| san_render_native | 0.897x | 0.795x | 2.640x |
| apply_children_full | 0.957x | 0.818x | 1.145x |
| make_unmake_full | 0.921x | 0.712x | 1.320x |
| startpos | 1.200x | 1.200x | 1.241x |
| chess960_materialize | 0.967x | 0.916x | 1.929x |
| chess960_perft_d3 | 0.833x | 0.781x | 1.500x |
| polyglot_key | 0.993x | 0.888x | 45.401x |
| polyglot_recompute | 0.967x | 0.921x | 1.007x |
| repetition_key | 0.893x | 0.836x | 20.531x |
| corpus_count | 0.862x | 0.828x | 1.218x |
| corpus_materialize | 0.805x | 0.664x | 0.988x |
| corpus_uci_sorted | 0.931x | 0.902x | 1.420x |
| replay_trusted_native | 1.087x | 1.001x | 0.730x |
| replay_checked_native | 1.000x | 0.763x | 1.088x |

Cold first legal move list: 0.004 ms -> 0.004 ms (1.10x).
