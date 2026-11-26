| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.974x | 0.916x | 1.371x |
| materialize | 0.962x | 0.894x | 1.526x |
| perft_bulk_d4 | 0.849x | 0.765x | 1.039x |
| perft_full_d3 | 0.959x | 0.837x | 1.454x |
| fen_parse | 0.983x | 0.938x | 1.119x |
| fen_write_native_ep | 1.004x | 0.997x | 1.940x |
| snapshot | 1.407x | 1.399x | 0.996x |
| san_parse_all | 0.877x | 0.865x | 1.688x |
| san_render_native | 1.009x | 1.006x | 2.150x |
| apply_children_full | 1.022x | 0.977x | 1.195x |
| make_unmake_full | 0.829x | 0.685x | 0.936x |
| startpos | 1.220x | 1.220x | 1.247x |
| chess960_materialize | 0.991x | 0.934x | 1.378x |
| chess960_perft_d3 | 1.025x | 0.977x | 1.406x |
| polyglot_key | 0.996x | 0.996x | 39.025x |
| polyglot_recompute | 1.034x | 0.993x | 1.055x |
| repetition_key | 0.946x | 0.728x | 18.945x |
| corpus_count | 0.665x | 0.564x | 0.892x |
| corpus_materialize | 0.942x | 0.928x | 1.045x |
| corpus_uci_sorted | 1.002x | 0.998x | 1.576x |
| replay_trusted_native | 0.941x | 0.895x | 0.777x |
| replay_checked_native | 0.997x | 0.964x | 1.064x |

Cold first legal move list: 0.009 ms -> 0.007 ms (1.35x).
