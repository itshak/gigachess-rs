| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.952x | 0.755x | 1.261x |
| materialize | 1.023x | 0.847x | 1.022x |
| perft_bulk_d4 | 0.823x | 0.686x | 1.067x |
| perft_full_d3 | 1.004x | 0.796x | 1.431x |
| fen_parse | 0.986x | 0.811x | 1.166x |
| fen_write_native_ep | 0.937x | 0.682x | 1.845x |
| snapshot | 1.037x | 0.739x | 1.069x |
| san_parse_all | 0.923x | 0.786x | 1.615x |
| san_render_native | 0.965x | 0.691x | 2.549x |
| apply_children_full | 0.895x | 0.707x | 1.072x |
| make_unmake_full | 0.939x | 0.811x | 1.248x |
| startpos | 0.986x | 0.986x | 1.284x |
| chess960_materialize | 0.858x | 0.777x | 1.177x |
| chess960_perft_d3 | 0.853x | 0.809x | 1.172x |
| polyglot_key | 1.015x | 0.954x | 39.301x |
| polyglot_recompute | 0.784x | 0.651x | 0.996x |
| repetition_key | 0.779x | 0.501x | 16.055x |
| corpus_count | 0.692x | 0.629x | 0.934x |
| corpus_materialize | 0.843x | 0.734x | 0.986x |
| corpus_uci_sorted | 0.821x | 0.737x | 1.558x |
| replay_trusted_native | 0.930x | 0.765x | 0.615x |
| replay_checked_native | 1.041x | 0.950x | 1.151x |

Cold first legal move list: 0.009 ms -> 0.006 ms (1.48x).
