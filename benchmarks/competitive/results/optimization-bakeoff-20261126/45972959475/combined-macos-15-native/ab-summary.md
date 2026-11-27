| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.976x | 0.767x | 1.533x |
| materialize | 0.964x | 0.752x | 1.122x |
| perft_bulk_d4 | 1.044x | 0.857x | 1.173x |
| perft_full_d3 | 0.990x | 0.828x | 1.395x |
| fen_parse | 1.039x | 0.771x | 1.224x |
| fen_write_native_ep | 1.087x | 0.930x | 1.729x |
| snapshot | 0.902x | 0.643x | 0.861x |
| san_parse_all | 0.994x | 0.888x | 1.692x |
| san_render_native | 0.977x | 0.766x | 3.139x |
| apply_children_full | 0.898x | 0.738x | 1.005x |
| make_unmake_full | 0.865x | 0.800x | 1.207x |
| startpos | 0.607x | 0.607x | 0.731x |
| chess960_materialize | 1.099x | 1.093x | 1.502x |
| chess960_perft_d3 | 0.945x | 0.937x | 1.578x |
| polyglot_key | 0.974x | 0.940x | 37.691x |
| polyglot_recompute | 0.882x | 0.846x | 0.997x |
| repetition_key | 1.091x | 0.927x | 19.358x |
| corpus_count | 1.288x | 1.144x | 1.544x |
| corpus_materialize | 1.031x | 1.026x | 1.044x |
| corpus_uci_sorted | 0.952x | 0.754x | 1.404x |
| replay_trusted_native | 0.879x | 0.871x | 0.702x |
| replay_checked_native | 0.866x | 0.819x | 1.182x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.04x).
